mod db;
pub mod rest_api;
pub mod chart;

use serde::Serialize;
use std::error::Error;
use deadpool_postgres::Pool;
use super::CustomError;
use crate::traits::*;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Tag {
	pub id: Option<u32>,
	pub name: String,
	pub user_id: u32,
	pub parent_id: Option<u32>,
}

impl Save for Tag {
	async fn save(self, pool: &Pool) -> Result<u32, Box<dyn Error>> {
		if self.parent_id.is_some() && !self.is_valid_parent(pool).await {
			return Err(Box::new(CustomError::InvalidItem{reason: String::from("parent doesn't exist or would create a cyclic relationship")}));
		}

		match self.id {
			Some(id) => {
				db::TagDbWriter::new(pool, self).replace().await?;
				return Ok(id);
			},
			None => return db::TagDbWriter::new(pool, self).insert().await,
		}
	}
}

impl Delete for Tag {
	async fn delete(self, pool: &Pool) -> Result<(), Box<dyn Error>> {
		match self.id {
			Some(_) => return db::TagDbWriter::new(pool, self).delete().await,
			None => return Err(Box::new(crate::CustomError::MissingProperty { property: "id".to_string(), item_type: "Tag".to_string() }))
		}
	}
}

impl Tag {
	async fn is_valid_parent(&self, pool: &Pool) -> bool {
		if self.parent_id.is_none() {
			return true;
		}

		if TagLoader::new(pool).set_filter_id(self.parent_id.unwrap(), NumberFilterModes::Exact).get_first().await.is_err() {
			return false;
		}

		if self.id.is_none() {
			return true;
		}

		//Check cyclic dependency
		let mut next_parent_id_to_check = self.parent_id.unwrap();
		loop {
			if next_parent_id_to_check == self.id.unwrap() {
				return false;
			}
			let next_tag = TagLoader::new(pool).set_filter_id(next_parent_id_to_check, NumberFilterModes::Exact).get_first().await;
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

	pub fn set_id(mut self, id: u32) -> Self {
		self.id = Some(id);
		return self;
	}

	pub fn set_name(mut self, name: String) -> Self {
		self.name = name;
		return self;
	}

	pub fn set_user_id(mut self, user_id: u32) -> Self {
		self.user_id = user_id;
		return self;
	}

	#[allow(dead_code)]
	pub fn set_parent_id(mut self, parent_id: u32) -> Self {
		self.parent_id = Some(parent_id);
		return self;
	}

	pub fn set_parent_id_opt(mut self, parent_id: Option<u32>) -> Self {
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