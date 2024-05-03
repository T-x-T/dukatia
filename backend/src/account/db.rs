use deadpool_postgres::Pool;
use std::error::Error;
use uuid::Uuid;
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

impl<'a> DbWriter<'a, Account> for AccountDbWriter<'a> {
	fn new(pool: &'a Pool, item: Account) -> Self {
		Self {
			pool,
			account: item,
		}
	}

	async fn insert(self) -> Result<Uuid, Box<dyn Error>> {
		let client = self.pool.get().await?;
		client
			.query(
				"INSERT INTO public.accounts (id, name, default_currency_id, user_id) VALUES ($1, $2, $3, $4);",
				&[&self.account.id, &self.account.name, &self.account.default_currency_id, &self.account.user_id]
			)
			.await?;
			 
		for tag_id in self.account.clone().tag_ids {
			client.query("INSERT INTO public.account_tags (account_id, tag_id) VALUES ($1, $2);", &[&self.account.id, &tag_id]).await?;
		}
		
		return Ok(self.account.id);
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		let old = super::AccountLoader::new(self.pool)
			.set_filter_id(self.account.id, NumberFilterModes::Exact)
			.get_first().await?;

		if old.user_id != self.account.user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
		}
	
		let client = self.pool.get().await?;
	
		client.query(
			"UPDATE public.accounts SET name=$1, default_currency_id=$2 WHERE id=$3;",
			&[&self.account.name, &self.account.default_currency_id, &self.account.id]
		)
		.await?;
	
		client.query(
			"DELETE FROM public.account_tags WHERE account_id=$1;",
			&[&self.account.id]
		)
		.await?;
	
		for tag_id in self.account.tag_ids.clone() {
			client.query(
				"INSERT INTO public.account_tags (account_id, tag_id) VALUES ($1, $2);",
				&[&self.account.id, &tag_id]
			)
			.await?;
		}
	
		return Ok(());
	}
}

#[allow(clippy::unwrap_or_default)]
impl From<tokio_postgres::Row> for Account {
	fn from(value: tokio_postgres::Row) -> Self {
		let id: Uuid = value.get(0);
		let name: String = value.get(1);
		let default_currency_id: Uuid = value.get(2);
		let user_id: Uuid = value.get(3);
		let tag_ids: Vec<Uuid> = value.try_get(4).unwrap_or_default();
		let balance: Option<i64> = value.get(5);
	
		return Account {
			id,
			name,
			default_currency_id,
			user_id,
			tag_ids,
			balance,
		};
	}
}