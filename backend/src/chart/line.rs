use std::error::Error;
use std::collections::BTreeMap;
use deadpool_postgres::Pool;
use serde::Serialize;
use chrono::{DateTime, Date, Utc, MIN_DATETIME, MAX_DATETIME};

use super::{Chart, ChartData};

use crate::CustomError;
use crate::transaction;
use crate::currency;
use crate::recipient;

#[derive(Debug, Clone, Serialize)]
pub struct Point {
	pub timestamp: DateTime<Utc>,
	pub value: f64,
	pub label: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PointWithCurrencies {
	pub timestamp: DateTime<Utc>,
	pub value: BTreeMap<u32, i32>,
}

pub async fn get_chart_data(pool: &Pool, chart: Chart) -> Result<ChartData, Box<dyn Error>> {
	if chart.filter_collection.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("filter_collection"), item_type: String::from("chart") }));
	}

	let output = match chart.filter_collection.as_ref().unwrap().as_str() {
		"recipients" => compute_recipients(pool, chart).await?,
		_ => return Err(Box::new(CustomError::InvalidItem { reason: format!("Pie chart collection {} is not recognized", chart.filter_collection.unwrap()) })),
	};

	return Ok(ChartData { text: None, pie: None, line: Some(output) });
}

async fn compute_recipients(pool: &Pool, chart: Chart) -> Result<BTreeMap<u32, Vec<Point>>, Box<dyn Error>> {
	let mut raw_output: BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>> = BTreeMap::new();
	let currencies = currency::get_all(&pool).await?;
	let transactions = get_relevant_time_sorted_transactions(&pool, &chart).await?;
	let recipients = recipient::get_all(&pool).await?;

	transactions.into_iter().for_each(|transaction| {
		if !raw_output.contains_key(&transaction.recipient_id) {
			raw_output.insert(transaction.recipient_id, BTreeMap::new());
		}

		if !raw_output.get(&transaction.recipient_id).unwrap().contains_key(&transaction.timestamp.date()) {
			let mut value: BTreeMap<u32, i32> = BTreeMap::new();
			value.insert(transaction.currency_id.unwrap(), transaction.amount);

			let point = PointWithCurrencies {
				timestamp: transaction.timestamp.clone(),
				value,
			};

			raw_output.entry(transaction.recipient_id).or_insert(BTreeMap::new()).insert(transaction.timestamp.date(), point);
		} else {
			let new_point = PointWithCurrencies {
				timestamp: transaction.timestamp,
				value: BTreeMap::new(),
			};
			
			*raw_output.entry(transaction.recipient_id)
				.or_insert(BTreeMap::new())
				.entry(transaction.timestamp.date())
				.or_insert(new_point)
				.value.entry(transaction.currency_id.unwrap())
				.or_insert(0) += transaction.amount;
		}
	});

	let mut output: BTreeMap<u32, Vec<Point>> = BTreeMap::new();
	//Turn PointWithCurrencies into Point
	return Ok(output);
}


async fn get_relevant_time_sorted_transactions(pool: &Pool, chart: &Chart) -> Result<Vec<transaction::Transaction>, Box<dyn Error>> {
	let from_date = chart.filter_from.unwrap_or(MIN_DATETIME);
	let to_date = chart.filter_to.unwrap_or(MAX_DATETIME);

	let mut transactions = transaction::get_all(&pool).await?;
	transactions.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

	return Ok(transactions.into_iter()
		.filter(|x| {
		return &from_date.signed_duration_since(x.timestamp).num_seconds() <= &0 
				&& &to_date.signed_duration_since(x.timestamp).num_seconds() >= &0;
		})
		.collect());
}