mod db;
pub mod rest_api;

use deadpool_postgres::Pool;
use serde::{Serialize, Deserialize};
use std::error::Error;
use chrono::{DateTime, Utc};
use crate::transaction::{Transaction, TransactionLoader};
use crate::traits::*;
use crate::CustomError;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TotalCostOfOwnership {
	pub total: i32,
	pub monthly: i32,
	pub yearly: i32,
}



#[derive(Debug, Clone, Serialize, Default, Deserialize)]
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

impl Save for Asset {
	async fn save(self, pool: &Pool) -> Result<u32, Box<dyn Error>> {
		match self.id {
			Some(id) => {
				db::AssetDbWriter::new(pool, self).replace().await?;
				return Ok(id);
			},
			None => db::AssetDbWriter::new(pool, self).insert().await,
		}
	}
}

impl Delete for Asset {
	async fn delete(self, pool: &Pool) -> Result<(), Box<dyn Error>> {
		match self.id {
			Some(_) => return db::AssetDbWriter::new(pool, self).delete().await,
			None => return Err(Box::new(CustomError::MissingProperty { property: "id".to_string(), item_type: "Asset".to_string() }))
		}
	}
}

impl Asset {
	pub fn set_id(mut self, id: u32) -> Self {
		self.id = Some(id);
		return self;
	}

	pub fn set_user_id(mut self, user_id: u32) -> Self {
		self.user_id = user_id;
		return self;
	}

	pub fn set_name(mut self, name: String) -> Self {
		self.name = name;
		return self;
	}

	#[allow(unused)]
	pub fn set_description(mut self, description: String) -> Self {
		self.description = Some(description);
		return self;
	}

	pub fn set_description_opt(mut self, description: Option<String>) -> Self {
		self.description = description;
		return self;
	}

	pub fn set_currency_id(mut self, currency_id: u32) -> Self {
		self.currency_id = currency_id;
		return self;
	}

	#[allow(unused)]
	pub fn set_tag_ids(mut self, tag_ids: Vec<u32>) -> Self {
		self.tag_ids = Some(tag_ids);
		return self;
	}

	pub fn set_tag_ids_opt(mut self, tag_ids: Option<Vec<u32>>) -> Self {
		self.tag_ids = tag_ids;
		return self;
	}

	pub async fn get_total_cost_of_ownership(self, pool: &Pool) -> Result<Self, Box<dyn Error>> {
		if self.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty { property: "id".to_string(), item_type: "Asset".to_string() }));
		}

		let transactions = TransactionLoader::new(pool)
			.set_filter_asset_id(self.id.unwrap(), NumberFilterModes::Exact)
			.get().await?;
		
		return Ok(Self {
			total_cost_of_ownership: Some(actually_get_total_cost_of_ownership(transactions, self.amount.unwrap_or(0.0) == 0.0)),
			..self
		});
	}

	pub async fn replace_valuation_history(self, pool: &Pool, asset_valuations: Vec<AssetValuation>) -> Result<(), Box<dyn Error>> {
		match self.id {
			Some(_) => db::AssetDbWriter::new(pool, self).replace_valuation_history(asset_valuations).await,
			None => Err(Box::new(CustomError::MissingProperty { property: "id".to_string(), item_type: "Asset".to_string() }))
		}
	}
}

#[derive(Debug, Clone)]
pub struct AssetLoader<'a> {
	pool: &'a Pool,
	query_parameters: QueryParameters,
}

impl<'a> Loader<'a, Asset> for AssetLoader<'a> {
	fn new(pool: &'a Pool) -> Self {
		Self {
			pool,
			query_parameters: QueryParameters::default(),
		}
	}

	async fn get(self) -> Result<Vec<Asset>, Box<dyn Error>> {
		let assets: Vec<Asset> = futures_util::future::try_join_all(
			db::AssetDbReader::new(self.pool)
				.set_query_parameters(self.query_parameters)
				.execute()
				.await?
				.into_iter()
				.map(|x| x.get_total_cost_of_ownership(self.pool))
		).await?;

		return Ok(assets);
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
pub struct AssetValuation {
	pub value_per_unit: u32,
	pub amount: f64,
	pub timestamp: DateTime<Utc>,
	pub asset_id: u32,
}

impl Save for AssetValuation {
	async fn save(self, pool: &Pool) -> Result<u32, Box<dyn Error>> {
		let valuation_history = AssetValuationLoader::new(pool).set_filter_asset_id(self.asset_id, NumberFilterModes::Exact).get().await?;
		let newer_than_input: Vec<&AssetValuation> = valuation_history.iter()
			.filter(
				|x| x.timestamp.signed_duration_since(self.timestamp).num_seconds() > 0
			).collect();
		
		if newer_than_input.is_empty() {
			return db::AssetValuationDbWriter::new(pool, self).insert().await;
		}

		let mut last_asset_valuation_amount: f64 = 0.0;
		for x in &valuation_history {
			if x.timestamp.signed_duration_since(self.timestamp).num_seconds() < 0 {
				last_asset_valuation_amount = x.amount;
			}
		}
		
		let difference: f64 = self.amount - last_asset_valuation_amount;

		let older_than_input: Vec<&AssetValuation> = valuation_history.iter()
		.filter(
			|x| x.timestamp.signed_duration_since(self.timestamp).num_seconds() < 0
		).collect();

		let newer_than_input: Vec<AssetValuation> = newer_than_input.into_iter().map(|x| {
			let mut y = x.clone();
			y.amount += difference;
			return y;
		}).collect();

		let mut new_asset_valuations: Vec<AssetValuation> = older_than_input.into_iter().cloned().collect();
		new_asset_valuations.push(self.clone());
		newer_than_input.into_iter().for_each(|x| new_asset_valuations.push(x));

		db::AssetDbWriter::new(pool, Asset::default().set_id(self.asset_id)).replace_valuation_history(new_asset_valuations).await?;

		return Ok(0);
	}
}

#[derive(Debug, Clone)]
pub struct AssetValuationLoader<'a> {
	pool: &'a Pool,
	query_parameters: QueryParameters,
}

impl<'a> Loader<'a, AssetValuation> for AssetValuationLoader<'a> {
	fn new(pool: &'a Pool) -> Self {
		Self {
			pool,
			query_parameters: QueryParameters::default(),
		}
	}

	async fn get(self) -> Result<Vec<AssetValuation>, Box<dyn Error>> {
		return db::AssetValuationDbReader::new(self.pool)
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

fn actually_get_total_cost_of_ownership(mut transactions: Vec<Transaction>, current_amount_is_zero: bool) -> TotalCostOfOwnership {
	if transactions.is_empty() {
		return TotalCostOfOwnership::default();
	}
	
	let total_cost_of_ownership: i32 = transactions
		.iter()
		.map(|x| x.total_amount.clone().unwrap())
		.sum();

	transactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
	let first_timestamp = transactions.pop().unwrap().timestamp;
	
	transactions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

	let last_timestamp = if transactions.is_empty() {
		Utc::now()
	} else if current_amount_is_zero {
		transactions.pop().unwrap().timestamp
	} else {
		Utc::now()
	};

	
	let days_since_first_transaction: i32 = if last_timestamp.signed_duration_since(first_timestamp).num_days() > 0 {
		i32::try_from(last_timestamp.signed_duration_since(first_timestamp).num_days()).unwrap_or(1)
	} else {
		1
	};
	
	return TotalCostOfOwnership {
		total: total_cost_of_ownership,
		monthly: (total_cost_of_ownership / days_since_first_transaction) * 30,
		yearly: (total_cost_of_ownership / days_since_first_transaction) * 365,
	};
}