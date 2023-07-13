use deadpool_postgres::Pool;
use std::error::Error;
use super::Currency;
use crate::CustomError;
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

	async fn insert(self) -> Result<u32, Box<dyn Error>> {
		let id: i32 = self.pool.get()
			.await
			.unwrap()
			.query(
				"INSERT INTO public.currencies (id, name, minor_in_mayor, symbol) VALUES (DEFAULT, $1, $2, $3) RETURNING id;", 
				&[&self.currency.name, &(self.currency.minor_in_mayor as i32), &self.currency.symbol])
			.await?[0].get(0);

		return Ok(id as u32);
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		if self.currency.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("currency") }));
		}
	
		super::CurrencyLoader::new(self.pool).set_filter_id(self.currency.id.unwrap()).get_first().await?;
	
		self.pool.get()
			.await
			.unwrap()
			.query(
				"UPDATE public.currencies SET name=$1, minor_in_mayor=$2, symbol=$3 WHERE id=$4", 
				&[&self.currency.name, &(self.currency.minor_in_mayor as i32), &self.currency.symbol, &(self.currency.id.unwrap() as i32)])
			.await?;
	
		return Ok(());
	}
}

impl From<tokio_postgres::Row> for Currency {
	fn from(value: tokio_postgres::Row) -> Self {
		let id: i32 = value.get(0);
		let name: String = value.get(1);
		let minor_in_mayor: i32 = value.get(2);
		let symbol: String = value.get(3);

		return Self {
			id: Some(id as u32),
			name,
			minor_in_mayor: minor_in_mayor as u32,
			symbol,
		};
	}
}