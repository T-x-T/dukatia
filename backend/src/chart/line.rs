use std::error::Error;
use std::collections::BTreeMap;
use deadpool_postgres::Pool;
use serde::Serialize;
use chrono::{DateTime, Date, NaiveTime, Utc, MIN_DATETIME, MAX_DATETIME};

use super::{Chart, ChartData};

use crate::CustomError;
use crate::transaction;
use crate::currency;
use crate::recipient;
use crate::account;

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

#[derive(Debug, Clone, Serialize)]
pub struct PointWithoutCurrencies {
	pub timestamp: DateTime<Utc>,
	pub value: i32,
}

pub async fn get_chart_data(pool: &Pool, chart: Chart) -> Result<ChartData, Box<dyn Error>> {
	if chart.filter_collection.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("filter_collection"), item_type: String::from("chart") }));
	}

	let output = match chart.filter_collection.as_ref().unwrap().as_str() {
		"recipients" => compute_recipients(pool, chart).await?,
		"accounts" => compute_accounts(pool, chart).await?,
		_ => return Err(Box::new(CustomError::InvalidItem { reason: format!("Line chart collection {} is not recognized", chart.filter_collection.unwrap()) })),
	};

	return Ok(ChartData { text: None, pie: None, line: Some(output) });
}

async fn compute_recipients(pool: &Pool, chart: Chart) -> Result<BTreeMap<String, Vec<Point>>, Box<dyn Error>> {
	let currencies = currency::get_all(&pool).await?;
	let transactions = get_relevant_time_sorted_transactions(&pool, &chart).await?;
	let recipients = recipient::get_all(&pool).await?;
	
	let mut raw_output: BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>> = BTreeMap::new();
	transactions.into_iter().for_each(|transaction| {
		*raw_output.entry(transaction.recipient_id)
			.or_insert(BTreeMap::new())
			.entry(transaction.timestamp.date())
			.or_insert(PointWithCurrencies {
				timestamp: transaction.timestamp,
				value: BTreeMap::new(),
			})
			.value.entry(transaction.currency_id.unwrap())
			.or_insert(0) += transaction.amount;
	});

	let mut accumulated_raw_output: BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>> = BTreeMap::new();
	raw_output.keys().for_each(|recipient| {
		accumulated_raw_output.insert(*recipient, BTreeMap::new());
		
		let mut previous = PointWithCurrencies {
			timestamp: chrono::MIN_DATETIME,
			value: BTreeMap::new(),
		};
		raw_output.get(&recipient).unwrap().iter().for_each(|x| {

			let mut val: BTreeMap<Date<Utc>, PointWithCurrencies> = BTreeMap::new();
			val.insert(x.0.clone(), x.1.clone());
			if previous.timestamp == chrono::MIN_DATETIME {
				accumulated_raw_output.insert(*recipient, val);
				previous = x.1.clone();
			} else {
				let sum_point = PointWithCurrencies {
					timestamp: x.0.clone().and_time(NaiveTime::from_num_seconds_from_midnight(0, 0)).unwrap(),
					value: add_maps(previous.value.clone(), x.1.value.clone()),
				};
				accumulated_raw_output.entry(*recipient)
					.or_default()
					.insert(x.0.clone(), sum_point.clone());
				previous = sum_point;
			}
		});
	});

	let mut output: BTreeMap<u32, Vec<Point>> = BTreeMap::new();
	accumulated_raw_output.into_iter().for_each(|x| {
		x.1.into_iter().for_each(|y| {
			let mut value: f64 = 0.0;
			let mut label = String::new();
			y.1.value.into_iter().for_each(|z| {
				let currency = currencies.iter().filter(|c| c.id.unwrap() == z.0).next().unwrap();

				let current_value = z.1 as f64 / currency.minor_in_mayor as f64;
				value += current_value;
				label.push_str(
					format!("{}{} ", current_value, currency.symbol).as_str()
				);
			});

			output.entry(x.0)
				.or_insert(Vec::new())
				.append(&mut vec![Point {
					timestamp: y.0.and_time(NaiveTime::from_num_seconds_from_midnight(0, 0)).unwrap(),
					value,
					label,
				}]);
		});
	});

	let mut named_output: BTreeMap<String, Vec<Point>> = BTreeMap::new();
	output.into_iter().for_each(|x| {
		let recipient = recipients.iter().filter(|r| r.id.unwrap() == x.0).next().unwrap();
		named_output.insert(recipient.name.clone(), x.1);
	});

	return Ok(named_output);
}

async fn compute_accounts(pool: &Pool, chart: Chart) -> Result<BTreeMap<String, Vec<Point>>, Box<dyn Error>> {
	let currencies = currency::get_all(&pool).await?;
	let transactions = get_relevant_time_sorted_transactions(&pool, &chart).await?;
	let accounts = account::get_all(&pool).await?;

	let mut raw_output: BTreeMap<u32, BTreeMap<Date<Utc>, PointWithoutCurrencies>> = BTreeMap::new();
	transactions.into_iter().for_each(|transaction| {
		raw_output.entry(transaction.account_id)
			.or_insert(BTreeMap::new())
			.entry(transaction.timestamp.date())
			.or_insert(PointWithoutCurrencies {
				timestamp: transaction.timestamp,
				value: 0,
			})
			.value += transaction.amount;
	});

	let mut accumulated_raw_output: BTreeMap<u32, BTreeMap<Date<Utc>, PointWithoutCurrencies>> = BTreeMap::new();
	raw_output.keys().for_each(|account| {
		accumulated_raw_output.insert(*account, BTreeMap::new());
		
		let mut previous = PointWithoutCurrencies {
			timestamp: chrono::MIN_DATETIME,
			value: 0,
		};
		raw_output.get(&account).unwrap().iter().for_each(|x| {

			let mut val: BTreeMap<Date<Utc>, PointWithoutCurrencies> = BTreeMap::new();
			val.insert(x.0.clone(), x.1.clone());
			if previous.timestamp == chrono::MIN_DATETIME {
				accumulated_raw_output.insert(*account, val);
				previous = x.1.clone();
			} else {
				let sum_point = PointWithoutCurrencies {
					timestamp: x.0.clone().and_time(NaiveTime::from_num_seconds_from_midnight(0, 0)).unwrap(),
					value: previous.value + x.1.value,
				};
				accumulated_raw_output.entry(*account)
					.or_default()
					.insert(x.0.clone(), sum_point.clone());
				previous = sum_point;
			}
		});
	});

	let mut output: BTreeMap<u32, Vec<Point>> = BTreeMap::new();
	accumulated_raw_output.into_iter().for_each(|x| {
		x.1.into_iter().for_each(|y| {
			let mut label = String::new();
			let account = accounts.iter().filter(|a| a.id.unwrap() == x.0).next().unwrap();
			let currency = currencies.iter().filter(|c| c.id.unwrap() == account.default_currency_id).next().unwrap();

			let value = y.1.value as f64 / currency.minor_in_mayor as f64;
			label.push_str(
				format!("{}{} ", value, currency.symbol).as_str()
			);

			output.entry(x.0)
				.or_insert(Vec::new())
				.append(&mut vec![Point {
					timestamp: y.0.and_time(NaiveTime::from_num_seconds_from_midnight(0, 0)).unwrap(),
					value,
					label,
				}]);
		});
	});

	let mut named_output: BTreeMap<String, Vec<Point>> = BTreeMap::new();
	output.into_iter().for_each(|x| {
		let account = accounts.iter().filter(|a| a.id.unwrap() == x.0).next().unwrap();
		named_output.insert(account.name.clone(), x.1);
	});
	return Ok(named_output);
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

fn add_maps(a: BTreeMap<u32, i32>, b: BTreeMap<u32, i32>) -> BTreeMap<u32, i32> {
	let mut a = a;
	b.iter().for_each(|x| {
		*a.entry(*x.0).or_insert(0) += x.1;
	});
	return a;
}