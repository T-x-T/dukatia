mod db;
pub mod rest_api;

#[cfg(test)]
mod test;

use serde::{Serialize, Deserialize};
use serde_repr::*;
use chrono::prelude::*;
use deadpool_postgres::Pool;
use std::error::Error;
use super::account;
use super::asset::Asset;
use crate::traits::*;

#[derive(Debug, Copy, Clone, Serialize_repr)]
#[repr(u8)]
pub enum TransactionStatus {
	Withheld = 0, 
	Completed = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Position {
	pub id: Option<u32>,
	pub amount: i32,
	pub comment: Option<String>,
	pub tag_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
	pub id: Option<u32>,
	pub user_id: u32,
	pub currency_id: Option<u32>,
	pub account_id: u32,
	pub recipient_id: u32,
	pub status: TransactionStatus,
	pub timestamp: DateTime<Utc>,
	pub total_amount: Option<i32>,
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
			account_id: 0,
			recipient_id: 0,
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
		let account = account::AccountLoader::new(pool).set_filter_id(self.account_id).get_first().await?;
		self = self.set_currency_id(account.default_currency_id);
		let id = self.id; //TODO: use a match instead
		
		let db_writer = db::TransactionDbWriter::new(pool, self);

		if id.is_some() {
			db_writer.replace().await?;
			return Ok(id.unwrap());
		}
		
		return db_writer.insert().await;
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

	pub fn set_account_id(mut self, account_id: u32) -> Self {
		self.account_id = account_id;
		return self;
	}

	pub fn set_recipient_id(mut self, recipient_id: u32) -> Self {
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

	pub fn set_total_amount(mut self, total_amount: i32) -> Self {
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






#[derive(Debug, Clone, Serialize)]
pub struct DeepTransaction {
	pub id: u32,
	pub status: TransactionStatus,
	pub timestamp: DateTime<Utc>,
	pub total_amount: Option<i32>,
	pub comment: Option<String>,
	pub currency: crate::currency::Currency,
	pub user: crate::user::User,
	pub account: crate::account::DeepAccount,
	pub recipient: crate::recipient::DeepRecipient,
	pub tags: Vec<crate::tag::DeepTag>,
	pub asset: Option<crate::asset::DeepAsset>,
	pub positions: Vec<Position>,
}

#[derive(Debug, Clone)]
pub struct DeepTransactionLoader<'a> {
	pool: &'a Pool,
	query_parameters: QueryParameters,
}

impl<'a> Loader<'a, DeepTransaction> for DeepTransactionLoader<'a> {
	fn new(pool: &'a Pool) -> Self {
		return Self {
			pool,
			query_parameters: QueryParameters::default(),
		};
	}

	async fn get(self) -> Result<Vec<DeepTransaction>, Box<dyn Error>> {
		return db::DeepTransactionDbReader::new(self.pool)
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