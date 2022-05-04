pub mod rest_api;

use std::collections::BTreeMap;
use std::error::Error;
use deadpool_postgres::Pool;
use serde::Serialize;

use super::transaction;
use super::currency;

#[derive(Debug, Serialize)]
pub struct Output {
	pub x: chrono::NaiveDate,
	pub y: i32
}

pub async fn balance_over_time_per_currency(
	pool: &Pool,
	from_date: Option<chrono::NaiveDate>,
	to_date: Option<chrono::NaiveDate>
) -> Result<BTreeMap<u32, Vec<Output>>, Box<dyn Error>> {
	let mut transactions = transaction::get_all(&pool).await?;
	transactions.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

	let currencies = currency::get_all(&pool).await?;

	let mut output_map: BTreeMap<u32, Vec<Output>> = BTreeMap::new();
	for currency in currencies {
		let filtered_transactions = transactions.iter().filter(|x| x.currency_id.unwrap() == currency.id.unwrap());

		let mut data: BTreeMap<chrono::NaiveDate, i32> = BTreeMap::new();

		//Fill data while adding values from same date
		for filtered_transaction in filtered_transactions {
			if data.contains_key(&filtered_transaction.timestamp.date().naive_local()) {
				data.insert(filtered_transaction.timestamp.date().naive_local(), data.get(&filtered_transaction.timestamp.date().naive_local()).unwrap() + filtered_transaction.amount);
			} else {
				data.insert(filtered_transaction.timestamp.date().naive_local(), filtered_transaction.amount);
			}
		}

		//Create rolling sum of values
		let mut prev: i32 = 0;
		for i in 0..data.len() {
			let cur_key = data.iter().nth(i).unwrap().0.clone();
			let cur_val = data.iter().nth(i).unwrap().1.clone();
			data.insert(cur_key, prev + cur_val);
			prev = data.iter().nth(i).unwrap().1.clone();
		}

		//Optional date filtering
		if from_date.is_some() && to_date.is_some() {
			data = data.drain_filter(|k, _v| &from_date.unwrap().signed_duration_since(*k).num_seconds() <= &0 && &to_date.unwrap().signed_duration_since(*k).num_seconds() >= &0).collect();
		}

		let mut output: Vec<Output> = Vec::new();
		for i in 0..data.len() {
			output.push(Output { x: data.iter().nth(i).unwrap().0.clone(), y: data.iter().nth(i).unwrap().1.clone() });
		}
		output_map.insert(currency.id.unwrap(), output);

	}
	return Ok(output_map);
}