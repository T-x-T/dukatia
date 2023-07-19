use deadpool_postgres::Pool;
use std::error::Error;
use crate::CustomError;
use super::{Asset, AssetValuation, DeepAsset};
use crate::traits::*;

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
pub struct DeepAssetDbReader<'a> {
	query_parameters: QueryParameters,
	pool: &'a Pool,
}

impl<'a> DbReader<'a, DeepAsset> for DeepAssetDbReader<'a> {
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

	async fn execute(self) -> Result<Vec<DeepAsset>, Box<dyn Error>> {
		let query = "SELECT * FROM public.deep_assets";
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

	async fn insert(self) -> Result<u32, Box<dyn Error>> {
		let client = self.pool.get().await?;

		let id: i32 = client.query(
				"INSERT INTO public.assets (id, name, description, user_id, currency_id) VALUES (DEFAULT, $1, $2, $3, $4) RETURNING id;", 
				&[&self.asset.name, &self.asset.description, &(self.asset.user_id as i32), &(self.asset.currency_id as i32)]
			).await?
			[0].get(0);

		if self.asset.tag_ids.is_some() {
			for tag_id in self.asset.tag_ids.clone().unwrap() {
				client.query(
						"INSERT INTO public.asset_tags (asset_id, tag_id) VALUES ($1, $2);",
						&[&id, &(tag_id as i32)]
					).await?;
			}
		}

		return Ok(id as u32);
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		if self.asset.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("asset") }));
		}
	
		super::AssetLoader::new(self.pool).set_filter_id(self.asset.id.unwrap()).get_first().await?;
	
		let client = self.pool.get().await?;
	
		client.query(
			"UPDATE public.assets SET name=$1, description=$2 WHERE id=$3",
			&[&self.asset.name, &self.asset.description, &(self.asset.id.unwrap() as i32)]
		).await?;
	
		client.query(
			"DELETE FROM public.asset_tags WHERE asset_id=$1", 
			&[&(self.asset.id.unwrap() as i32)]
		).await?;
	
		if self.asset.tag_ids.is_some() {
			for tag_id in self.asset.tag_ids.clone().unwrap() {
				client.query(
						"INSERT INTO public.asset_tags (asset_id, tag_id) VALUES ($1, $2);",
						&[&(self.asset.id.unwrap() as i32), &(tag_id as i32)]
					).await?;
			}
		}
	
		return Ok(());
	}
}

impl<'a> AssetDbWriter<'a> {
	pub async fn replace_valuation_history(self, asset_valuations: Vec<AssetValuation>) -> Result<(), Box<dyn Error>> {
		if self.asset.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("asset") }));
		}

		super::AssetLoader::new(self.pool).set_filter_id(self.asset.id.unwrap()).get_first().await?;
	
		let client = self.pool.get().await?;
	
		client.query(
			"DELETE FROM public.asset_amounts WHERE asset_id=$1",
			&[&(self.asset.id.unwrap() as i32)]
		).await?;
	
		client.query(
			"DELETE FROM public.asset_valuations WHERE asset_id=$1",
			&[&(self.asset.id.unwrap() as i32)]
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

impl<'a> DbWriter<'a, AssetValuation> for AssetValuationDbWriter<'a> {
	fn new(pool: &'a Pool, item: AssetValuation) -> Self {
		Self { 
			pool,
			asset_valuation: item
		}
	}

	async fn insert(self) -> Result<u32, Box<dyn Error>> {
		super::AssetLoader::new(self.pool).set_filter_id(self.asset_valuation.asset_id).get_first().await?;
	
		let client = self.pool.get().await?;
		
		client.query(
			"INSERT INTO public.asset_amounts (asset_id, timestamp, amount) VALUES ($1, $2, $3);",
			&[&(self.asset_valuation.asset_id as i32), &self.asset_valuation.timestamp, &self.asset_valuation.amount]
		).await?;
		
		client.query(
			"INSERT INTO public.asset_valuations (asset_id, timestamp, value_per_unit) VALUES ($1, $2, $3)", 
			&[&(self.asset_valuation.asset_id as i32), &self.asset_valuation.timestamp, &(self.asset_valuation.value_per_unit as i32)]
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
		if self.asset.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("asset") }));
		}

		self.pool.get()
			.await?
			.query(
				"DELETE FROM public.assets WHERE id=$1;", 
				&[&(self.asset.id.unwrap() as i32)]
			).await?;

		return Ok(());
	}
}

impl From<tokio_postgres::Row> for Asset {
	fn from(value: tokio_postgres::Row) -> Self {
		let id: i32 = value.get(0);
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

		return Asset {
			id: Some(id as u32),
			name,
			description,
			user_id: user_id as u32,
			currency_id: currency_id as u32,
			tag_ids: Some(tag_ids),
			value_per_unit: Some(value_per_unit as u32),
			amount: Some(amount),
			total_cost_of_ownership: None,
		}
	}
}

impl From<tokio_postgres::Row> for AssetValuation {
	fn from(value: tokio_postgres::Row) -> Self {
		let asset_id: i32 = value.get(0);
		let timestamp: chrono::DateTime<chrono::Utc> = value.get(1);
		let amount: f64 = value.try_get(2).unwrap_or(0.0);
		let value_per_unit: i32 = value.try_get(3).unwrap_or(0);
	
		return AssetValuation {
			asset_id: asset_id as u32,
			value_per_unit: value_per_unit as u32,
			amount,
			timestamp
		}
	}
}

impl From<tokio_postgres::Row> for DeepAsset {
	fn from(value: tokio_postgres::Row) -> Self {
		let id: i32 = value.get(0);
		let name: String = value.get(1);
		let description: Option<String> = value.get(2);
		let value_per_unit: i32 = value.try_get(3).unwrap_or(0);
		let amount: f64 = value.try_get(4).unwrap_or(0.0);
		let currency_id: i32 = value.get(5);
		let currency_minor_in_mayor: i32 = value.get(6);
		let currency_name: String = value.get(7);
		let currency_symbol: String = value.get(8);
		let user_id: i32 = value.get(9);
		let user_name: String = value.get(10);
		let user_superuser: bool = value.get(11);
		let tag_ids: Vec<Option<i32>> = value.get(12);
		let tag_names: Vec<Option<String>> = value.get(13);
		let tag_parent_ids: Vec<Option<i32>> = value.get(14);
		let tag_parent_names: Vec<Option<String>> = value.get(15);
		let tag_parent_parent_ids: Vec<Option<i32>> = value.get(16);
		let tag_parent_user_ids: Vec<Option<i32>> = value.get(17);
		let tag_user_ids: Vec<Option<i32>> = value.get(18);
		let tag_user_names: Vec<Option<String>> = value.get(19);
		let tag_user_superusers: Vec<Option<bool>> = value.get(20);

		let currency = crate::currency::Currency {
			id: Some(currency_id as u32),
			name: currency_name,
			minor_in_mayor: currency_minor_in_mayor as u32,
			symbol: currency_symbol
		};

		let user = crate::user::User {
			id: Some(user_id as u32),
			name: user_name,
			secret: None,
			superuser: user_superuser
		};

		let tags: Vec<crate::tag::DeepTag> = tag_ids
			.into_iter()
			.filter(Option::is_some)
			.enumerate()
			.map(|(i, tag_id)| {
				let parent: Option<crate::tag::Tag> = match tag_parent_ids.get(i) {
					Some(x) => {
						x.as_ref().map(|_| crate::tag::Tag {
							id: tag_parent_ids[i].map(|x| x as u32),
							name: tag_parent_names[i].clone().unwrap(),
							user_id: tag_parent_user_ids[i].unwrap() as u32,
							parent_id: tag_parent_parent_ids[i].map(|x| x as u32),
						})
					},
					None => None,
				};
				
				crate::tag::DeepTag {
					id: tag_id.unwrap() as u32,
					name: tag_names[i].clone().unwrap(),
					user: crate::user::User {
						id: tag_user_ids[i].map(|x| x as u32),
						name: tag_user_names[i].clone().unwrap(),
						secret: None,
						superuser: tag_user_superusers[i].unwrap(),
					},
					parent,
				}
			}).collect();

		return DeepAsset {
			id: id as u32,
			name,
			description,
			value_per_unit: value_per_unit as u32,
			amount,
			user,
			currency,
			tags,
			total_cost_of_ownership: None,
		}		
	}
}