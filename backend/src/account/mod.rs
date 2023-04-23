mod db;
pub mod rest_api;

use serde::Serialize;
use std::error::Error;
use deadpool_postgres::Pool;

#[derive(Debug, Clone, Serialize)]
pub struct Account {
	pub id: Option<u32>,
	pub name: String,
	pub default_currency_id: u32,
	pub user_id: u32,
	pub tag_ids: Option<Vec<u32>>
}

pub async fn add(pool: &Pool, account: &Account) -> Result<(), Box<dyn Error>> {
	return db::add(pool, account).await;
}

pub async fn get_all(pool: &Pool) -> Result<Vec<Account>, Box<dyn Error>> {
	return db::get_all(pool).await;
}

pub async fn get_by_id(pool: &Pool, account_id: u32) -> Result<Account, Box<dyn Error>> {
	return db::get_by_id(pool, account_id).await;
}

pub async fn update(pool: &Pool, account: &Account) -> Result<(), Box<dyn Error>> {
	return db::update(pool, account).await;
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::{setup, teardown};

	fn get_account() -> Account {
		return Account {
			id: None,
			name: String::from("test"),
			default_currency_id: 0,
			user_id: 0,
			tag_ids: None,
		};
	}

	mod add {
		use super::*;
		use super::super::super::tag;

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_without_tag_ids() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			add(&pool, &get_account()).await?;

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_with_tag_ids() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

			let mut account = get_account();
			account.tag_ids = Some(vec![0, 1, 2]);
			add(&pool, &account).await?;

			teardown(&config).await;
			return Ok(());
		}
	}

	mod get_all {
		use super::*;
		use super::super::super::tag;

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_on_default_db() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			get_all(&pool).await?;
			
			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_all_rows() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

			let account = get_account();
			add(&pool, &account).await?;
			add(&pool, &account).await?;
			add(&pool, &account).await?;

			let res = get_all(&pool).await?;
			assert_eq!(res.len(), 4);

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_single_tag_correctly() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

			let mut account = get_account();
			account.tag_ids = Some(vec![0]);
			add(&pool, &account).await?;

			let res = get_all(&pool).await?;
			assert_eq!(res[1].clone().tag_ids.unwrap(), vec![0]);

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_multiple_tags_correctly() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

			let mut account = get_account();
			account.tag_ids = Some(vec![0, 1, 2]);
			add(&pool, &account).await?;
			add(&pool, &account).await?;

			let res = get_all(&pool).await?;
			assert_eq!(res[1].clone().tag_ids.unwrap(), vec![0, 1, 2]);

			teardown(&config).await;
			return Ok(());
		}
	}
}