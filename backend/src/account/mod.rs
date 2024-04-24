mod db;
pub mod rest_api;
pub mod chart;

use serde::Serialize;
use std::error::Error;
use deadpool_postgres::Pool;
use uuid::Uuid;
use crate::traits::*;

#[derive(Debug, Clone, Serialize)]
pub struct Account {
	pub id: Uuid,
	pub name: String,
	pub default_currency_id: u32,
	pub user_id: u32,
	pub tag_ids: Vec<Uuid>,
	pub balance: Option<i64>,
}

impl Default for Account {
	fn default() -> Self {
		Self {
			id: Uuid::new_v4(),
			name: String::new(),
			default_currency_id: 0,
			user_id: 0,
			tag_ids: Vec::new(), 
			balance: None
		}
	}
}

impl Create for Account {
	async fn create(self, pool: &Pool) -> Result<Uuid, Box<dyn Error>> {
		return db::AccountDbWriter::new(pool, self).insert().await;
	}
}

impl Update for Account {
	async fn update(self, pool: &Pool) -> Result<(), Box<dyn Error>> {
		return db::AccountDbWriter::new(pool, self).replace().await;
	}
}

impl Account {
	pub fn set_id(mut self, id: Uuid) -> Self {
		self.id = id;
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

	pub fn set_tag_ids(mut self, tag_ids: Vec<Uuid>) -> Self {
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