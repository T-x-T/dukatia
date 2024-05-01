use deadpool_postgres::Pool;
use uuid::Uuid;
use std::error::Error;
use super::Currency;
use crate::traits::*;

#[derive(Debug)]
pub struct CurrencyDbReader<'a> {
	query_parameters: QueryParameters,
	pool: &'a Pool,
}

impl<'a> DbReader<'a, Currency> for CurrencyDbReader<'a> {
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

	async fn execute(self) -> Result<Vec<Currency>, Box<dyn Error>> {
		let query = "SELECT * FROM public.currencies";
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
pub struct CurrencyDbWriter<'a> {
	pool: &'a Pool,
	currency: Currency,
}

impl<'a> DbWriter<'a, Currency> for CurrencyDbWriter<'a> {
	fn new(pool: &'a Pool, item: Currency) -> Self {
		Self {
			pool,
			currency: item,
		}
	}

	async fn insert(self) -> Result<Uuid, Box<dyn Error>> {
		self.pool.get()
			.await
			.unwrap()
			.query(
				"INSERT INTO public.currencies (id, name, minor_in_major, symbol) VALUES ($1, $2, $3, $4);", 
				&[&self.currency.id, &self.currency.name, &(self.currency.minor_in_major as i32), &self.currency.symbol])
			.await?;

		return Ok(self.currency.id);
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {	
		super::CurrencyLoader::new(self.pool).set_filter_id_uuid(self.currency.id, NumberFilterModes::Exact).get_first().await?;
	
		self.pool.get()
			.await
			.unwrap()
			.query(
				"UPDATE public.currencies SET name=$1, minor_in_major=$2, symbol=$3 WHERE id=$4", 
				&[&self.currency.name, &(self.currency.minor_in_major as i32), &self.currency.symbol, &self.currency.id])
			.await?;
	
		return Ok(());
	}
}

impl From<tokio_postgres::Row> for Currency {
	fn from(value: tokio_postgres::Row) -> Self {
		let name: String = value.get(0);
		let minor_in_major: i32 = value.get(1);
		let symbol: String = value.get(2);
		let id: Uuid = value.get(3);

		return Self {
			id,
			name,
			minor_in_major: minor_in_major as u32,
			symbol,
		};
	}
}