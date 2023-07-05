mod db;
pub mod rest_api;

use deadpool_postgres::Pool;
use serde::Serialize;
use std::{error::Error, collections::BTreeMap};
use chrono::{DateTime, Utc, Date};
use crate::transaction;

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
	pub total_cost_of_ownership: Option<TotalCostOfOwnership>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeepAsset {
	pub id: u32,
	pub name: String,
	pub description: Option<String>,
	pub value_per_unit: u32,
	pub amount: f64,
	pub user: crate::user::User,
	pub currency: crate::currency::Currency,
	pub tags: Vec<crate::tag::DeepTag>,
	pub total_cost_of_ownership: Option<TotalCostOfOwnership>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct TotalCostOfOwnership {
	pub total: i32,
	pub monthly: i32,
	pub yearly: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct AssetValuation {
	pub value_per_unit: u32,
	pub amount: f64,
	pub timestamp: DateTime<Utc>,
}

pub async fn add(pool: &Pool, asset: &Asset) -> Result<u32, Box<dyn Error>> {
	return db::add(pool, asset).await;
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
	let mut output: Vec<Asset> = Vec::new();
	for asset in db::get_all(pool).await?.into_iter() {
		output.push(get_total_cost_of_ownership(pool, asset).await?);
	}
	return Ok(output);
}

pub async fn get_all_deep(pool: &Pool) -> Result<Vec<DeepAsset>, Box<dyn Error>> {
	let mut output: Vec<DeepAsset> = Vec::new();
	for asset in db::get_all_deep(pool).await?.into_iter() {
		output.push(get_total_cost_of_ownership_deep(pool, asset).await?);
	}
	return Ok(output);
}

#[allow(unused)]
pub async fn get_all_from_user(pool: &Pool, user_id: u32) -> Result<Vec<Asset>, Box<dyn Error>> {
	return db::get_all_from_user(pool, user_id).await;
}

pub async fn get_by_id(pool: &Pool, asset_id: u32) -> Result<Asset, Box<dyn Error>> {
	return get_total_cost_of_ownership(pool, db::get_by_id(pool, asset_id).await?).await;
}

pub async fn update(pool: &Pool, asset: &Asset) -> Result<(), Box<dyn Error>> {
	return db::update(pool, asset).await;
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

pub async fn get_total_cost_of_ownership(pool: &Pool, asset: Asset) -> Result<Asset, Box<dyn Error>> {
	let transactions = transaction::TransactionLoader::new(pool)
		.set_filter_asset_id(asset.id.unwrap())
		.get().await?;
	
	return Ok(Asset {
		total_cost_of_ownership: Some(actually_get_total_cost_of_ownership(transactions, if asset.amount.unwrap_or(0.0) == 0.0 { true } else { false } )),
		..asset
	});
}

pub async fn get_total_cost_of_ownership_deep(pool: &Pool, asset: DeepAsset) -> Result<DeepAsset, Box<dyn Error>> {
	let transactions = transaction::TransactionLoader::new(pool)
	.set_filter_asset_id(asset.id)
	.get().await?;

	return Ok(DeepAsset {
		total_cost_of_ownership: Some(actually_get_total_cost_of_ownership(transactions, if asset.amount == 0.0 { true } else { false } )),
		..asset
	});
}

fn actually_get_total_cost_of_ownership(mut transactions: Vec<transaction::Transaction>, current_amount_is_zero: bool) -> TotalCostOfOwnership {
	if transactions.is_empty() {
		return TotalCostOfOwnership::default();
	}
	
	let total_cost_of_ownership: i32 = transactions
		.iter()
		.map(|x| x.total_amount.unwrap())
		.sum();

	transactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
	let first_timestamp = transactions.pop().unwrap().timestamp;
	
	transactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

	let last_timestamp = if !transactions.is_empty() {
		if current_amount_is_zero {
			transactions.pop().unwrap().timestamp
		} else {
			Utc::now()
		}
	} else {
		Utc::now()
	};

	
	let days_since_first_transaction = if last_timestamp.signed_duration_since(first_timestamp).num_days() > 0 {
		last_timestamp.signed_duration_since(first_timestamp).num_days()
	} else {
		1
	};
	
	return TotalCostOfOwnership {
		total: total_cost_of_ownership,
		monthly: (total_cost_of_ownership / days_since_first_transaction as i32) * 30,
		yearly: (total_cost_of_ownership / days_since_first_transaction as i32) * 365,
	};
}