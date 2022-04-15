mod db;
pub mod rest_api;

use serde::Serialize;
use std::error::Error;
use deadpool_postgres::Pool;

#[derive(Debug, Clone, Serialize)]
pub struct Currency {
	id: Option<u32>,
	name: String,
	minor_in_mayor: u32,
	symbol: String,
}

pub async fn get_all(pool: &Pool) -> Result<Vec<Currency>, Box<dyn Error>> {
	return db::get_all(&pool).await;
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::{setup, teardown};

	#[tokio::test]
	async fn doesnt_panic() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		get_all(&pool).await?;

		teardown(&config).await;
		return Ok(());
	}

	#[tokio::test]
	async fn returns_two_rows() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		let res = get_all(&pool).await?;
		assert_eq!(res.len(), 2);
		
		teardown(&config).await;
		return Ok(());
	}
}