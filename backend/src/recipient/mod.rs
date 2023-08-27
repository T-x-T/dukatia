mod db;
pub mod rest_api;

#[cfg(test)]
mod test;

use serde::Serialize;
use std::error::Error;
use deadpool_postgres::Pool;
use crate::traits::*;
#[derive(Debug, Clone, Serialize, Default)]
pub struct Recipient {
	pub id: Option<u32>,
	pub name: String,
	pub user_id: Option<u32>,
	pub tag_ids: Option<Vec<u32>>,
}

impl Save for Recipient {
	async fn save(self, pool: &Pool) -> Result<u32, Box<dyn Error>> {
		match self.id {
			Some(id) => {
				db::RecipientDbWriter::new(pool, self).replace().await?;
				return Ok(id)
			},
			None => return db::RecipientDbWriter::new(pool, self).insert().await,
		}
	}
}

impl Recipient {
	pub fn set_id(mut self, id: u32) -> Self {
		self.id = Some(id);
		return self;
	}

	pub fn set_name(mut self, name: String) -> Self {
		self.name = name;
		return self;
	}

	pub fn set_user_id(mut self, user_id: u32) -> Self {
		self.user_id = Some(user_id);
		return self;
	}

	#[allow(dead_code)]
	pub fn set_user_id_opt(mut self, user_id: Option<u32>) -> Self {
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
pub struct RecipientLoader<'a> {
	pool: &'a Pool,
	query_parameters: QueryParameters,
}

impl<'a> Loader<'a, Recipient> for RecipientLoader<'a> {
	fn new(pool: &'a Pool) -> Self {
		Self {
			pool,
			query_parameters: QueryParameters::default(),
		}
	}

	async fn get(self) -> Result<Vec<Recipient>, Box<dyn Error>> {
		return db::RecipientDbReader::new(self.pool)
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