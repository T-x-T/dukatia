mod db;
pub mod rest_api;

use deadpool_postgres::Pool;
use serde::Serialize;
use std::{error::Error, collections::BTreeMap};
use chrono::{DateTime, Utc, Date};

#[derive(Debug, Clone, Serialize)]
pub struct Asset {
	pub id: Option<u32>,
	pub user_id: u32,
	pub name: String,
	pub description: Option<String>,
	pub currency_id: u32,
	pub value_per_unit: Option<u32>,
	pub amount: Option<f64>,
	pub tag_ids: Option<Vec<u32>>
}

#[derive(Debug, Clone, Serialize)]
pub struct AssetValuation {
	pub value_per_unit: u32,
	pub amount: f64,
	pub timestamp: DateTime<Utc>,
}

pub async fn add(pool: &Pool, asset: &Asset) -> Result<u32, Box<dyn Error>> {
	return db::add(&pool, &asset).await;
}

pub async fn add_valuation(pool: &Pool, asset_id: u32, asset_valuation: &AssetValuation) -> Result<(), Box<dyn Error>> {
	let valuation_history = get_valuation_history_by_asset_id(&pool, asset_id).await?;
	let newer_than_input: Vec<&AssetValuation> = valuation_history.iter()
		.filter(
			|x| x.timestamp.signed_duration_since(asset_valuation.timestamp).num_seconds() > 0
		).collect();
	
	if newer_than_input.len() > 0 {
		let mut last_asset_valuation_amount: f64 = 0.0;
		for x in &valuation_history {
			if x.timestamp.signed_duration_since(asset_valuation.timestamp).num_seconds() < 0 {
				last_asset_valuation_amount = x.amount;
			}
		}
		
		let difference: f64 = asset_valuation.amount - last_asset_valuation_amount;

		let older_than_input: Vec<&AssetValuation> = valuation_history.iter()
		.filter(
			|x| x.timestamp.signed_duration_since(asset_valuation.timestamp).num_seconds() < 0
		).collect();

		let newer_than_input: Vec<AssetValuation> = newer_than_input.into_iter().map(|x| {
			let mut y = x.clone();
			y.amount += difference;
			return y;
		}).collect();

		let mut new_asset_valuations: Vec<AssetValuation> = older_than_input.into_iter().map(|x| x.clone()).collect();
		new_asset_valuations.push(asset_valuation.clone());
		newer_than_input.into_iter().for_each(|x| new_asset_valuations.push(x));

		return db::replace_valuation_history_of_asset(&pool, asset_id, new_asset_valuations).await;
	} else {
		return db::add_valuation(&pool, asset_id, &asset_valuation).await;
	}

}

pub async fn get_all(pool: &Pool) -> Result<Vec<Asset>, Box<dyn Error>> {
	return db::get_all(&pool).await;
}

#[allow(unused)]
pub async fn get_all_from_user(pool: &Pool, user_id: u32) -> Result<Vec<Asset>, Box<dyn Error>> {
	return db::get_all_from_user(pool, user_id).await;
}

pub async fn get_by_id(pool: &Pool, asset_id: u32) -> Result<Asset, Box<dyn Error>> {
	return db::get_by_id(&pool, asset_id).await;
}

pub async fn update(pool: &Pool, asset: &Asset) -> Result<(), Box<dyn Error>> {
	return db::update(&pool, &asset).await;
}

#[allow(unused)]
pub async fn get_total_value_at_day(pool: &Pool, asset_id: u32, date: Date<Utc>) -> Result<f64, Box<dyn Error>> {
	let amount = db::get_amount_at_day(pool, asset_id, date).await?;
	let value = db::get_value_at_day(pool, asset_id, date).await? as f64;
	return Ok(amount * value);
}

pub async fn get_valuation_history_by_asset_id(pool: &Pool, asset_id: u32) -> Result<Vec<AssetValuation>, Box<dyn Error>> {
	return db::get_valuation_history_by_asset_id(&pool, asset_id).await;
}

pub async fn replace_valuation_history_of_asset(pool: &Pool, asset_id: u32, asset_valuations: Vec<AssetValuation>) -> Result<(), Box<dyn Error>> {
	return db::replace_valuation_history_of_asset(&pool, asset_id, asset_valuations).await;
}

pub async fn delete_by_id(pool: &Pool, asset_id: u32) -> Result<(), Box<dyn Error>> {
	return db::delete_by_id(&pool, asset_id).await;
}

pub async fn get_value_per_unit_history(pool: &Pool, asset_id: u32) -> Result<BTreeMap<chrono::DateTime<chrono::Utc>, u32>, Box<dyn Error>> {
	return db::get_value_per_unit_history(&pool, asset_id).await;
}

pub async fn get_amount_history(pool: &Pool, asset_id: u32) -> Result<BTreeMap<chrono::DateTime<chrono::Utc>, f64>, Box<dyn Error>> {
	return db::get_amount_history(&pool, asset_id).await;
}