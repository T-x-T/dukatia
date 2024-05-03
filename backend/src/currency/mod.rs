mod db;
pub mod rest_api;
pub mod chart;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::error::Error;
use deadpool_postgres::Pool;
use crate::traits::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
	pub id: Uuid,
	pub name: String,
	pub minor_in_major: u32,
	pub symbol: String,
}

impl Default for Currency {
	fn default() -> Self {
		Self {
			id: Uuid::new_v4(),
			name: String::new(),
			minor_in_major: 100,
			symbol: String::new(),
		}
	}
}

impl Create for Currency {
	async fn create(self, pool: &Pool) -> Result<Uuid, Box<dyn Error>> {
		return db::CurrencyDbWriter::new(pool, self).insert().await;
	}
}

impl Update for Currency {
	async fn update(self, pool: &Pool) -> Result<(), Box<dyn Error>> {
		return db::CurrencyDbWriter::new(pool, self).replace().await;
	}
}

impl Currency {
	pub fn set_id(mut self, id: Uuid) -> Self {
		self.id = id;
		return self;
	}

	pub fn set_name(mut self, name: String) -> Self {
		self.name = name;
		return self;
	}

	pub fn set_minor_in_major(mut self, minor_in_major: u32) -> Self {
		self.minor_in_major = minor_in_major;
		return self;
	}

	pub fn set_symbol(mut self, symbol: String) -> Self {
		self.symbol = symbol;
		return self;
	}
}

#[derive(Debug, Clone)]
pub struct CurrencyLoader<'a> {
	pool: &'a Pool,
	query_parameters: QueryParameters,
}

impl<'a> Loader<'a, Currency> for CurrencyLoader<'a> {
	fn new(pool: &'a Pool) -> Self {
		Self {
			pool,
			query_parameters: QueryParameters::default(),
		}
	}

	async fn get(self) -> Result<Vec<Currency>, Box<dyn Error>> {
		return db::CurrencyDbReader::new(self.pool)
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
