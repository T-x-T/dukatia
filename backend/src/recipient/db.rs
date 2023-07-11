use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::{Recipient, DeepRecipient};
use crate::traits::*;

#[derive(Debug)]
pub struct RecipientDbReader<'a> {
	query_parameters: QueryParameters,
	pool: &'a Pool
}

impl<'a> DbReader<'a, Recipient> for RecipientDbReader<'a> {
	fn new(pool: &'a Pool) -> Self {
		return Self {
			query_parameters: QueryParameters::default(),
			pool,
		}
	}

	fn get_pool(&self) -> &Pool {
		return self.pool;
	}

	fn get_query_parameters(&self) -> &QueryParameters {
		return &self.query_parameters;
	}

	fn set_query_parameters(mut self, query_parameters: QueryParameters) -> Self {
		self.query_parameters = query_parameters;
		return self;
	}

	async fn execute(self) -> Result<Vec<Recipient>, Box<dyn Error>> {
		let query = "SELECT * FROM public.recipient_data";
		return Ok(
			self.actually_execute(query)
			.await?
			.into_iter()
			.map(Into::into)
			.collect()
		);
	}
}

#[derive(Debug)]
pub struct DeepRecipientDbReader<'a> {
	query_parameters: QueryParameters,
	pool: &'a Pool
}

impl<'a> DbReader<'a, DeepRecipient> for DeepRecipientDbReader<'a> {
	fn new(pool: &'a Pool) -> Self {
		return Self {
			query_parameters: QueryParameters::default(),
			pool,
		}
	}

	fn get_pool(&self) -> &Pool {
		return self.pool;
	}

	fn get_query_parameters(&self) -> &QueryParameters {
		return &self.query_parameters;
	}

	fn set_query_parameters(mut self, query_parameters: QueryParameters) -> Self {
		self.query_parameters = query_parameters;
		return self;
	}

	async fn execute(self) -> Result<Vec<DeepRecipient>, Box<dyn Error>> {
		let query = "SELECT * FROM public.deep_recipients";
		return Ok(
			self.actually_execute(query)
			.await?
			.into_iter()
			.map(Into::into)
			.collect()
		);
	}
}

#[derive(Debug)]
pub struct RecipientDbWriter<'a> {
	pool: &'a Pool,
	recipient: Recipient,
}

impl<'a> DbWriter<'a, Recipient> for RecipientDbWriter<'a> {
	fn new(pool: &'a Pool, item: Recipient) -> Self {
		Self {
			pool,
			recipient: item,
		}
	}

	async fn insert(self) -> Result<(), Box<dyn Error>> {
		let client = self.pool.get().await.unwrap();
		let id: i32 = client
			.query(
				"INSERT INTO public.recipients (id, name, user_id) VALUES (DEFAULT, $1, $2) RETURNING id;",
				&[&self.recipient.name, &(self.recipient.user_id.unwrap_or(0) as i32)]
			)
			.await?
			[0].get(0);
	
		if self.recipient.tag_ids.is_some() {
			for tag_id in self.recipient.clone().tag_ids.unwrap() {
				client.query(
					"INSERT INTO public.recipient_tags (recipient_id, tag_id) VALUES ($1, $2);",
					&[&id, &(tag_id as i32)]
				).await?;
			}
		}
		return Ok(());
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		if self.recipient.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty{property: String::from("id"), item_type: String::from("recipient")}));
		}
	
		super::RecipientLoader::new(self.pool).get().await?;
	
		let client = self.pool.get().await?;
		
		client.query(
				"UPDATE public.recipients SET name=$1 WHERE id=$2;",
				&[&self.recipient.name, &(self.recipient.id.unwrap() as i32)]
			)
			.await?;
		
		client.query(
				"DELETE FROM public.recipient_tags WHERE recipient_id=$1",
				&[&(self.recipient.id.unwrap() as i32)]
			).await?;
	
		if self.recipient.tag_ids.is_some() {
			for tag_id in self.recipient.tag_ids.clone().unwrap() {
				client.query(
					"INSERT INTO public.recipient_tags (recipient_id, tag_id) VALUES ($1, $2);",
					&[&(self.recipient.id.unwrap() as i32), &(tag_id as i32)]
				).await?;
			}
		}
	
		return Ok(());
	}
}

impl From<tokio_postgres::Row> for Recipient {
	fn from(value: tokio_postgres::Row) -> Self {
		let id: i32 = value.get(0);
		let name: String = value.get(1);
		let user_id: Option<i32> = value.get(2);
		let tag_ids = value
			.try_get(3)
			.unwrap_or(Vec::new())
			.into_iter()
			.map(|x: i32| x as u32)
			.collect();
		
		return Recipient {
			id: Some(id as u32),
			name,
			user_id: user_id.map(|x| x as u32),
			tag_ids: Some(tag_ids),
		};
	}
}

impl From<tokio_postgres::Row> for DeepRecipient {
	fn from(value: tokio_postgres::Row) -> Self {
		let id: i32 = value.get(0);
		let name: String = value.get(1);
		let user_id: Option<i32> = value.get(2);
		let user_name: Option<String> = value.get(3);
		let user_superuser: Option<bool> = value.get(4);
		let tag_ids: Vec<Option<i32>> = value.get(5);
		let tag_names: Vec<Option<String>> = value.get(6);
		let tag_parent_ids: Vec<Option<i32>> = value.get(7);
		let tag_parent_names: Vec<Option<String>> = value.get(8);
		let tag_parent_parent_ids: Vec<Option<i32>> = value.get(9);
		let tag_parent_user_ids: Vec<Option<i32>> = value.get(10);
		let tag_user_ids: Vec<Option<i32>> = value.get(11);
		let tag_user_names: Vec<Option<String>> = value.get(12);
		let tag_user_superusers: Vec<Option<bool>> = value.get(13);
	
		let user = user_id.map(|_| crate::user::User {
			id: Some(user_id.unwrap() as u32),
			name: user_name.unwrap(),
			secret: None,
			superuser: user_superuser.unwrap()
		});
	
		let tags: Vec<crate::tag::DeepTag> = tag_ids
			.into_iter()
			.filter(Option::is_some)
			.enumerate()
			.map(|(i, tag_id)| {
				let parent: Option<crate::tag::Tag> = match tag_parent_ids.get(i) {
					Some(x) => {
						x.as_ref().map(|_| crate::tag::Tag {
							id: tag_parent_ids[i].map(|x| x as u32),
							name: tag_parent_names[i].clone().unwrap(),
							user_id: tag_parent_user_ids[i].unwrap() as u32,
							parent_id: tag_parent_parent_ids[i].map(|x| x as u32),
						})
					},
					None => None,
				};
				
				crate::tag::DeepTag {
					id: tag_id.unwrap() as u32,
					name: tag_names[i].clone().unwrap(),
					user: crate::user::User {
						id: tag_user_ids[i].map(|x| x as u32),
						name: tag_user_names[i].clone().unwrap(),
						secret: None,
						superuser: tag_user_superusers[i].unwrap(),
					},
					parent,
				}
			}).collect();
	
		return DeepRecipient {
			id: id as u32,
			name,
			user,
			tags,
		};
	}
}