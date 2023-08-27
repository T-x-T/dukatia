mod db;
pub mod rest_api;
#[cfg(test)]
mod test;

use serde::Serialize;
use std::error::Error;
use deadpool_postgres::Pool;
use crate::traits::*;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Account {
	pub id: Option<u32>,
	pub name: String,
	pub default_currency_id: u32,
	pub user_id: u32,
	pub tag_ids: Option<Vec<u32>>,
	pub balance: Option<i64>,
}

impl Save for Account {
	async fn save(self, pool: &Pool) -> Result<u32, Box<dyn Error>> {
		match self.id {
			Some(id) => {
				db::AccountDbWriter::new(pool, self).replace().await?;
				return Ok(id);
			},
			None => return db::AccountDbWriter::new(pool, self).insert().await,
		}
	}
}

impl Account {
	pub fn set_id(mut self, id: u32) -> Self {
		self.id = Some(id);
		return self;
	}

	pub fn set_name(mut self, name: String) -> Self {
		self.name = name;
		return self;
	}

	pub fn set_default_currency_id(mut self, default_currency_id: u32) -> Self {
		self.default_currency_id = default_currency_id;
		return self;
	}

	pub fn set_user_id(mut self, user_id: u32) -> Self {
		self.user_id = user_id;
		return self;
	}

	#[allow(dead_code)]
	pub fn set_tag_ids(mut self, tag_ids: Vec<u32>) -> Self {
		self.tag_ids = Some(tag_ids);
		return self;
	}

	pub fn set_tag_ids_opt(mut self, tag_ids: Option<Vec<u32>>) -> Self {
		self.tag_ids = tag_ids;
		return self;
	}
}

#[derive(Debug, Clone)]
pub struct AccountLoader<'a> {
	pool: &'a Pool,
	query_parameters: QueryParameters,
}

impl<'a> Loader<'a, Account> for AccountLoader<'a> {
	fn new(pool: &'a Pool) -> Self {
		Self {
			pool,
			query_parameters: QueryParameters::default(),
		}
	}

	async fn get(self) -> Result<Vec<Account>, Box<dyn Error>> {
		return db::AccountDbReader::new(self.pool)
			.set_query_parameters(self.query_parameters)
			.execute()
			.await;
	}

	fn get_query_parameters(&self) -> &QueryParameters {
		return &self.query_parameters;
	}

	fn set_query_parameters(mut self, query_parameters: QueryParameters) -> Self {
		self.query_parameters = query_parameters;
		return self;
	}
}