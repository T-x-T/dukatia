#[cfg(test)]
mod test;

use std::error::Error;
use std::collections::BTreeMap;
use deadpool_postgres::Pool;
use serde::Serialize;
use chrono::{DateTime, Date, NaiveTime, NaiveDate, Datelike, Utc, MIN_DATETIME, MAX_DATETIME, Duration};

use super::{Chart, ChartData};

use crate::CustomError;
use crate::transaction::{Transaction, TransactionLoader};
use crate::traits::*;
use crate::currency;
use crate::recipient;
use crate::account;
use crate::asset;

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
		"currencies" => compute_currencies(pool, chart).await?,
		"earning_spending_net" => compute_earning_spending_net(pool, chart).await?,
		"asset_total_value" => compute_asset_total_value(pool, chart).await?,
		"asset_single_value" => compute_asset_single_value(pool, chart).await?,
		"asset_amount" => compute_asset_amount(pool, chart).await?,
		_ => return Err(Box::new(CustomError::InvalidItem { reason: format!("Line chart collection {} is not recognized", chart.filter_collection.unwrap()) })),
	};

	return Ok(ChartData { text: None, pie: None, line: Some(output) });
}

async fn compute_recipients(pool: &Pool, chart: Chart) -> Result<Vec<(std::string::String, Vec<Point>)>, Box<dyn Error>> {
	let currencies = currency::CurrencyLoader::new(pool).get().await?;
	let transactions = get_relevant_time_sorted_transactions(pool, &chart, false).await?;
	let recipients = recipient::RecipientLoader::new(pool).get().await?;

	let raw_output = build_raw_output(transactions, RawOutputProperties::Recipient, &chart.date_period.unwrap_or("daily".to_string()));
	let accumulated_raw_output = accumulate(&raw_output);
	let output = sum_currencies(accumulated_raw_output, &currencies);
	let named_output = add_names_to_output(&output, &NamedTypes::Recipient(recipients));

	let sorted_output = sort_output(named_output);
	let limited_output = limit_output(sorted_output, chart.max_items);

	return Ok(limited_output);
}

async fn compute_accounts(pool: &Pool, chart: Chart) -> Result<Vec<(std::string::String, Vec<Point>)>, Box<dyn Error>> {
	let currencies = currency::CurrencyLoader::new(pool).get().await?;
	let transactions = get_relevant_time_sorted_transactions(pool, &chart, true).await?;
	let accounts = account::AccountLoader::new(pool).get().await?;
	let raw_output = build_raw_output(transactions, RawOutputProperties::Account, &chart.clone().date_period.unwrap_or("daily".to_string()));
	let accumulated_raw_output = accumulate(&raw_output);
	let raw_output_only_relevant_dates = limit_raw_output_dates(accumulated_raw_output, &chart);
	let output = sum_currencies(raw_output_only_relevant_dates, &currencies);
	let named_output = add_names_to_output(&output, &NamedTypes::Account(accounts));

	let sorted_output = sort_output(named_output);
	let limited_output = limit_output(sorted_output, chart.max_items);

	return Ok(limited_output);
}

async fn compute_currencies(pool: &Pool, chart: Chart) -> Result<Vec<(std::string::String, Vec<Point>)>, Box<dyn Error>> {
	let currencies = currency::CurrencyLoader::new(pool).get().await?;
	let transactions = get_relevant_time_sorted_transactions(pool, &chart, true).await?;

	let raw_output = build_raw_output(transactions, RawOutputProperties::Currency, &chart.clone().date_period.unwrap_or("daily".to_string()));
	let accumulated_raw_output = accumulate(&raw_output);
	let raw_output_only_relevant_dates = limit_raw_output_dates(accumulated_raw_output, &chart);
	let output = sum_currencies(raw_output_only_relevant_dates, &currencies);
	let named_output = add_names_to_output(&output, &NamedTypes::Currency(currencies));

	let sorted_output = sort_output(named_output);
	let limited_output = limit_output(sorted_output, chart.max_items);

	return Ok(limited_output);
}

async fn compute_earning_spending_net(pool: &Pool, chart: Chart) -> Result<Vec<(std::string::String, Vec<Point>)>, Box<dyn Error>> {
	let currencies = currency::CurrencyLoader::new(pool).get().await?;
	let transactions = get_relevant_time_sorted_transactions(pool, &chart, false).await?;

	let raw_output = build_raw_output(transactions, RawOutputProperties::EarningSpendingNet, &chart.date_period.unwrap_or("monthly".to_string()));
	let output = sum_currencies(raw_output, &currencies);
	let named_output = add_names_to_output(&output, &NamedTypes::EarningSpendingNet);

	return Ok(Vec::from_iter(named_output));
}

#[derive(Debug, Copy, Clone)]
enum RawOutputProperties {
	Recipient, Account, Currency, EarningSpendingNet
}

fn build_raw_output(transactions: Vec<Transaction>, property: RawOutputProperties, date_period: &str) -> BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>> {
	let mut output: BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>> = BTreeMap::new();
	for transaction in transactions {
		let id = match property {
			RawOutputProperties::Recipient => transaction.recipient_id,
			RawOutputProperties::Account => transaction.account_id,
			RawOutputProperties::Currency => transaction.currency_id.unwrap(),
			RawOutputProperties::EarningSpendingNet => {
				*output.entry(2)
					.or_default()
					.entry(get_date_for_period(date_period, &transaction.timestamp))
					.or_insert(PointWithCurrencies {
						timestamp: transaction.timestamp,
						value: BTreeMap::new(),
					})
					.value.entry(transaction.currency_id.unwrap())
					.or_insert(0) += transaction.total_amount.clone().unwrap_or_default().to_amount();
				
				if transaction.total_amount.clone().unwrap_or_default().to_amount() > 0 {
					0
				} else { 
					1
				}
			},
		};	
	
		*output.entry(id)
			.or_default()
			.entry(get_date_for_period(date_period, &transaction.timestamp))
			.or_insert(PointWithCurrencies {
				timestamp: transaction.timestamp,
				value: BTreeMap::new(),
			})
			.value.entry(transaction.currency_id.unwrap())
			.or_insert(0) += transaction.total_amount.unwrap_or_default().to_amount();
	}

	return output;
}

fn limit_raw_output_dates(input: BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>>, chart: &Chart) -> BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>> {
	let mut output: BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>> = BTreeMap::new();
	for mut x in input {
		x.1.retain(|k, _| chart.filter_from.unwrap_or(MIN_DATETIME).date().signed_duration_since(*k).num_seconds() <= 0 && chart.filter_to.unwrap_or(MAX_DATETIME).date().signed_duration_since(*k).num_seconds() >= 0);
		output.insert(x.0, x.1);
	}
	return output;
}

fn get_date_for_period(date_period: &str, timestamp: &DateTime<Utc>) -> Date<Utc> {
	match date_period {
		"daily" => {
			timestamp.date()
		},
		"monthly" => {
			Date::from_utc(NaiveDate::from_ymd_opt(timestamp.year(), timestamp.month(), 1).unwrap(), Utc)
		},
		"quarterly" => {
			match timestamp.month() {
				1..=3 => Date::from_utc(NaiveDate::from_ymd_opt(timestamp.year(), 1, 1).unwrap(), Utc),
				4..=6 => Date::from_utc(NaiveDate::from_ymd_opt(timestamp.year(), 4, 1).unwrap(), Utc),
				7..=9 => Date::from_utc(NaiveDate::from_ymd_opt(timestamp.year(), 7, 1).unwrap(), Utc),
				_ => Date::from_utc(NaiveDate::from_ymd_opt(timestamp.year(), 10, 1).unwrap(), Utc)
			}
		},
		"yearly" => {
			Date::from_utc(NaiveDate::from_ymd_opt(timestamp.year(), 1, 1).unwrap(), Utc)
		},
		_ => chrono::MIN_DATE,
	}
}

fn accumulate(input: &BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>>) -> BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>> {
	let mut output: BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>> = BTreeMap::new();
	for id in input.keys() {
		output.insert(*id, BTreeMap::new());
		
		let mut previous = PointWithCurrencies {
			timestamp: chrono::MIN_DATETIME,
			value: BTreeMap::new(),
		};
		input.get(id).unwrap().iter().for_each(|dated_point| {
			if previous.timestamp == chrono::MIN_DATETIME {
				let mut val: BTreeMap<Date<Utc>, PointWithCurrencies> = BTreeMap::new();
				val.insert(*dated_point.0, dated_point.1.clone());
				output.insert(*id, val);
				previous = dated_point.1.clone();
			} else {
				let sum_point = PointWithCurrencies {
					timestamp: dated_point.0.clone().and_time(NaiveTime::from_num_seconds_from_midnight(0, 0)).unwrap(),
					value: add_maps(previous.value.clone(), &dated_point.1.value),
				};
				output.entry(*id)
					.or_default()
					.insert(*dated_point.0, sum_point.clone());
				previous = sum_point;
			}
		});
	}

	return output;
}

fn sum_currencies(input: BTreeMap<u32, BTreeMap<Date<Utc>, PointWithCurrencies>>, currencies: &[currency::Currency]) -> BTreeMap<u32, Vec<Point>> {
	let mut output: BTreeMap<u32, Vec<Point>> = BTreeMap::new();
	for x in input {
		for y in x.1 {
			let mut value: f64 = 0.0;
			let mut label = String::new();
			y.1.value.into_iter().for_each(|z| {
				let currency = currencies.iter().find(|c| c.id.unwrap() == z.0).unwrap();
				let current_value = f64::from(z.1) / f64::from(currency.minor_in_major);
				value += current_value;
				label.push_str(
					format!("{}{} ", current_value, currency.symbol).as_str()
				);
			});
	
			output.entry(x.0)
				.or_default()
				.append(&mut vec![Point {
					timestamp: y.0.and_time(NaiveTime::from_num_seconds_from_midnight(0, 0)).unwrap(),
					value,
					label,
				}]
			);
		}
	}

	return output;
} 

#[derive(Debug, Clone)]
enum NamedTypes {
	Recipient(Vec<recipient::Recipient>),
	Account(Vec<account::Account>),
	Currency(Vec<currency::Currency>),
	EarningSpendingNet,
}

fn add_names_to_output(input: &BTreeMap<u32, Vec<Point>>, named_types: &NamedTypes) -> BTreeMap<String, Vec<Point>> {
	let mut output: BTreeMap<String, Vec<Point>> = BTreeMap::new();
	for x in input {
		match &named_types {
			NamedTypes::Recipient(recipients) => {
				let recipient = recipients.iter().find(|r| r.id.unwrap() == *x.0).unwrap();
				output.insert(recipient.name.clone(), x.1.clone());
			},
			NamedTypes::Account(accounts) => {
				let account = accounts.iter().find(|r| r.id.unwrap() == *x.0).unwrap();
				output.insert(account.name.clone(), x.1.clone());
			},
			NamedTypes::Currency(currencies) => {
				let currency = currencies.iter().find(|c| c.id.unwrap() == *x.0).unwrap();
				output.insert(currency.name.clone(), x.1.clone());
			},
			NamedTypes::EarningSpendingNet => {
				match x.0 {
					0 => output.insert(String::from("Earning"), x.1.clone()),
					1 => output.insert(String::from("Spending"), x.1.clone()),
					_ => output.insert(String::from("Net"), x.1.clone()),
				};
			},
		}
	}
	return output;
}

async fn get_relevant_time_sorted_transactions(pool: &Pool, chart: &Chart, get_all: bool) -> Result<Vec<Transaction>, Box<dyn Error>> {
	let from_date = if get_all {
		MIN_DATETIME
	} else {
		chart.filter_from.unwrap_or(MIN_DATETIME)
	};
	let to_date = if get_all {
		MAX_DATETIME
	} else {
		chart.filter_to.unwrap_or(MAX_DATETIME)
	};

	let mut transactions = TransactionLoader::new(pool).get().await?;
	transactions.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

	return Ok(transactions.into_iter()
		.filter(|x| {
		return from_date.signed_duration_since(x.timestamp).num_seconds() <= 0 
				&& to_date.signed_duration_since(x.timestamp).num_seconds() >= 0;
		})
		.collect());
}

fn add_maps(a: BTreeMap<u32, i32>, b: &BTreeMap<u32, i32>) -> BTreeMap<u32, i32> {
	let mut a = a;
	for x in b {
		*a.entry(*x.0).or_insert(0) += x.1;
	}
	return a;
}

fn sort_output(input: BTreeMap<String, Vec<Point>>) -> Vec<(std::string::String, Vec<Point>)> {
	let mut output = Vec::from_iter(input);
	output.sort_by(|a, b| b.1.last().unwrap().value.total_cmp(&a.1.last().unwrap().value));
	return output;
}

fn limit_output(input: Vec<(std::string::String, Vec<Point>)>, limit: Option<u32>) -> Vec<(std::string::String, Vec<Point>)> {
	let mut input = input;
	let mut output: Vec<(std::string::String, Vec<Point>)>;
	
	if limit.is_some() && input.len() > limit.unwrap() as usize {
		let top_limited_output: Vec<(std::string::String, Vec<Point>)> = input.clone().into_iter().take(limit.unwrap() as usize / 2).collect();
		input.reverse();
		let mut bottom_limited_output: Vec<(std::string::String, Vec<Point>)> = input.into_iter().take(limit.unwrap() as usize / 2).collect();
		output = top_limited_output;
		output.append(&mut bottom_limited_output);
	} else {
		output = input;
	}

	return output;
}

async fn compute_asset_total_value(pool: &Pool, chart: Chart) -> Result<Vec<(std::string::String, Vec<Point>)>, Box<dyn Error>> {
	if chart.asset_id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("asset_id"), item_type: String::from("chart") }));
	}
	
	let asset = asset::AssetLoader::new(pool).set_filter_id(chart.asset_id.unwrap(), NumberFilterModes::Exact).get_first().await?;
	let currency = currency::CurrencyLoader::new(pool).set_filter_id(asset.currency_id, NumberFilterModes::Exact).get_first().await?;
	let asset_valuation_history = asset::AssetValuationLoader::new(pool).set_filter_asset_id(chart.asset_id.unwrap(), NumberFilterModes::Exact).get().await?;
	let value_history: BTreeMap<DateTime<Utc>, u32> = asset_valuation_history.iter().map(|x| (x.timestamp, x.value_per_unit.to_amount() as u32)).collect();
	let amount_history: BTreeMap<DateTime<Utc>, f64> = asset_valuation_history.iter().map(|x| (x.timestamp, x.amount)).collect();
	
	let mut output: BTreeMap<String, Vec<Point>> = BTreeMap::new();
	output.insert(asset.name.clone(), Vec::new());

	if value_history.is_empty() || amount_history.is_empty() {
		return Ok(Vec::from_iter(output));
	}

	let first_day = get_first_day_of_asset_valuations(&value_history, &amount_history);
	let tomorrow: Date<Utc> = Utc::now().date().checked_add_signed(chrono::Duration::days(1)).unwrap();

	let mut last_value = f64::MIN;
	let mut current_day = first_day;
	while tomorrow.signed_duration_since(current_day).num_seconds() > 0 {
		let no_future_values: BTreeMap<&DateTime<Utc>, &u32> = value_history.iter().filter(|(x, _)| x.date().signed_duration_since(current_day).num_seconds() <= 0).collect();
		let no_future_amounts: BTreeMap<&DateTime<Utc>, &f64> = amount_history.iter().filter(|(x, _)| x.date().signed_duration_since(current_day).num_seconds() <= 0).collect();

		let value = (f64::from(**no_future_values.last_key_value().unwrap().1) * **no_future_amounts.last_key_value().unwrap().1) / f64::from(currency.minor_in_major);

		if (last_value - value).abs() > 0.0001 {
			let point = Point {
				timestamp: current_day.and_time(NaiveTime::from_num_seconds_from_midnight(0, 0)).unwrap(),
				value,
				label: format!("{}{}", value, currency.symbol),
			};
			output.entry(asset.name.clone()).or_default().push(point);
		}
		last_value = value;
		current_day += Duration::days(1);
	}

	return Ok(Vec::from_iter(output));
}

async fn compute_asset_single_value(pool: &Pool, chart: Chart) -> Result<Vec<(std::string::String, Vec<Point>)>, Box<dyn Error>> {
	if chart.asset_id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("asset_id"), item_type: String::from("chart") }));
	}
	
	let asset = asset::AssetLoader::new(pool).set_filter_id(chart.asset_id.unwrap(), NumberFilterModes::Exact).get_first().await?;
	let currency = currency::CurrencyLoader::new(pool).set_filter_id(asset.currency_id, NumberFilterModes::Exact).get_first().await?;
	let asset_valuation_history = asset::AssetValuationLoader::new(pool).set_filter_asset_id(chart.asset_id.unwrap(), NumberFilterModes::Exact).get().await?;
	let value_history: BTreeMap<DateTime<Utc>, u32> = asset_valuation_history.iter().map(|x| (x.timestamp, x.value_per_unit.to_amount() as u32)).collect();
	
	let mut output: BTreeMap<String, Vec<Point>> = BTreeMap::new();
	output.insert(asset.name.clone(), Vec::new());
	
	if value_history.is_empty() {
		return Ok(Vec::from_iter(output));
	}
	
	let first_day = get_first_day_of_asset_valuations(&value_history, &BTreeMap::new());
	let tomorrow: Date<Utc> = Utc::now().date().checked_add_signed(chrono::Duration::days(1)).unwrap();
	
	let mut last_value = f64::MIN;
	let mut current_day = first_day;
	while tomorrow.signed_duration_since(current_day).num_seconds() > 0 {
		let no_future_values: BTreeMap<&DateTime<Utc>, &u32> = value_history.iter().filter(|(x, _)| x.date().signed_duration_since(current_day).num_seconds() <= 0).collect();
		
		let value = f64::from(**no_future_values.last_key_value().unwrap().1) / f64::from(currency.minor_in_major);
		
		if (last_value - value).abs() > 0.0001 {
			let point = Point {
				timestamp: current_day.and_time(NaiveTime::from_num_seconds_from_midnight(0, 0)).unwrap(),
				value,
				label: format!("{}{}", value, currency.symbol),
			};
			output.entry(asset.name.clone()).or_default().push(point);
		}
		last_value = value;
		current_day += Duration::days(1);
	}

	return Ok(Vec::from_iter(output));
}

async fn compute_asset_amount(pool: &Pool, chart: Chart) -> Result<Vec<(std::string::String, Vec<Point>)>, Box<dyn Error>> {
	if chart.asset_id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("asset_id"), item_type: String::from("chart") }));
	}
	
	let asset = asset::AssetLoader::new(pool).set_filter_id(chart.asset_id.unwrap(), NumberFilterModes::Exact).get_first().await?;
	let asset_valuation_history = asset::AssetValuationLoader::new(pool).set_filter_asset_id(chart.asset_id.unwrap(), NumberFilterModes::Exact).get().await?;
	let amount_history: BTreeMap<DateTime<Utc>, f64> = asset_valuation_history.iter().map(|x| (x.timestamp, x.amount)).collect();

	let mut output: BTreeMap<String, Vec<Point>> = BTreeMap::new();
	output.insert(asset.name.clone(), Vec::new());

	if amount_history.is_empty() {
		return Ok(Vec::from_iter(output));
	}

	let first_day = get_first_day_of_asset_valuations(&BTreeMap::new(), &amount_history);
	let tomorrow: Date<Utc> = Utc::now().date().checked_add_signed(chrono::Duration::days(1)).unwrap();

	let mut last_value = f64::MIN;
	let mut current_day = first_day;
	while tomorrow.signed_duration_since(current_day).num_seconds() > 0 {
		let no_future_amounts: BTreeMap<&DateTime<Utc>, &f64> = amount_history.iter().filter(|(x, _)| x.date().signed_duration_since(current_day).num_seconds() <= 0).collect();

		let value = **no_future_amounts.last_key_value().unwrap().1;

		if (last_value - value).abs() > 0.0001 {
			let point = Point {
				timestamp: current_day.and_time(NaiveTime::from_num_seconds_from_midnight(0, 0)).unwrap(),
				value,
				label: value.to_string(),
			};
			output.entry(asset.name.clone()).or_default().push(point);
		}
		last_value = value;
		current_day += Duration::days(1);
	}

	return Ok(Vec::from_iter(output));
}

fn get_first_day_of_asset_valuations(value_history: &BTreeMap<DateTime<Utc>, u32>, amount_history: &BTreeMap<DateTime<Utc>, f64>) -> Date<Utc> {
	let mut first_day: Date<Utc> = Utc::now().date();
	if !value_history.is_empty() && value_history.first_key_value().unwrap().0.date().signed_duration_since(first_day).num_seconds() < 0 {
		first_day = value_history.first_key_value().unwrap().0.date();	
	}
	if !amount_history.is_empty() && amount_history.first_key_value().unwrap().0.date().signed_duration_since(first_day).num_seconds() < 0 {
		first_day = amount_history.first_key_value().unwrap().0.date();	
	}

	return first_day;
}