use deadpool_postgres::Pool;
use std::error::Error;
use super::Currency;
use super::super::CustomError;

pub async fn get_all(pool: &Pool) -> Result<Vec<Currency>, Box<dyn Error>> {
	let rows = pool.get().await?
		.query("SELECT * FROM public.\"Currencies\";", &[])
		.await?;

	if rows.is_empty() {
		return Err(Box::new(CustomError::NoItemFound{item_type: String::from("currency")}));
	}
	
	return Ok(
		rows
			.iter()
			.map(|x| turn_row_into_currency(&x))
			.collect()
	);
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