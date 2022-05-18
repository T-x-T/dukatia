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
use super::tag;
use super::tag::Tag;

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

pub async fn spending_per_recipient_in_date_range(
	pool: &Pool, from_date: chrono::NaiveDate, to_date: chrono::NaiveDate
) -> Result<BTreeMap<u32, BTreeMap<u32, i32>>, Box<dyn Error>> {
	let mut output_map: BTreeMap<u32, BTreeMap<u32, i32>> = BTreeMap::new();
	let transactions: Vec<Transaction> = transaction::get_all(&pool).await?.into_iter().filter(|x| {
		return &from_date.signed_duration_since(x.timestamp.naive_local().date()).num_seconds() <= &0 && &to_date.signed_duration_since(x.timestamp.naive_local().date()).num_seconds() >= &0;
	}).collect();
	
	let mut recipient_ids: Vec<u32> = transactions.iter().map(|x| x.recipient_id).collect();
	recipient_ids.sort();
	recipient_ids.dedup();
	recipient_ids.iter().for_each(|recipient_id| {
		let mut recipients_map: BTreeMap<u32, i32> = BTreeMap::new();
		transactions.iter().filter(|x| &x.recipient_id == recipient_id).for_each(|transaction| {
			let currency_id = transaction.currency_id.unwrap();
			if recipients_map.contains_key(&currency_id) {
				recipients_map.insert(currency_id, recipients_map.get(&currency_id).unwrap() + transaction.amount);
			} else {
				recipients_map.insert(currency_id, transaction.amount);
			}
		});
		recipients_map.retain(|_, v| v < &mut 0);
		recipients_map = recipients_map.iter().map(|(k, v)| (*k, v * -1)).collect();

		if recipients_map.len() > 0 {
			output_map.insert(*recipient_id, recipients_map);
		}
	});
	
	return Ok(output_map);
}

pub async fn spending_per_tag_in_date_range(
	pool: &Pool, from_date: chrono::NaiveDate, to_date: chrono::NaiveDate, only_parents: bool
) -> Result<BTreeMap<u32, BTreeMap<u32, i32>>, Box<dyn Error>> {
	let mut output_map: BTreeMap<u32, BTreeMap<u32, i32>> = BTreeMap::new();
	let transactions: Vec<Transaction> = transaction::get_all(&pool).await?.into_iter().filter(|x| {
		return &from_date.signed_duration_since(x.timestamp.naive_local().date()).num_seconds() <= &0 && &to_date.signed_duration_since(x.timestamp.naive_local().date()).num_seconds() >= &0;
	}).collect();
	
	let mut tag_ids: Vec<u32> = transactions.clone().into_iter().map(|x| x.tag_ids.unwrap()).flatten().collect();
	tag_ids.sort();
	tag_ids.dedup();
	tag_ids.iter().for_each(|tag_id| {
		let mut tags_map: BTreeMap<u32, i32> = BTreeMap::new();
		transactions.iter().filter(|x| x.tag_ids.as_ref().unwrap().contains(tag_id)).for_each(|transaction| {
			let currency_id = transaction.currency_id.unwrap();
			if tags_map.contains_key(&currency_id) {
				tags_map.insert(currency_id, tags_map.get(&currency_id).unwrap() + transaction.amount);
			} else {
				tags_map.insert(currency_id, transaction.amount);
			}
		});
		tags_map.retain(|_, v| v < &mut 0);
		tags_map = tags_map.iter().map(|(k, v)| (*k, v * -1)).collect();

		if tags_map.len() > 0 {
			output_map.insert(*tag_id, tags_map);
		}
	});

	if only_parents {
		let mut temp_output_map = output_map;
		output_map = BTreeMap::new();

		for (tag_id, value) in temp_output_map.clone().iter() {
			let highest_parent = get_highest_parent_of_tag(*tag_id, tag::get_all(&pool).await.unwrap());

			if output_map.contains_key(&highest_parent) {
				let mut new_value = temp_output_map.get(&highest_parent).unwrap().clone();
				for (currency_id, amount) in value.iter() {
					if new_value.contains_key(&currency_id) {
						new_value.insert(*currency_id, new_value.get(&currency_id).unwrap() + amount);
					} else {
						new_value.insert(*currency_id, *amount);
					}
				}
				output_map.insert(highest_parent, new_value.clone());
				temp_output_map.insert(highest_parent, new_value);
			} else {
				output_map.insert(highest_parent, value.clone());
				temp_output_map.insert(highest_parent, value.clone());
			}
		}
	}

	return Ok(output_map);
}

fn get_highest_parent_of_tag(tag_id: u32, tags: Vec<Tag>) -> u32 {
	let tag = tags.iter().find(|&tag| tag.id.unwrap() == tag_id);
	if tag.is_some() && tag.unwrap().parent_id.is_none() {
		return tag_id;
	} else {
		return get_highest_parent_of_tag(tag.unwrap().parent_id.unwrap(), tags);
	}
}