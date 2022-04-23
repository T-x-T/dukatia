mod db;
pub mod rest_api;

use serde::Serialize;
use std::error::Error;
use deadpool_postgres::Pool;

#[derive(Debug, Clone, Serialize)]
pub struct Recipient {
	pub id: Option<u32>,
	pub name: String,
	pub user_id: Option<u32>,
	pub tag_ids: Option<Vec<u32>>,
}

pub async fn add(pool: &Pool, recipient: &Recipient) -> Result<(), Box<dyn Error>> {
	return db::add(&pool, &recipient).await;
}

pub async fn get_all(pool: &Pool) -> Result<Vec<Recipient>, Box<dyn Error>> {
	return db::get_all(&pool).await;
}

pub async fn update(pool: &Pool, recipient: &Recipient) -> Result<(), Box<dyn Error>> {
	return db::update(&pool, &recipient).await;
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::{setup, teardown};

	fn get_recipient() -> Recipient {
		return Recipient {
			id: None,
			name: String::from("thisisaname"),
			user_id: Some(0),
			tag_ids: None,
		};
	}

	mod add {
		use super::*;
		use super::super::super::tag;

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_without_tag_ids() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			add(&pool, &get_recipient()).await?;

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic_with_tag_ids() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;
			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

			let mut recipient = get_recipient();
			recipient.tag_ids = Some(vec![0, 1, 2]);
			add(&pool, &recipient).await?;

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

			let recipient = get_recipient();

			add(&pool, &recipient).await?;
			add(&pool, &recipient).await?;
			add(&pool, &recipient).await?;

			let res = get_all(&pool).await?;
			assert_eq!(res.len(), 4);

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_single_tag_correctly() -> Result<(), Box<dyn Error>> {
			let (config, pool) = setup().await;

			tag::add(&pool, &tag::Tag{id: None, name: String::from("test_tag"), user_id: 0, parent_id: None}).await?;

			let mut recipient = get_recipient();
			recipient.tag_ids = Some(vec![0]);
			add(&pool, &recipient).await?;

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

			let mut recipient = get_recipient();
			recipient.tag_ids = Some(vec![0, 1, 2]);
			add(&pool, &recipient).await?;

			let res = get_all(&pool).await?;
			assert_eq!(res[1].clone().tag_ids.unwrap(), vec![0, 1, 2]);

			teardown(&config).await;
			return Ok(());
		}
	}
}