use deadpool_postgres::Pool;
use std::error::Error;



#[derive(Debug, Default, Clone)]
pub struct QueryParameters {
	pub max_results: Option<u32>,
	pub skip_results: Option<u32>,
	pub filters: Filters,
}

#[derive(Debug, Default, Clone)]
pub struct Filters {
	pub id: Option<u32>,
	pub asset_id: Option<u32>,
}




pub trait Save {
	async fn save(self, pool: &Pool) -> Result<(), Box<dyn Error>>;
}

pub trait Delete {
	async fn delete(self, pool: &Pool) -> Result<(), Box<dyn Error>>;
}

pub trait Loader<T: Clone>: Sized + Clone {
	fn get_query_parameters(self) -> QueryParameters;
	fn set_query_parameters(self, query_parameters: QueryParameters) -> Self;
	async fn get(self) -> Result<Vec<T>, Box<dyn Error>>;
	
	fn set_filter_id(self, id: u32) -> Self {
		let mut query_parameters = self.clone().get_query_parameters();
		query_parameters.filters.id = Some(id);
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_asset_id(self, asset_id: u32) -> Self {
		let mut query_parameters = self.clone().get_query_parameters();
		query_parameters.filters.asset_id = Some(asset_id);
		return self.set_query_parameters(query_parameters);
	}

	async fn get_first(self) -> Result<T, Box<dyn Error>> {
		match self.get().await?.first() {
			Some(x) => return Ok(x.clone()),
			None => return Err(Box::new(crate::CustomError::NoItemFound { item_type: "unknown".to_string() })),
		}
	}
}