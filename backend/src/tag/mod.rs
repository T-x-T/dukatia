mod db;
pub mod rest_api;
pub mod chart;

use serde::Serialize;
use std::error::Error;
use deadpool_postgres::Pool;
use uuid::Uuid;
use super::CustomError;
use crate::traits::*;

#[derive(Debug, Clone, Serialize)]
pub struct Tag {
	pub id: Uuid,
	pub name: String,
	pub user_id: Uuid,
	pub parent_id: Option<Uuid>,
}

impl Default for Tag {
	fn default() -> Self {
		return Self {
			id: Uuid::new_v4(),
			name: String::new(),
			user_id: Uuid::nil(),
			parent_id: None,
		};
	}
}

impl Create for Tag {
	async fn create(self, pool: &Pool) -> Result<Uuid, Box<dyn Error>> {
		if self.parent_id.is_some() && !self.is_valid_parent(pool).await {
			return Err(Box::new(CustomError::InvalidItem{reason: String::from("parent doesn't exist or would create a cyclic relationship")}));
		}

		return db::TagDbWriter::new(pool, self).insert().await;
	}
}

impl Update for Tag {
	async fn update(self, pool: &Pool) -> Result<(), Box<dyn Error>> {
		if self.parent_id.is_some() && !self.is_valid_parent(pool).await {
			return Err(Box::new(CustomError::InvalidItem{reason: String::from("parent doesn't exist or would create a cyclic relationship")}));
		}

		return db::TagDbWriter::new(pool, self).replace().await;
	}
}

impl Delete for Tag {
	async fn delete(self, pool: &Pool) -> Result<(), Box<dyn Error>> {
		return db::TagDbWriter::new(pool, self).delete().await;
	}
}

impl Tag {
	async fn is_valid_parent(&self, pool: &Pool) -> bool {
		if self.parent_id.is_none() {
			return true;
		}

		if TagLoader::new(pool).set_filter_id_uuid(self.parent_id.unwrap(), NumberFilterModes::Exact).get_first().await.is_err() {
			return false;
		}

		//Check cyclic dependency
		let mut next_parent_id_to_check = self.parent_id.unwrap();
		loop {
			if next_parent_id_to_check == self.id {
				return false;
			}
			let next_tag = TagLoader::new(pool).set_filter_id_uuid(next_parent_id_to_check, NumberFilterModes::Exact).get_first().await;
			if next_tag.is_err() {
				break;
			}
			let next_tag_parent = next_tag.unwrap().parent_id;
			if next_tag_parent.is_none() {
				break;
			}
			
			next_parent_id_to_check = next_tag_parent.unwrap();
		}
	
		return true;
	}

	pub fn set_id(mut self, id: Uuid) -> Self {
		self.id = id;
		return self;
	}

	pub fn set_name(mut self, name: String) -> Self {
		self.name = name;
		return self;
	}

	pub fn set_user_id(mut self, user_id: Uuid) -> Self {
		self.user_id = user_id;
		return self;
	}

	#[allow(dead_code)]
	pub fn set_parent_id(mut self, parent_id: Uuid) -> Self {
		self.parent_id = Some(parent_id);
		return self;
	}

	pub fn set_parent_id_opt(mut self, parent_id: Option<Uuid>) -> Self {
		self.parent_id = parent_id;
		return self;
	}
}

#[derive(Debug, Clone)]
pub struct TagLoader<'a> {
	pool: &'a Pool,
	query_parameters: QueryParameters,
}

impl<'a> Loader<'a, Tag> for TagLoader<'a> {
	fn new(pool: &'a Pool) -> Self {
		Self {
			pool,
			query_parameters: QueryParameters::default(),
		}
	}

	async fn get(self) -> Result<Vec<Tag>, Box<dyn Error>> {
		return db::TagDbReader::new(self.pool)
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