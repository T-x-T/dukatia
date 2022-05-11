pub mod rest_api;

use std::collections::BTreeMap;
use std::error::Error;
use deadpool_postgres::Pool;
use serde::Serialize;


use super::transaction;
use super::transaction::Transaction;
use super::currency;
use super::recipient;
use super::account;

#[derive(Debug, Serialize)]
pub struct TimestampedOutput {
	pub x: chrono::NaiveDate,
	pub y: i32
}

impl TimestampedOutput {
	fn from_data(data: BTreeMap<chrono::NaiveDate, i32>) -> Vec<Self> {
		let mut output: Vec<Self> = Vec::new();
		for i in 0..data.len() {
			output.push(TimestampedOutput { x: data.iter().nth(i).unwrap().0.clone(), y: data.iter().nth(i).unwrap().1.clone() });
		}
		return output;
	}
}

struct Map {
	data: BTreeMap<chrono::NaiveDate, i32>
}

impl Map {
	fn build(transactions: Vec<&Transaction>) -> Self {
		let mut data: BTreeMap<chrono::NaiveDate, i32> = BTreeMap::new();
	
		for transaction in transactions {
			if data.contains_key(&transaction.timestamp.date().naive_local()) {
				data.insert(transaction.timestamp.date().naive_local(), data.get(&transaction.timestamp.date().naive_local()).unwrap() + transaction.amount);
			} else {
				data.insert(transaction.timestamp.date().naive_local(), transaction.amount);
			}
		}
	
		return Self{data};
	}
	
	fn create_rolling_sum(mut self) -> Self {
		let mut prev: i32 = 0;
		for i in 0..self.data.len() {
			let cur_key = self.data.iter().nth(i).unwrap().0.clone();
			let cur_val = self.data.iter().nth(i).unwrap().1.clone();
			self.data.insert(cur_key, prev + cur_val);
			prev = self.data.iter().nth(i).unwrap().1.clone();
		}
		return self;
	}	
}

pub async fn balance_over_time_per_currency(
	pool: &Pool, from_date: Option<chrono::NaiveDate>,	to_date: Option<chrono::NaiveDate>
) -> Result<BTreeMap<u32, Vec<TimestampedOutput>>, Box<dyn Error>> {
	let transactions = get_transactions_timestamp_sorted(&pool).await?;

	return Ok(
		currency::get_all(&pool).await?.iter().map(|currency| {
			let data = Map::build(
				transactions.iter()
					.filter(|x| x.currency_id == currency.id)
					.collect()
			).create_rolling_sum();

			if from_date.is_some() && to_date.is_some() {
				return(currency.id.unwrap(), TimestampedOutput::from_data(retain_date_range(data.data, from_date.unwrap(), to_date.unwrap())));
			} else {
				return(currency.id.unwrap(), TimestampedOutput::from_data(data.data));
			}
	}).collect());
}

pub async fn balance_over_time_per_recipient(
	pool: &Pool, from_date: Option<chrono::NaiveDate>, to_date: Option<chrono::NaiveDate>
) -> Result<BTreeMap<u32, Vec<TimestampedOutput>>, Box<dyn Error>> {
	let transactions = get_transactions_timestamp_sorted(&pool).await?;

	return Ok(
		recipient::get_all(&pool).await?.iter().map(|recipient| {
			let data = Map::build(
				transactions.iter()
					.filter(|x| x.recipient_id == recipient.id.unwrap())
					.collect()
			).create_rolling_sum();

			if from_date.is_some() && to_date.is_some() {
				return(recipient.id.unwrap(), TimestampedOutput::from_data(retain_date_range(data.data, from_date.unwrap(), to_date.unwrap())));
			} else {
				return(recipient.id.unwrap(), TimestampedOutput::from_data(data.data));
			}
	}).collect());
}

pub async fn balance_over_time_per_account(
	pool: &Pool, from_date: Option<chrono::NaiveDate>, to_date: Option<chrono::NaiveDate>
) -> Result<BTreeMap<u32, Vec<TimestampedOutput>>, Box<dyn Error>> {
	let transactions = get_transactions_timestamp_sorted(&pool).await?;

	return Ok(
		account::get_all(&pool).await?.iter().map(|account| {
			let data = Map::build(
				transactions.iter()
					.filter(|x| x.account_id == account.id.unwrap())
					.collect()
			).create_rolling_sum();

			if from_date.is_some() && to_date.is_some() {
				return(account.id.unwrap(), TimestampedOutput::from_data(retain_date_range(data.data, from_date.unwrap(), to_date.unwrap())));
			} else {
				return(account.id.unwrap(), TimestampedOutput::from_data(data.data));
			}
	}).collect());
}

fn retain_date_range(mut data: BTreeMap<chrono::NaiveDate, i32>, from_date: chrono::NaiveDate, to_date: chrono::NaiveDate) -> BTreeMap<chrono::NaiveDate, i32> {
	return data.drain_filter(|k, _v| &from_date.signed_duration_since(*k).num_seconds() <= &0 && &to_date.signed_duration_since(*k).num_seconds() >= &0).collect();
}

async fn get_transactions_timestamp_sorted(pool: &Pool) -> Result<Vec<Transaction>, Box<dyn Error>> {
	let mut transactions = transaction::get_all(&pool).await?;
	transactions.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
	return Ok(transactions);
}

async fn total_per_currency(pool: &Pool) -> Result<BTreeMap<u32, i32>, Box<dyn Error>> {
	let mut output_map: BTreeMap<u32, i32> = BTreeMap::new();
	let transactions = transaction::get_all(&pool).await?;

	transactions.iter().for_each(|transaction| {
		let currency_id = transaction.currency_id.unwrap();
		if output_map.contains_key(&currency_id) {
			output_map.insert(currency_id, output_map.get(&currency_id).unwrap() + transaction.amount);
		} else {
			output_map.insert(currency_id, transaction.amount);
		}
	});

	return Ok(output_map);
}
