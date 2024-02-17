use deadpool_postgres::Pool;
use std::error::Error;
use uuid::Uuid;
use crate::CustomError;
use super::{Asset, AssetValuation};
use crate::traits::*;
use crate::money::Money;

#[derive(Debug)]
pub struct AssetDbReader<'a> {
	query_parameters: QueryParameters,
	pool: &'a Pool,
}

impl<'a> DbReader<'a, Asset> for AssetDbReader<'a> {
	fn new(pool: &'a Pool) -> Self {
		return Self {
			query_parameters: QueryParameters::default(),
			pool,
		}
	}

	fn get_pool(&self) -> &Pool {
		return self.pool;
	}

	fn get_query_parameters(&self) -> &QueryParameters {
		return &self.query_parameters;
	}

	fn set_query_parameters(mut self, query_parameters: QueryParameters) -> Self {
		self.query_parameters = query_parameters;
		return self;
	}

	async fn execute(self) -> Result<Vec<Asset>, Box<dyn Error>> {
		let query = "SELECT * FROM public.asset_data";
		return Ok(
			self.actually_execute(query)
			.await?
			.into_iter()
			.map(Into::into)
			.collect()
		);
	}
}

#[derive(Debug)]
pub struct AssetValuationDbReader<'a> {
	query_parameters: QueryParameters,
	pool: &'a Pool,
}

impl<'a> DbReader<'a, AssetValuation> for AssetValuationDbReader<'a> {
	fn new(pool: &'a Pool) -> Self {
		return Self {
			query_parameters: QueryParameters::default(),
			pool,
		}
	}

	fn get_pool(&self) -> &Pool {
		return self.pool;
	}

	fn get_query_parameters(&self) -> &QueryParameters {
		return &self.query_parameters;
	}

	fn set_query_parameters(mut self, query_parameters: QueryParameters) -> Self {
		self.query_parameters = query_parameters;
		return self;
	}

	async fn execute(self) -> Result<Vec<AssetValuation>, Box<dyn Error>> {
		let query = "SELECT * FROM public.asset_valuation_history";
		return Ok(
			self.actually_execute(query)
			.await?
			.into_iter()
			.map(Into::into)
			.collect()
		);
	}
}




#[derive(Debug)]
pub struct AssetDbWriter<'a> {
	pool: &'a Pool,
	asset: Asset,
}

impl<'a> DbWriter<'a, Asset> for AssetDbWriter<'a> {
	fn new(pool: &'a Pool, item: Asset) -> Self {
		Self { 
			pool,
			asset: item
		}
	}

	async fn insert(self) -> Result<Uuid, Box<dyn Error>> {
		let client = self.pool.get().await?;

		client.query(
				"INSERT INTO public.assets (id, name, description, user_id, currency_id) VALUES ($1, $2, $3, $4, $5)", 
				&[&self.asset.id, &self.asset.name, &self.asset.description, &(self.asset.user_id as i32), &(self.asset.currency_id as i32)]
			).await?;

		if self.asset.tag_ids.is_some() {
			for tag_id in self.asset.tag_ids.clone().unwrap() {
				client.query(
						"INSERT INTO public.asset_tags (asset_id, tag_id) VALUES ($1, $2);",
						&[&self.asset.id, &(tag_id as i32)]
					).await?;
			}
		}

		return Ok(self.asset.id);
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		let old = super::AssetLoader::new(self.pool)
			.set_filter_id_uuid(self.asset.id, NumberFilterModes::Exact)
			.get_first().await?;

		if old.user_id != self.asset.user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
		}
	
		let client = self.pool.get().await?;
	
		client.query(
			"UPDATE public.assets SET name=$1, description=$2 WHERE id=$3",
			&[&self.asset.name, &self.asset.description, &self.asset.id]
		).await?;
	
		client.query(
			"DELETE FROM public.asset_tags WHERE asset_id=$1", 
			&[&self.asset.id]
		).await?;
	
		if self.asset.tag_ids.is_some() {
			for tag_id in self.asset.tag_ids.clone().unwrap() {
				client.query(
						"INSERT INTO public.asset_tags (asset_id, tag_id) VALUES ($1, $2);",
						&[&self.asset.id, &(tag_id as i32)]
					).await?;
			}
		}
	
		return Ok(());
	}
}

impl<'a> AssetDbWriter<'a> {
	pub async fn replace_valuation_history(self, asset_valuations: Vec<AssetValuation>) -> Result<(), Box<dyn Error>> {
		super::AssetLoader::new(self.pool)
			.set_filter_id_uuid(self.asset.id, NumberFilterModes::Exact)
			.get_first().await?;
	
		let client = self.pool.get().await?;
	
		client.query(
			"DELETE FROM public.asset_amounts WHERE asset_id=$1",
			&[&self.asset.id]
		).await?;
	
		client.query(
			"DELETE FROM public.asset_valuations WHERE asset_id=$1",
			&[&self.asset.id]
		).await?;
	
		for asset_valuation in asset_valuations {
			AssetValuationDbWriter::new(self.pool, asset_valuation).insert().await?;
		}
	
		return Ok(());
	}
}

#[derive(Debug)]
pub struct AssetValuationDbWriter<'a> {
	pool: &'a Pool,
	asset_valuation: AssetValuation,
}

impl<'a> OldDbWriter<'a, AssetValuation> for AssetValuationDbWriter<'a> {
	fn new(pool: &'a Pool, item: AssetValuation) -> Self {
		Self { 
			pool,
			asset_valuation: item
		}
	}

	async fn insert(self) -> Result<u32, Box<dyn Error>> {
		super::AssetLoader::new(self.pool).set_filter_id_uuid(self.asset_valuation.asset_id, NumberFilterModes::Exact).get_first().await?;
	
		let client = self.pool.get().await?;
		
		client.query(
			"INSERT INTO public.asset_amounts (asset_id, timestamp, amount) VALUES ($1, $2, $3);",
			&[&self.asset_valuation.asset_id, &self.asset_valuation.timestamp, &self.asset_valuation.amount]
		).await?;
		
		client.query(
			"INSERT INTO public.asset_valuations (asset_id, timestamp, value_per_unit) VALUES ($1, $2, $3)", 
			&[&self.asset_valuation.asset_id, &self.asset_valuation.timestamp, &self.asset_valuation.value_per_unit.to_amount()]
		).await?;
	
		return Ok(0);
	}

	#[allow(clippy::unused_async)]
	async fn replace(self) -> Result<(), Box<dyn Error>> {
		return Err(Box::new(CustomError::InvalidActionForItem { action: "replace".to_string(), item_type: "AssetValuation".to_string() }))
	}
}

impl<'a> DbDeleter<'a, Asset> for AssetDbWriter<'a> {
	async fn delete(self) -> Result<(), Box<dyn Error>> {
		let old = super::AssetLoader::new(self.pool)
			.set_filter_id_uuid(self.asset.id, NumberFilterModes::Exact)
			.get_first().await?;

		if old.user_id != self.asset.user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
		}

		self.pool.get()
			.await?
			.query(
				"DELETE FROM public.assets WHERE id=$1;", 
				&[&self.asset.id]
			).await?;

		return Ok(());
	}
}

#[allow(clippy::unwrap_or_default)]
impl From<tokio_postgres::Row> for Asset {
	fn from(value: tokio_postgres::Row) -> Self {
		let id: Uuid = value.get(0);
		let name: String = value.get(1);
		let description: Option<String> = value.get(2);
		let user_id: i32 = value.get(3);
		let currency_id: i32 = value.get(4);
		let tag_ids: Vec<u32> = value.try_get(5)
			.unwrap_or(Vec::new())
			.into_iter()
			.map(|x: i32| x as u32)
			.collect();
		let amount: f64 = value.try_get(6).unwrap_or(0.0);
		let value_per_unit: i32 = value.try_get(7).unwrap_or(0);
		let minor_in_major: i32 = value.get(8);
		let symbol: String = value.get(9);

		return Asset {
			id,
			name,
			description,
			user_id: user_id as u32,
			currency_id: currency_id as u32,
			tag_ids: Some(tag_ids),
			value_per_unit: Some(Money::from_amount(value_per_unit, minor_in_major as u32, symbol)),
			amount: Some(amount),
			total_cost_of_ownership: None,
		}
	}
}

impl From<tokio_postgres::Row> for AssetValuation {
	fn from(value: tokio_postgres::Row) -> Self {
		let asset_id: Uuid = value.get(0);
		let timestamp: chrono::DateTime<chrono::Utc> = value.get(1);
		let amount: f64 = value.try_get(2).unwrap_or(0.0);
		let value_per_unit: i32 = value.try_get(3).unwrap_or(0);
		let minor_in_major: i32 = value.get(4);
		let symbol: String = value.get(5);
	
		return AssetValuation {
			asset_id,
			value_per_unit: Money::from_amount(value_per_unit, minor_in_major as u32, symbol),
			amount,
			timestamp
		}
	}
}