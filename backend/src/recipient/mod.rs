mod db;
pub mod rest_api;
pub mod chart;

use serde::Serialize;
use uuid::Uuid;
use std::error::Error;
use deadpool_postgres::Pool;
use crate::traits::*;
#[derive(Debug, Clone, Serialize)]
pub struct Recipient {
	pub id: Uuid,
	pub name: String,
	pub user_id: Option<u32>,
	pub tag_ids: Option<Vec<u32>>, //TODO: fix nonsensical Option
}

impl Default for Recipient {
	fn default() -> Self {
		return Self {
			id: Uuid::new_v4(),
			name: String::new(),
			user_id: None,
			tag_ids: None
		};
	}
}

impl Create for Recipient {
	async fn create(self, pool: &Pool) -> Result<Uuid, Box<dyn Error>> {
		return db::RecipientDbWriter::new(pool, self).insert().await;
	}
}

impl Update for Recipient {
	async fn update(self, pool: &Pool) -> Result<(), Box<dyn Error>> {
		return db::RecipientDbWriter::new(pool, self).replace().await;
	}
}

impl Recipient {
	pub fn set_id(mut self, id: Uuid) -> Self {
		self.id = id;
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