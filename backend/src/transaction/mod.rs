mod db;
pub mod rest_api;

use serde::Serialize;
use serde_repr::*;
use chrono::prelude::*;
use deadpool_postgres::Pool;
use std::error::Error;
use super::account;

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
	pub amount: i32,
	pub comment: Option<String>,
	pub tag_ids: Option<Vec<u32>>,
}

pub async fn add(pool: &Pool, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
	let mut transaction = transaction.clone();

	transaction.currency_id = Some(
		account::get_all(&pool)
			.await?
			.into_iter()
			.filter(|x| x.id.unwrap() == transaction.account_id)
			.collect::<Vec<account::Account>>()[0]
			.default_currency_id
		);

	return db::add(&pool, &transaction).await;
}

pub async fn get_all(pool: &Pool) -> Result<Vec<Transaction>, Box<dyn Error>> {
	return db::get_all(&pool).await;
}

pub async fn update(pool: &Pool, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
	let mut transaction = transaction.clone();

	transaction.currency_id = Some(
		account::get_all(&pool)
			.await?
			.into_iter()
			.filter(|x| x.id.unwrap() == transaction.account_id)
			.collect::<Vec<account::Account>>()[0]
			.default_currency_id
		);

	return db::update(&pool, &transaction).await;
}

pub async fn delete_by_id(pool: &Pool, transaction_id: u32) -> Result<(), Box<dyn Error>> {
	return db::delete_by_id(&pool, transaction_id).await;
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::{setup, teardown};

	fn get_transaction() -> Transaction {
		return Transaction {
			id: None,
			user_id: 0,
			currency_id: Some(0),
			account_id: 0,
			recipient_id: 0,
			status: TransactionStatus::Completed,
			timestamp: Utc::now(),
			amount: 12345,
			comment: Some(String::from("this is a comment")),
			tag_ids: None,
		};
	}

	mod add {
		use super::*;
		use super::super::super::tag;

		#[tokio::test]
		async fn doesnt_panic_without_tag_ids() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			add(&pool, &get_transaction()).await?;
			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test]
		async fn doesnt_panic_with_tag_ids() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

			let mut transaction = get_transaction();
			transaction.tag_ids = Some(vec![0, 1, 2]);
			add(&pool, &transaction).await?;

			teardown(&config).await;
			return Ok(());
		}
	}

	mod get_all {
		use super::*;
		use super::super::super::tag;

		#[tokio::test]
		async fn returns_error_on_default_db() {
			let (config, pool) = setup().await;

			let res = get_all(&pool).await;
			
			teardown(&config).await;

			match res {
				Ok(_) => panic!("this should return an error"),
				Err(_) => return (),
			};
		}

		#[tokio::test]
		async fn returns_all_rows() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			let transaction = get_transaction();
			add(&pool, &transaction).await?;
			add(&pool, &transaction).await?;
			add(&pool, &transaction).await?;

			let res = get_all(&pool).await?;
			assert_eq!(res.len(), 3);

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test]
		async fn returns_single_tag_correctly() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

			let mut transaction = get_transaction();
			transaction.tag_ids = Some(vec![0]);
			add(&pool, &transaction).await?;

			let res = get_all(&pool).await?;
			assert_eq!(res[0].clone().tag_ids.unwrap(), vec![0]);

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test]
		async fn returns_multiple_tags_correctly() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

			let mut transaction = get_transaction();
			transaction.tag_ids = Some(vec![0, 1, 2]);
			add(&pool, &transaction).await?;
			add(&pool, &transaction).await?;

			let res = get_all(&pool).await?;
			assert_eq!(res[0].clone().tag_ids.unwrap(), vec![0, 1, 2]);

			teardown(&config).await;
			return Ok(());
		}
	}
}