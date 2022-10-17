mod db;
pub mod rest_api;

use deadpool_postgres::Pool;
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Clone, Serialize)]
pub struct Asset {
	pub id: Option<u32>,
	pub user_id: u32,
	pub name: String,
	pub description: Option<String>,
	pub currency_id: u32,
	pub value_per_unit: Option<u32>,
	pub amount: Option<f64>,
	pub tag_ids: Option<Vec<u32>>,
}

pub async fn add(pool: &Pool, asset: &Asset) -> Result<(), Box<dyn Error>> {
	return db::add(&pool, &asset).await;
}

pub async fn get_all(pool: &Pool) -> Result<Vec<Asset>, Box<dyn Error>> {
	return db::get_all(&pool).await;
}

pub async fn update(pool: &Pool, asset: &Asset) -> Result<(), Box<dyn Error>> {
	return db::update(&pool, &asset).await;
}

pub async fn delete_by_id(pool: &Pool, asset_id: u32) -> Result<(), Box<dyn Error>> {
	return db::delete_by_id(&pool, asset_id).await;
}