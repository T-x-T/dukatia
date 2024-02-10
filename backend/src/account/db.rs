use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::Account;
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
pub struct AccountDbWriter<'a> {
	pool: &'a Pool,
	account: Account,
}

impl<'a> OldDbWriter<'a, Account> for AccountDbWriter<'a> {
	fn new(pool: &'a Pool, item: Account) -> Self {
		Self {
			pool,
			account: item,
		}
	}

	async fn insert(self) -> Result<u32, Box<dyn Error>> {
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
		return Ok(id as u32);
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		if self.account.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("account") }));
		}
	
		let old = super::AccountLoader::new(self.pool)
			.set_filter_id(self.account.id.unwrap(), NumberFilterModes::Exact)
			.get_first().await?;

		if old.user_id != self.account.user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
		}
	
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

#[allow(clippy::unwrap_or_default)]
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
		let balance: Option<i64> = value.get(5);
	
		return Account {
			id: Some(id as u32),
			name,
			default_currency_id: default_currency_id as u32,
			user_id: user_id as u32,
			tag_ids: Some(tag_ids),
			balance,
		};
	}
}