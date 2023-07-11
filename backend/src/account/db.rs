use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::{Account, DeepAccount};
use crate::traits::*;

#[derive(Debug)]
pub struct AccountDbReader<'a> {
	query_parameters: QueryParameters,
	pool: &'a Pool,
}

impl<'a> DbReader<'a, Account> for AccountDbReader<'a> {
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

	async fn execute(self) -> Result<Vec<Account>, Box<dyn Error>> {
		let query = "SELECT * FROM public.account_data";
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
pub struct DeepAccountDbReader<'a> {
	query_parameters: QueryParameters,
	pool: &'a Pool,
}

impl<'a> DbReader<'a, DeepAccount> for DeepAccountDbReader<'a> {
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

	async fn execute(self) -> Result<Vec<DeepAccount>, Box<dyn Error>> {
		let query = "SELECT * FROM public.deep_accounts";
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
pub struct AccountDbWriter<'a> {
	pool: &'a Pool,
	account: Account,
}

impl<'a> DbWriter<'a, Account> for AccountDbWriter<'a> {
	fn new(pool: &'a Pool, item: Account) -> Self {
		Self {
			pool,
			account: item,
		}
	}

	async fn insert(self) -> Result<(), Box<dyn Error>> {
		let client = self.pool.get().await?;
		let id: i32 = client
			.query(
				"INSERT INTO public.accounts (id, name, default_currency_id, user_id) VALUES (DEFAULT, $1, $2, $3) RETURNING id;",
				&[&self.account.name, &(self.account.default_currency_id as i32), &(self.account.user_id as i32)]
			)
			.await?
			 [0].get(0);
			 
		if self.account.tag_ids.is_some() {
			for tag_id in self.account.clone().tag_ids.unwrap() {
				client.query("INSERT INTO public.account_tags (account_id, tag_id) VALUES ($1, $2);", &[&id, &(tag_id as i32)]).await?;
			}
		}
		return Ok(());
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		if self.account.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("account") }));
		}
	
		super::AccountLoader::new(self.pool).set_filter_id(self.account.id.unwrap()).get_first().await?;
	
		let client = self.pool.get().await?;
	
		client.query(
			"UPDATE public.accounts SET name=$1, default_currency_id=$2 WHERE id=$3;",
			&[&self.account.name, &(self.account.default_currency_id as i32), &(self.account.id.unwrap() as i32)]
		)
		.await?;
	
		client.query(
			"DELETE FROM public.account_tags WHERE account_id=$1;",
			&[&(self.account.id.unwrap() as i32)]
		)
		.await?;
	
		if self.account.tag_ids.is_some() {
			for tag_id in self.account.tag_ids.clone().unwrap() {
				client.query(
					"INSERT INTO public.account_tags (account_id, tag_id) VALUES ($1, $2);",
					&[&(self.account.id.unwrap() as i32), &(tag_id as i32)]
				)
				.await?;
			}
		}
	
		return Ok(());
	}
}

impl From<tokio_postgres::Row> for Account {
	fn from(value: tokio_postgres::Row) -> Self {
		let id: i32 = value.get(0);
		let name: String = value.get(1);
		let default_currency_id: i32 = value.get(2);
		let user_id: i32 = value.try_get(3).unwrap_or(0);
		let tag_ids: Vec<u32> = value
			.try_get(4)
			.unwrap_or(Vec::new())
			.into_iter()
			.map(|x: i32| x as u32)
			.collect();
	
		return Account {
			id: Some(id as u32),
			name,
			default_currency_id: default_currency_id as u32,
			user_id: user_id as u32,
			tag_ids: Some(tag_ids)
		};
	}
}

impl From<tokio_postgres::Row> for DeepAccount {
	fn from(value: tokio_postgres::Row) -> Self {
		let id: i32 = value.get(0);
		let name: String = value.get(1);
		let default_currency_id: i32 = value.get(2);
		let default_currency_minor_in_mayor: i32 = value.get(3);
		let default_currency_name: String = value.get(4);
		let default_currency_symbol: String = value.get(5);
		let user_id: Option<i32> = value.get(6);
		let user_name: Option<String> = value.get(7);
		let user_superuser: Option<bool> = value.get(8);
		let tag_ids: Vec<Option<i32>> = value.get(9);
		let tag_names: Vec<Option<String>> = value.get(10);
		let tag_parent_ids: Vec<Option<i32>> = value.get(11);
		let tag_parent_names: Vec<Option<String>> = value.get(12);
		let tag_parent_parent_ids: Vec<Option<i32>> = value.get(13);
		let tag_parent_user_ids: Vec<Option<i32>> = value.get(14);
		let tag_user_ids: Vec<Option<i32>> = value.get(15);
		let tag_user_names: Vec<Option<String>> = value.get(16);
		let tag_user_superusers: Vec<Option<bool>> = value.get(17);
	
		let default_currency = crate::currency::Currency {
			id: Some(default_currency_id as u32),
			name: default_currency_name,
			minor_in_mayor: default_currency_minor_in_mayor as u32,
			symbol: default_currency_symbol
		};
	
		let user = user_id.map(|_| crate::user::User {
			id: Some(user_id.unwrap() as u32),
			name: user_name.unwrap(),
			secret: None,
			superuser: user_superuser.unwrap(),
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
			
	
		return DeepAccount {
			id: id as u32,
			name,
			default_currency,
			user,
			tags,
		}
	}
}