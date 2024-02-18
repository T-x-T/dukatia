mod db;
pub mod rest_api;
pub mod chart;

use serde::{Serialize, Deserialize};
use serde_repr::*;
use chrono::prelude::*;
use deadpool_postgres::Pool;
use std::error::Error;
use uuid::Uuid;
use super::account;
use super::asset::Asset;
use crate::traits::*;
use crate::money::Money;

#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize)]
#[repr(u8)]
pub enum TransactionStatus {
	Withheld = 0, 
	Completed = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Position {
	pub id: Option<u32>,
	pub amount: Money,
	pub comment: Option<String>,
	pub tag_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransactionSummary {
	pub count: u32,
	pub total_amount: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
	pub id: Option<u32>,
	pub user_id: u32,
	pub currency_id: Option<u32>,
	pub account_id: Uuid,
	pub recipient_id: Uuid,
	pub status: TransactionStatus,
	pub timestamp: DateTime<Utc>,
	pub total_amount: Option<Money>,
	pub comment: Option<String>,
	pub tag_ids: Option<Vec<u32>>,
	pub asset: Option<Asset>,
	pub positions: Vec<Position>,
}

impl Default for Transaction {
	fn default() -> Self {
		Self { 
			id: None,
			user_id: 0,
			currency_id: None,
			account_id: Uuid::nil(),
			recipient_id: Uuid::nil(),
			status: TransactionStatus::Completed,
			timestamp: Utc::now(),
			total_amount: None,
			comment: None,
			tag_ids: None,
			asset: None,
			positions: Vec::new(),
		}
	}
}

impl Save for Transaction {
	async fn save(mut self, pool: &Pool) -> Result<u32, Box<dyn Error>> {
		let account = account::AccountLoader::new(pool).set_filter_id_uuid(self.account_id, NumberFilterModes::Exact).get_first().await?;
		self = self.set_currency_id(account.default_currency_id);

		match self.id {
			Some(id) => {
				db::TransactionDbWriter::new(pool, self).replace().await?;
				return Ok(id);
			},
			None => return db::TransactionDbWriter::new(pool, self).insert().await
		}
	}
}

impl Delete for Transaction {
	async fn delete(self, pool: &Pool) -> Result<(), Box<dyn Error>> {
		match self.id {
			Some(_) => return db::TransactionDbWriter::new(pool, self).delete().await,
			None => return Err(Box::new(crate::CustomError::MissingProperty { property: "id".to_string(), item_type: "Transaction".to_string() }))
		}
	}
}

impl Transaction {
	pub fn set_id(mut self, id: u32) -> Self {
		self.id = Some(id);
		return self;
	}

	pub fn set_user_id(mut self, user_id: u32) -> Self {
		self.user_id = user_id;
		return self;
	}

	pub fn set_currency_id(mut self, currency_id: u32) -> Self {
		self.currency_id = Some(currency_id);
		return self;
	}

	pub fn set_account_id(mut self, account_id: Uuid) -> Self {
		self.account_id = account_id;
		return self;
	}

	pub fn set_recipient_id(mut self, recipient_id: Uuid) -> Self {
		self.recipient_id = recipient_id;
		return self;
	}

	pub fn set_status(mut self, status: TransactionStatus) -> Self {
		self.status = status;
		return self;
	}

	pub fn set_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
		self.timestamp = timestamp;
		return self;
	}

	pub fn set_comment(mut self, comment: String) -> Self {
		self.comment = Some(comment);
		return self;
	}

	pub fn set_comment_opt(mut self, comment: Option<String>) -> Self {
		self.comment = comment;
		return self;
	}

	pub fn set_tag_ids(mut self, tag_ids: Vec<u32>) -> Self {
		self.tag_ids = Some(tag_ids);
		return self;
	}

	pub fn set_tag_ids_opt(mut self, tag_ids: Option<Vec<u32>>) -> Self {
		self.tag_ids = tag_ids;
		return self;
	}

	pub fn set_asset(mut self, asset: Asset) -> Self {
		self.asset = Some(asset);
		return self;
	}

	pub fn set_asset_opt(mut self, asset: Option<Asset>) -> Self {
		self.asset = asset;
		return self;
	}

	pub fn set_positions(mut self, positions: Vec<Position>) -> Self {
		self.positions = positions;
		return self;
	}

	pub fn set_total_amount(mut self, total_amount: Money) -> Self {
		self.total_amount = Some(total_amount);
		return self;
	}
}

#[derive(Debug, Clone)]
pub struct TransactionLoader<'a> {
	pool: &'a Pool,
	query_parameters: QueryParameters,
}

impl<'a> Loader<'a, Transaction> for TransactionLoader<'a> {
	fn new(pool: &'a Pool) -> Self {
		Self {
			pool,
			query_parameters: QueryParameters::default(),
		}
	}

	async fn get(self) -> Result<Vec<Transaction>, Box<dyn Error>> {
		return db::TransactionDbReader::new(self.pool)
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

impl<'a> TransactionLoader<'a> {
	pub async fn summarize(self) -> Result<TransactionSummary, Box<dyn Error>> {
		return db::TransactionDbReader::new(self.pool)
			.set_query_parameters(self.query_parameters)
			.summarize()
			.await;
	}
}