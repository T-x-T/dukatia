mod db;
pub mod rest_api;

use serde::{Serialize, Deserialize};
use serde_repr::*;
use chrono::prelude::*;
use deadpool_postgres::Pool;
use std::error::Error;
use super::account;
use super::asset::Asset;

#[derive(Debug, Copy, Clone, Serialize_repr)]
#[repr(u8)]
pub enum TransactionStatus {
	Withheld = 0, 
	Completed = 1,
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

	pub async fn save(mut self, pool: &Pool) -> Result<(), Box<dyn Error>> {
		let account = account::get_by_id(&pool, self.account_id).await?;
		self = self.set_currency_id(account.default_currency_id);

		if self.id.is_some() {
			return Ok(db::update(&pool, &self).await?);
		} else {
			return Ok(db::add(&pool, &self).await?);
		}
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Position {
	pub id: Option<u32>,
	pub amount: i32,
	pub comment: Option<String>,
	pub tag_id: Option<u32>,
}

pub async fn get_all(pool: &Pool) -> Result<Vec<Transaction>, Box<dyn Error>> {
	return db::get_all(&pool).await;
}

pub async fn get_all_deep(pool: &Pool) -> Result<Vec<DeepTransaction>, Box<dyn Error>> {
	return db::get_all_deep(pool).await;
}

pub async fn get_by_id(pool: &Pool, transaction_id: u32) -> Result<Transaction, Box<dyn Error>> {
	return db::get_by_id(pool, transaction_id).await;
}

pub async fn get_by_asset_id(pool: &Pool, asset_id: u32) -> Result<Vec<Transaction>, Box<dyn Error>> {
	return db::get_by_asset_id(pool, asset_id).await;
}

pub async fn delete_by_id(pool: &Pool, transaction_id: u32) -> Result<(), Box<dyn Error>> {
	return db::delete_by_id(&pool, transaction_id).await;
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::{setup, teardown};

	fn get_transaction() -> Transaction {
		return Transaction::default()
			.set_currency_id(0)
			.set_comment("this is a comment".to_string())
			.set_positions(vec![Position {amount: 12345, ..Default::default()}]);
	}

	mod add {
		use super::*;
		use super::super::super::tag;

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_without_tag_ids() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			get_transaction().save(&pool).await?;
			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_with_tag_ids() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

			get_transaction()
				.set_tag_ids(vec![0, 1, 2])
				.save(&pool).await?;

			teardown(&config).await;
			return Ok(());
		}
	}

	mod get_all {
		use super::*;
		use super::super::super::tag;

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_no_results_on_empty_db() {
			let (config, pool) = setup().await;

			let res = get_all(&pool).await.unwrap();
			assert_eq!(res.len(), 0);

			teardown(&config).await;
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_all_rows() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			get_transaction().save(&pool).await?;
			get_transaction().save(&pool).await?;
			get_transaction().save(&pool).await?;

			let res = get_all(&pool).await?;
			assert_eq!(res.len(), 3);

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_single_tag_correctly() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

			get_transaction()
				.set_tag_ids(vec![0])
				.save(&pool).await?;

			let res = get_all(&pool).await?;
			assert_eq!(res[0].clone().tag_ids.unwrap(), vec![0]);

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_multiple_tags_correctly() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

			get_transaction()
				.set_tag_ids(vec![0, 1, 2])
				.save(&pool).await?;
			get_transaction()
				.set_tag_ids(vec![0, 1, 2])
				.save(&pool).await?;

			let res = get_all(&pool).await?;
			assert_eq!(res[0].clone().tag_ids.unwrap(), vec![0, 1, 2]);

			teardown(&config).await;
			return Ok(());
		}
	}
}