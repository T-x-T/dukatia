use deadpool_postgres::Pool;
use std::error::Error;
use super::Currency;
use super::super::CustomError;

pub async fn get_all(pool: &Pool) -> Result<Vec<Currency>, Box<dyn Error>> {
	let rows = pool.get().await?
		.query("SELECT * FROM public.\"Currencies\";", &[])
		.await?;
	
	return Ok(
		rows
			.iter()
			.map(|x| turn_row_into_currency(&x))
			.collect()
	);
}

pub async fn get_by_id(pool: &Pool, currency_id: u32) -> Result<Currency, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query("SELECT * FROM public.\"Currencies\" WHERE id=$1;", &[&(currency_id as i32)])
		.await?;

	if rows.is_empty() {
		return Err(Box::new(CustomError::SpecifiedItemNotFound { item_type: String::from("currency"), filter: format!("id={currency_id}") } ));
	}

	return Ok(turn_row_into_currency(&rows[0]));
}

pub async fn add(pool: &Pool, currency: &Currency) -> Result<(), Box<dyn Error>> {
	pool.get()
		.await
		.unwrap()
		.query(
			"INSERT INTO public.\"Currencies\" (id, name, minorinmayor, symbol) VALUES (DEFAULT, $1, $2, $3)", 
			&[&currency.name, &(currency.minor_in_mayor as i32), &currency.symbol])
		.await?;

	return Ok(());
}

pub async fn update(pool: &Pool, currency: &Currency) -> Result<(), Box<dyn Error>> {
	if currency.id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("currency") }));
	}

	get_by_id(&pool, currency.id.unwrap()).await?;

	pool.get()
		.await
		.unwrap()
		.query(
			"UPDATE public.\"Currencies\" SET name=$1, minorinmayor=$2, symbol=$3 WHERE id=$4", 
			&[&currency.name, &(currency.minor_in_mayor as i32), &currency.symbol, &(currency.id.unwrap() as i32)])
		.await?;

	return Ok(());
}

fn turn_row_into_currency(row: &tokio_postgres::Row) -> Currency {
	let id: i32 = row.get(0);
	let minor_in_mayor: i32 = row.get(2);
	return Currency {
		id: Some(id as u32),
		name: row.get(1),
		minor_in_mayor: minor_in_mayor as u32,
		symbol: row.get(3)
	};
}