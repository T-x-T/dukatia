mod db;
pub mod rest_api;

use serde::Serialize;
use std::error::Error;
use deadpool_postgres::Pool;

#[derive(Debug, Clone, Serialize)]
pub struct Currency {
	pub id: Option<u32>,
	pub name: String,
	pub minor_in_mayor: u32,
	pub symbol: String,
}

pub async fn get_all(pool: &Pool) -> Result<Vec<Currency>, Box<dyn Error>> {
	return db::get_all(&pool).await;
}

pub async fn get_by_id(pool: &Pool, currency_id: u32) -> Result<Currency, Box<dyn Error>> {
	return db::get_by_id(&pool, currency_id).await;
}

pub async fn add(pool: &Pool, currency: &Currency) -> Result<(), Box<dyn Error>> {
	return db::add(pool, currency).await;
}

pub async fn update(pool: &Pool, currency: &Currency) -> Result<(), Box<dyn Error>> {
	return db::update(pool, currency).await;
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::{setup, teardown};

	#[tokio::test(flavor = "multi_thread")]
	async fn doesnt_panic() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		get_all(&pool).await?;

		teardown(&config).await;
		return Ok(());
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn returns_two_rows() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		let res = get_all(&pool).await?;
		assert_eq!(res.len(), 2);
		
		teardown(&config).await;
		return Ok(());
	}
}