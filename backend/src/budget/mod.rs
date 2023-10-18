pub mod rest_api;
mod db;

use serde::{Serialize, Deserialize};
use std::error::Error;
use deadpool_postgres::Pool;
use crate::money::Money;
use crate::traits::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum Period {
	Daily = 0,
	Weekly = 1,
	#[default]
	Monthly = 2,
	Quarterly = 3,
	Yearly = 4,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Budget {
	pub id: Option<u32>,
  pub name: String,
	pub user_id: u32,
	pub amount: Money,
	pub rollover: bool,
	pub period: Period,
	pub filter_tag_ids: Vec<u32>,
}

impl Save for Budget {
	async fn save(self, pool: &Pool) -> Result<u32, Box<dyn Error>> {
		match self.id {
			Some(id) => {
				db::BudgetDbWriter::new(pool, self).replace().await?;
				return Ok(id)
			},
			None => return db::BudgetDbWriter::new(pool, self).insert().await,
		}
	}
}

impl Delete for Budget {
	async fn delete(self, pool: &Pool) -> Result<(), Box<dyn Error>> {
		match self.id {
			Some(_) => return db::BudgetDbWriter::new(pool, self).delete().await,
			None => return Err(Box::new(crate::CustomError::MissingProperty { property: "id".to_string(), item_type: "Budget".to_string() }))
		}
	}
}

impl Budget {
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

	pub fn set_amount(mut self, amount: u32) -> Self {
		self.amount = Money::from_amount(amount as i32, 100, "€".to_string()); //TODO: figure out currency shit
		return self;
	}

	pub fn set_rollover(mut self, rollover: bool) -> Self {
		self.rollover = rollover;
		return self;
	}

	pub fn set_period(mut self, period: Period) -> Self {
		self.period = period;
		return self;
	}

	pub fn set_filter_tag_ids(mut self, filter_tag_ids: Vec<u32>) -> Self {
		self.filter_tag_ids = filter_tag_ids;
		return self;
	}
}

#[derive(Debug, Clone)]
pub struct BudgetLoader<'a> {
	pool: &'a Pool,
	query_parameters: QueryParameters,
}

impl<'a> Loader<'a, Budget> for BudgetLoader<'a> {
	fn new(pool: &'a Pool) -> Self {
		Self {
			pool,
			query_parameters: QueryParameters::default(),
		}
	}

	async fn get(self) -> Result<Vec<Budget>, Box<dyn Error>> {
		return db::BudgetDbReader::new(self.pool)
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