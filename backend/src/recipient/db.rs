use deadpool_postgres::Pool;
use std::error::Error;
use uuid::Uuid;
use super::super::CustomError;
use super::Recipient;
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

	async fn insert(self) -> Result<Uuid, Box<dyn Error>> {
		let client = self.pool.get().await.unwrap();
		client
			.query(
				"INSERT INTO public.recipients (id, name, user_id) VALUES ($1, $2, $3);",
				&[&self.recipient.id, &self.recipient.name, &self.recipient.user_id]
			)
			.await?;
	
		for tag_id in self.recipient.clone().tag_ids {
			client.query(
				"INSERT INTO public.recipient_tags (recipient_id, tag_id) VALUES ($1, $2);",
				&[&self.recipient.id, &tag_id]
			).await?;
		}
		
		return Ok(self.recipient.id);
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		let old = super::RecipientLoader::new(self.pool)
			.set_filter_id_uuid(self.recipient.id, NumberFilterModes::Exact)
			.get_first().await?;

		match old.user_id {
			Some(old_user_id) => {
				if old_user_id != self.recipient.user_id.unwrap() {
					return Err(Box::new(CustomError::UserIsntOwner));
				}
			},
			None => {
				let old_user = crate::user::UserLoader::new(self.pool)
					.set_filter_id_uuid(self.recipient.id, NumberFilterModes::Exact)
					.get_first().await?;

				if !old_user.superuser {
					return Err(Box::new(CustomError::UserIsntOwner));
				}
			},
		}

		let client = self.pool.get().await?;
		
		client.query(
				"UPDATE public.recipients SET name=$1 WHERE id=$2;",
				&[&self.recipient.name, &self.recipient.id]
			)
			.await?;
		
		client.query(
				"DELETE FROM public.recipient_tags WHERE recipient_id=$1",
				&[&self.recipient.id]
			).await?;
	
		for tag_id in self.recipient.tag_ids.clone() {
			client.query(
				"INSERT INTO public.recipient_tags (recipient_id, tag_id) VALUES ($1, $2);",
				&[&self.recipient.id, &tag_id]
			).await?;
		}
	
		return Ok(());
	}
}

#[allow(clippy::unwrap_or_default)]
impl From<tokio_postgres::Row> for Recipient {
	fn from(value: tokio_postgres::Row) -> Self {
		let id: Uuid = value.get(0);
		let name: String = value.get(1);
		let user_id: Option<Uuid> = value.get(2);
		let tag_ids: Vec<Uuid> = value.try_get(3).unwrap_or_default();
		
		return Recipient {
			id,
			name,
			user_id,
			tag_ids,
		};
	}
}