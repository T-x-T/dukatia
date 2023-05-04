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

	let raw_output = build_raw_output(transactions, RawOutputProperties::Recipient);
	let accumulated_raw_output = accumulate(raw_output);
	let output = sum_currencies(accumulated_raw_output, currencies);
	let named_output = add_names_to_output(output, NamedTypes::Recipient(recipients));

	return Ok(named_output);
}

async fn compute_accounts(pool: &Pool, chart: Chart) -> Result<BTreeMap<String, Vec<Point>>, Box<dyn Error>> {
	let currencies = currency::get_all(&pool).await?;
	let transactions = get_relevant_time_sorted_transactions(&pool, &chart).await?;
	let accounts = account::get_all(&pool).await?;

	let raw_output = build_raw_output(transactions, RawOutputProperties::Account);
	let accumulated_raw_output = accumulate(raw_output);
	let output = sum_currencies(accumulated_raw_output, currencies);
	let named_output = add_names_to_output(output, NamedTypes::Account(accounts));

	return Ok(named_output);
}

enum RawOutputProperties {
	Recipient, Account	
}

fn build_raw_output(transactions: Vec<transaction::Transaction>, property: RawOutputProperties) -> BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>> {
	let mut output: BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>> = BTreeMap::new();
	transactions.into_iter().for_each(|transaction| {
		let id = match property {
			RawOutputProperties::Recipient => transaction.recipient_id,
			RawOutputProperties::Account => transaction.account_id,
		};	

		*output.entry(id)
			.or_insert(BTreeMap::new())
			.entry(transaction.timestamp.date())
			.or_insert(PointWithCurrencies {
				timestamp: transaction.timestamp,
				value: BTreeMap::new(),
			})
			.value.entry(transaction.currency_id.unwrap())
			.or_insert(0) += transaction.amount;
	});

	return output;
}

fn accumulate(input: BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>>) -> BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>> {
	let mut output: BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>> = BTreeMap::new();
	input.keys().for_each(|id| {
		output.insert(*id, BTreeMap::new());
		
		let mut previous = PointWithCurrencies {
			timestamp: chrono::MIN_DATETIME,
			value: BTreeMap::new(),
		};
		input.get(&id).unwrap().iter().for_each(|dated_point| {
			if previous.timestamp == chrono::MIN_DATETIME {
				let mut val: BTreeMap<Date<Utc>, PointWithCurrencies> = BTreeMap::new();
				val.insert(dated_point.0.clone(), dated_point.1.clone());
				output.insert(*id, val);
				previous = dated_point.1.clone();
			} else {
				let sum_point = PointWithCurrencies {
					timestamp: dated_point.0.clone().and_time(NaiveTime::from_num_seconds_from_midnight(0, 0)).unwrap(),
					value: add_maps(previous.value.clone(), dated_point.1.value.clone()),
				};
				output.entry(*id)
					.or_default()
					.insert(dated_point.0.clone(), sum_point.clone());
				previous = sum_point;
			}
		});
	});

	return output;
}

fn sum_currencies(input: BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>>, currencies: Vec<currency::Currency>) -> BTreeMap<u32, Vec<Point>> {
	let mut output: BTreeMap<u32, Vec<Point>> = BTreeMap::new();
	input.into_iter().for_each(|x| {
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

	return output;
} 

#[derive(Debug, Clone)]
enum NamedTypes {
	Recipient(Vec<recipient::Recipient>),
	Account(Vec<account::Account>),
}

fn add_names_to_output(input: BTreeMap<u32, Vec<Point>>, named_types: NamedTypes) -> BTreeMap<String, Vec<Point>> {
	let mut output: BTreeMap<String, Vec<Point>> = BTreeMap::new();
	input.iter().for_each(|x| {
		match &named_types {
			NamedTypes::Recipient(recipients) => {
				let recipient = recipients.iter().filter(|r| r.id.unwrap() == *x.0).next().unwrap();
				output.insert(recipient.name.clone(), x.1.to_vec());
			},
			NamedTypes::Account(accounts) => {
				let account = accounts.iter().filter(|r| r.id.unwrap() == *x.0).next().unwrap();
				output.insert(account.name.clone(), x.1.to_vec());
			},
		}
	});
	return output;
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