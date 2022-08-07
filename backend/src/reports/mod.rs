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

type CurrencyId = u32;
type RecipientId = u32;
type AccountId = u32;
type TagId = u32;


#[derive(Debug, Serialize)]
pub struct RankedData {
	data: BTreeMap<u32, i32>,
	rank: u32,
}

#[derive(Debug, Serialize)]
pub struct TimeseriesRankedData {
	data: Vec<TimestampedOutput>,
	rank: u32,
}

#[derive(Debug, Serialize, Clone)]
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

struct Timeseries {
	data: BTreeMap<chrono::NaiveDate, i32>
}

impl Timeseries {
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

fn retain_date_range(mut data: BTreeMap<chrono::NaiveDate, i32>, from_date: chrono::NaiveDate, to_date: chrono::NaiveDate) -> BTreeMap<chrono::NaiveDate, i32> {
	return data.drain_filter(|k, _v| &from_date.signed_duration_since(*k).num_seconds() <= &0 && &to_date.signed_duration_since(*k).num_seconds() >= &0).collect();
}

async fn get_transactions_timestamp_sorted(pool: &Pool) -> Result<Vec<Transaction>, Box<dyn Error>> {
	let mut transactions = transaction::get_all(&pool).await?;
	transactions.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
	return Ok(transactions);
}

fn add_ranks_timeseries(input: BTreeMap<u32, Vec<TimestampedOutput>>)  -> BTreeMap<u32, TimeseriesRankedData> {
	let mut last_values: Vec<(u32, i32)> = input.iter().map(|(i, v)| (*i, v.last().unwrap().y)).collect();
	last_values.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
	
	let mut output_map: BTreeMap<u32, TimeseriesRankedData> = BTreeMap::new();

	let mut counter = 0;
	for(i, _) in last_values.into_iter() {
		output_map.insert(i, TimeseriesRankedData{data: input.get(&i).expect("This should never happen").clone(), rank: counter});
		counter += 1;
	}

	return output_map;
}

fn get_highest_parent_of_tag(tag_id: u32, tags: &Vec<Tag>) -> u32 {
	let tag = tags.iter().find(|&tag| tag.id.unwrap() == tag_id);
	if tag.is_some() && tag.unwrap().parent_id.is_none() {
		return tag_id;
	} else {
		return get_highest_parent_of_tag(tag.unwrap().parent_id.unwrap(), tags);
	}
}

fn add_ranks(input: BTreeMap<u32, BTreeMap<u32, i32>>) -> BTreeMap<u32, RankedData> {
	let mut flat_output_map = flatten_map(&input);
	flat_output_map.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

	let mut output_map: BTreeMap<u32, RankedData> = BTreeMap::new();
	let mut counter = 0;
	for (i, _) in flat_output_map.into_iter() {
		output_map.insert(*i, RankedData{data: input.get(i).expect("This should never happen").clone(), rank: counter});
		counter += 1;
	};

	return output_map;
}

fn flatten_map(map: &BTreeMap<u32, BTreeMap<u32, i32>>) -> Vec<(&u32, i32)> {
	return Vec::from_iter(map).iter()
		.map(|(y, x)| (*y, x.clone().clone().into_values().sum::<i32>()))
		.collect();
}

async fn get_transactions_between_dates(
	pool: &Pool, from_date: chrono::NaiveDate, to_date: chrono::NaiveDate
) -> Result<Vec<Transaction>, Box<dyn Error>> {
	return Ok(
		transaction::get_all(&pool).await?.into_iter().filter(|x| {
			return &from_date.signed_duration_since(x.timestamp.naive_local().date()).num_seconds() <= &0 
					&& &to_date.signed_duration_since(x.timestamp.naive_local().date()).num_seconds() >= &0;
		}).collect()
	);
}





pub async fn balance_over_time_per_currency(
	pool: &Pool, from_date: Option<chrono::NaiveDate>,	to_date: Option<chrono::NaiveDate>
) -> Result<BTreeMap<CurrencyId, TimeseriesRankedData>, Box<dyn Error>> {
	let transactions = get_transactions_timestamp_sorted(&pool).await?;

	let mut timeseries_output: BTreeMap<CurrencyId, Vec<TimestampedOutput>> = currency::get_all(&pool).await?.iter().map(|currency| {
		let timeseries = Timeseries::build(
			transactions.iter()
				.filter(|x| x.currency_id == currency.id)
				.collect()
		).create_rolling_sum();

		if from_date.is_some() && to_date.is_some() {
			return(currency.id.unwrap(), TimestampedOutput::from_data(retain_date_range(timeseries.data, from_date.unwrap(), to_date.unwrap())));
		} else {
			return(currency.id.unwrap(), TimestampedOutput::from_data(timeseries.data));
		}
	}).collect();
	timeseries_output.drain_filter(|_, v| v.len() == 0);

	return Ok(add_ranks_timeseries(timeseries_output));
}

pub async fn balance_over_time_per_recipient(
	pool: &Pool, from_date: Option<chrono::NaiveDate>, to_date: Option<chrono::NaiveDate>
) -> Result<BTreeMap<RecipientId, TimeseriesRankedData>, Box<dyn Error>> {
	let transactions = get_transactions_timestamp_sorted(&pool).await?;

	let mut timeseries_output: BTreeMap<RecipientId, Vec<TimestampedOutput>> = recipient::get_all(&pool).await?.iter().map(|recipient| {
		let data = Timeseries::build(
			transactions.iter()
				.filter(|x| x.recipient_id == recipient.id.unwrap())
				.collect()
		).create_rolling_sum();

		if from_date.is_some() && to_date.is_some() {
			return(recipient.id.unwrap(), TimestampedOutput::from_data(retain_date_range(data.data, from_date.unwrap(), to_date.unwrap())));
		} else {
			return(recipient.id.unwrap(), TimestampedOutput::from_data(data.data));
		}
	}).collect();
	timeseries_output.drain_filter(|_, v| v.len() == 0);

	return Ok(add_ranks_timeseries(timeseries_output));
}

pub async fn balance_over_time_per_account(
	pool: &Pool, from_date: Option<chrono::NaiveDate>, to_date: Option<chrono::NaiveDate>
) -> Result<BTreeMap<AccountId, TimeseriesRankedData>, Box<dyn Error>> {
	let transactions = get_transactions_timestamp_sorted(&pool).await?;

	let mut timeseries_output: BTreeMap<AccountId, Vec<TimestampedOutput>> = account::get_all(&pool).await?.iter().map(|account| {
		let data = Timeseries::build(
			transactions.iter()
				.filter(|x| x.account_id == account.id.unwrap())
				.collect()
		).create_rolling_sum();

		if from_date.is_some() && to_date.is_some() {
			return(account.id.unwrap(), TimestampedOutput::from_data(retain_date_range(data.data, from_date.unwrap(), to_date.unwrap())));
		} else {
			return(account.id.unwrap(), TimestampedOutput::from_data(data.data));
		}
	}).collect();
	timeseries_output.drain_filter(|_, v| v.len() == 0);

	return Ok(add_ranks_timeseries(timeseries_output));
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
) -> Result<BTreeMap<RecipientId, RankedData>, Box<dyn Error>> {
	let mut output: BTreeMap<RecipientId, BTreeMap<CurrencyId, i32>> = BTreeMap::new();
	let transactions = get_transactions_between_dates(pool, from_date, to_date).await?;
	
	let mut recipient_ids: Vec<RecipientId> = transactions.iter().map(|x| x.recipient_id).collect();
	recipient_ids.sort();
	recipient_ids.dedup();

	recipient_ids.iter().for_each(|recipient_id| {
		let mut currencies_with_money: BTreeMap<CurrencyId, i32> = BTreeMap::new();
		
		transactions.iter()
			.filter(|transaction| &transaction.recipient_id == recipient_id)
			.for_each(|transaction| 
				*currencies_with_money.entry(transaction.currency_id.unwrap()).or_insert(0) += transaction.amount)
			;

		currencies_with_money.retain(|_, v| v < &mut 0);
		currencies_with_money = currencies_with_money.iter().map(|(k, v)| (*k, v * -1)).collect();

		if currencies_with_money.len() > 0 {
			output.insert(*recipient_id, currencies_with_money);
		}
	});

	return Ok(add_ranks(output));
}

pub async fn spending_per_tag_in_date_range(
	pool: &Pool, from_date: chrono::NaiveDate, to_date: chrono::NaiveDate, only_parents: bool
) -> Result<BTreeMap<TagId, RankedData>, Box<dyn Error>> {
	let mut output: BTreeMap<TagId, BTreeMap<CurrencyId, i32>> = BTreeMap::new();
	let transactions = get_transactions_between_dates(pool, from_date, to_date).await?;
	
	let mut tag_ids: Vec<TagId> = transactions.clone().into_iter().map(|x| x.tag_ids.unwrap()).flatten().collect();
	tag_ids.sort();
	tag_ids.dedup();

	tag_ids.iter().for_each(|tag_id| {
		let mut currencies_with_money: BTreeMap<CurrencyId, i32> = BTreeMap::new();

		transactions.iter()
			.filter(|transaction| transaction.tag_ids.as_ref().unwrap().contains(tag_id))
			.for_each(|transaction| 
				*currencies_with_money.entry(transaction.currency_id.unwrap()).or_insert(0) += transaction.amount)
			;

		currencies_with_money.retain(|_, v| v < &mut 0);
		currencies_with_money = currencies_with_money.iter().map(|(k, v)| (*k, v * -1)).collect();

		if currencies_with_money.len() > 0 {
			output.insert(*tag_id, currencies_with_money);
		}
	});

	if only_parents {
		let mut output_with_parents = output;
		output = BTreeMap::new();
		let tags = tag::get_all(&pool).await?;

		output_with_parents.clone().iter().for_each(|(tag_id, currencies_with_money)| {
			let highest_parent = get_highest_parent_of_tag(*tag_id, &tags);
	
			if output.contains_key(&highest_parent) {
				let mut new_currencies_with_money = output_with_parents.get(&highest_parent).unwrap().clone();
				
				currencies_with_money.iter().for_each(|(currency_id, amount)|
					*new_currencies_with_money.entry(*currency_id).or_insert(0) += amount
				);

				output.insert(highest_parent, new_currencies_with_money.clone());
				output_with_parents.insert(highest_parent, new_currencies_with_money);
			} else {
				output.insert(highest_parent, currencies_with_money.clone());
				output_with_parents.insert(highest_parent, currencies_with_money.clone());
			}
		});
	}

	return Ok(add_ranks(output));
}