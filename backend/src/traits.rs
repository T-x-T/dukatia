use deadpool_postgres::Pool;
use postgres_types::ToSql;
use tokio_postgres::Row;
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
	pub user_id: Option<u32>,
}




pub trait Save {
	async fn save(self, pool: &Pool) -> Result<u32, Box<dyn Error>>;
}

pub trait Delete {
	async fn delete(self, pool: &Pool) -> Result<(), Box<dyn Error>>;
}

pub trait Loader<'a, T: Clone>: Sized + Clone {
	fn new(pool: &'a Pool) -> Self;
	fn get_query_parameters(&self) -> &QueryParameters;
	fn set_query_parameters(self, query_parameters: QueryParameters) -> Self;
	async fn get(self) -> Result<Vec<T>, Box<dyn Error>>;
	
	fn set_filter_id(self, id: u32) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.id = Some(id);
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_asset_id(self, asset_id: u32) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.asset_id = Some(asset_id);
		return self.set_query_parameters(query_parameters);
	}

	fn set_filter_user_id(self, user_id: u32) -> Self {
		let mut query_parameters = self.get_query_parameters().clone();
		query_parameters.filters.user_id = Some(user_id);
		return self.set_query_parameters(query_parameters);
	}

	async fn get_first(self) -> Result<T, Box<dyn Error>> {
		match self.get().await?.first() {
			Some(x) => return Ok(x.clone()),
			None => return Err(Box::new(crate::CustomError::NoItemFound { item_type: "unknown".to_string() })),
		}
	}
}

pub trait DbReader<'a, T: From<Row>>: Sized {
	fn new(pool: &'a Pool) -> Self;
	fn get_pool(&self) -> &Pool;
	fn get_query_parameters(&self) -> &QueryParameters;
	fn set_query_parameters(self, query_parameters: QueryParameters) -> Self;
	async fn execute(self) -> Result<Vec<T>, Box<dyn Error>>;

	fn get_formatted_query_parameters(&self) -> (String, Vec<Box<(dyn ToSql + Sync)>>)  {
		let mut i = 1;
		let mut parameters = String::new();
		let mut parameter_values: Vec<Box<(dyn ToSql + Sync)>> = Vec::new();

		if self.get_query_parameters().filters.id.is_some() {
			parameters.push_str(format!(" WHERE id=${i}").as_str());
			parameter_values.push(Box::new(self.get_query_parameters().filters.id.unwrap() as i32));
			i += 1;
		} else if self.get_query_parameters().filters.asset_id.is_some() {
			parameters.push_str(format!(" WHERE asset_id=${i}").as_str());
			parameter_values.push(Box::new(self.get_query_parameters().filters.asset_id.unwrap() as i32));
			i += 1;
		}

		if self.get_query_parameters().max_results.is_some() {
			parameters.push_str(format!(" LIMIT ${i}").as_str());
			i += 1;
		}
		if self.get_query_parameters().skip_results.is_some() {
			parameters.push_str(format!(" OFFSET ${i}").as_str());
		}

		return (
			parameters,
			parameter_values
		);
	}

	async fn actually_execute(self, query: &str) -> Result<Vec<Row>, Box<dyn Error>> {
		let parameters = self.get_formatted_query_parameters();
		let parameter_values: Vec<_> = parameters.1.iter()
			.map(|x| &**x as &(dyn ToSql + Sync))
			.collect();

		let rows = self.get_pool()
			.get()
			.await?
			.query(format!("{}{};", query, parameters.0).as_str(), parameter_values.as_slice())
			.await?;
	
		return Ok(rows);
	}
}

pub trait DbWriter<'a, T> {
	fn new(pool: &'a Pool, item: T) -> Self;
	async fn insert(self) -> Result<u32, Box<dyn Error>>;
	async fn replace(self) -> Result<(), Box<dyn Error>>;
}

pub trait DbDeleter<'a, T>: DbWriter<'a, T> {
	async fn delete(self) -> Result<(), Box<dyn Error>>;
}