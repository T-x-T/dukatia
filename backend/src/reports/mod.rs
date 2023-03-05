pub mod rest_api;
mod timeseries;

use std::collections::BTreeMap;
use std::error::Error;
use deadpool_postgres::Pool;
use serde::Serialize;
use chrono::{Utc, DateTime, NaiveDate, Duration, Date};

use timeseries::*;
use super::transaction;
use super::transaction::Transaction;
use super::recipient;
use super::account;
use super::tag;
use super::tag::Tag;
use super::asset;

type CurrencyId = u32;
type RecipientId = u32;
type AccountId = u32;
type TagId = u32;
type AssetId = u32;


#[derive(serde::Deserialize, Copy, Clone)]
pub enum Period {
	Monthly,
	Quarterly,
	Yearly,
}

#[derive(Debug, Serialize, Clone)]
pub struct RankedData {
	data: BTreeMap<u32, i32>,
	rank: u32,
}

#[derive(Debug, Serialize, Clone)]
pub struct TimeseriesRankedData {
	data: Vec<TimestampedOutput>,
	rank: u32,
}

#[derive(Debug, Serialize, Clone)]
pub struct TimestampedOutput {
	pub x: NaiveDate,
	pub y: i32
}

impl TimestampedOutput {
	fn from_data(data: BTreeMap<NaiveDate, i32>) -> Vec<Self> {
		let mut output: Vec<Self> = Vec::new();
		for i in 0..data.len() {
			output.push(TimestampedOutput { x: data.iter().nth(i).unwrap().0.clone(), y: data.iter().nth(i).unwrap().1.clone() });
		}

		return output;
	}
}

fn retain_date_range(mut data: BTreeMap<NaiveDate, i32>, from_date: NaiveDate, to_date: NaiveDate) -> BTreeMap<NaiveDate, i32> {
	return data.drain_filter(|k, _v| &from_date.signed_duration_since(*k).num_seconds() <= &0 && &to_date.signed_duration_since(*k).num_seconds() >= &0).collect();
}

async fn get_transactions_timestamp_sorted(pool: &Pool) -> Result<Vec<Transaction>, Box<dyn Error>> {
	let mut transactions = transaction::get_all(&pool).await?;
	transactions.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
	return Ok(transactions);
}

fn add_ranks_timeseries(input: BTreeMap<u32, Vec<TimestampedOutput>>) -> BTreeMap<u32, TimeseriesRankedData> {
	let mut last_values: Vec<(u32, i32)> = input.iter().map(|(i, v)| (*i, v.last().unwrap().y)).collect();
	last_values.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
	
	let mut output_map: BTreeMap<u32, TimeseriesRankedData> = BTreeMap::new();

	let mut counter = 0;
	for(i, _) in last_values.into_iter() {
		output_map.insert(i, TimeseriesRankedData{data: input.get(&i).expect("This should never happen (maybe get some ECC memory)").clone(), rank: counter});
		counter += 1;
	}

	return output_map;
}

fn limit_results_timeseries(input: BTreeMap<u32, TimeseriesRankedData>, top_entries: u32, bottom_entries: u32) -> BTreeMap<u32, TimeseriesRankedData> {
	return input.clone().drain_filter(|_, v| {
		v.rank < top_entries || v.rank >= input.len() as u32 - bottom_entries
	}).collect();
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

fn limit_results(input: BTreeMap<u32, RankedData>, max_entries: u32) -> BTreeMap<u32, RankedData> {
	let mut output: BTreeMap<u32, RankedData> = BTreeMap::new();

	for (k, v) in input.into_iter() {
		if v.rank < max_entries {
			output.insert(k, v);
		} else {
			if output.contains_key(&u32::MAX) {
				
				let mut new_data: BTreeMap<CurrencyId, i32> = output.get(&u32::MAX).unwrap().data.clone();
				
				for (k2, v2) in v.data.into_iter() {
					*new_data.entry(k2).or_insert(0) += v2;
				}

				output.insert(u32::MAX, RankedData{data: new_data, rank: max_entries});
			} else {
				output.insert(u32::MAX, RankedData{data: v.data, rank: max_entries});
			}
		}
	}

	return output;
}

fn flatten_map(map: &BTreeMap<u32, BTreeMap<u32, i32>>) -> Vec<(&u32, i32)> {
	return Vec::from_iter(map).iter()
		.map(|(y, x)| (*y, x.clone().clone().into_values().sum::<i32>()))
		.collect();
}

async fn get_transactions_between_dates(
	pool: &Pool, from_date: NaiveDate, to_date: NaiveDate
) -> Result<Vec<Transaction>, Box<dyn Error>> {
	return Ok(
		transaction::get_all(&pool).await?.into_iter().filter(|x| {
			return &from_date.signed_duration_since(x.timestamp.naive_local().date()).num_seconds() <= &0 
					&& &to_date.signed_duration_since(x.timestamp.naive_local().date()).num_seconds() >= &0;
		}).collect()
	);
}

pub async fn balance_over_time_per_currency(
	pool: &Pool, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>
) -> Result<BTreeMap<CurrencyId, TimeseriesRankedData>, Box<dyn Error>> {
	let mut output: BTreeMap<CurrencyId, Vec<TimestampedOutput>> = BTreeMap::new();

	let transactions = get_transactions_timestamp_sorted(&pool).await?;
	let assets = asset::get_all_from_user(pool, 0).await.unwrap(); //TODO: get only data from correct user

	let mut asset_valuations: BTreeMap<u32, Vec<asset::AssetValuation>> = BTreeMap::new();

	for asset in assets.iter() {
		let mut asset_valuation = asset::get_valuation_history_by_asset_id(pool, asset.id.unwrap()).await?;
		asset_valuation.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
		asset_valuations.insert(asset.id.unwrap(), asset_valuation);
	}

	let first_day: Date<Utc> = match from_date {
		Some(x) => {
			if x >= transactions.get(0).unwrap().timestamp.date().naive_utc() { //TODO: This will panic when no transaction are found
				Date::from_utc(x, Utc)
			} else {
				transactions.get(0).unwrap().timestamp.date() //TODO: This will panic when no transaction are found
			}
		},
		None => transactions.get(0).unwrap().timestamp.date(), //TODO: This will panic when no transaction are found
	};

	let tomorrow = Utc::now().date().checked_add_signed(Duration::days(1)).unwrap();
	let last_day: Date<Utc> = match to_date {
		Some(x) => {
			if x <= tomorrow.naive_utc() {
				Date::from_utc(x.checked_add_signed(Duration::days(1)).unwrap(), Utc)
			} else {
				tomorrow
			}
		},
		None => tomorrow,
	};

	let mut current_day = first_day;

	let mut todays_output: BTreeMap<CurrencyId, i32> = BTreeMap::new();

	while last_day.signed_duration_since(current_day).num_seconds() > 0 {
		for asset in assets.iter() {
			let current_days_asset_valuation = asset_valuations.get(&asset.id.unwrap()).unwrap().clone().drain_filter(|x| x.timestamp.date() <= current_day).last();
			let current_days_value_of_asset: i32 = match current_days_asset_valuation {
				Some(x) => (x.amount * x.value_per_unit as f64).round() as i32,
				None => 0,
			};
			
			if !todays_output.contains_key(&asset.currency_id) {
				todays_output.insert(asset.currency_id, current_days_value_of_asset);
			} else {
				todays_output.insert(asset.currency_id, todays_output.get(&asset.currency_id).unwrap() + current_days_value_of_asset);
			}
		}

		for transaction in transactions.iter() {
			if transaction.timestamp.date() > current_day {
				continue;
			}

			if !todays_output.contains_key(&transaction.currency_id.unwrap()) {
				todays_output.insert(transaction.currency_id.unwrap(), transaction.amount);
			} else {
				todays_output.insert(transaction.currency_id.unwrap(), todays_output.get(&transaction.currency_id.unwrap()).unwrap() + transaction.amount);
			}
		}

		todays_output.into_iter().for_each(|(currency_id, value)| {
			if !output.contains_key(&currency_id) {
				output.insert(currency_id, vec![TimestampedOutput {x: current_day.naive_utc(), y: value}]);
			} else {
				let mut new_timestamped_output = output.get(&currency_id).unwrap().clone();
				new_timestamped_output.push(TimestampedOutput {x: current_day.naive_utc(), y: value});
				output.insert(currency_id, new_timestamped_output);
			}
		});

		todays_output = BTreeMap::new();
		current_day = current_day + Duration::days(1);
	}

	return Ok(limit_results_timeseries(add_ranks_timeseries(output), 3, 3));
}

pub async fn balance_over_time_per_recipient(
	pool: &Pool, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>
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

	return Ok(limit_results_timeseries(add_ranks_timeseries(timeseries_output), 3, 3));
}

pub async fn balance_over_time_per_account(
	pool: &Pool, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>
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

	return Ok(limit_results_timeseries(add_ranks_timeseries(timeseries_output), 3, 3));
}

pub async fn balance_over_time(
	pool: &Pool, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>, period: Period
) -> Result<BTreeMap<u32, TimeseriesRankedData>, Box<dyn Error>> {
	let transactions = get_transactions_timestamp_sorted(&pool).await?;

	let mut timeseries_output: BTreeMap<u32, Vec<TimestampedOutput>> = vec![0, 1, 2].into_iter().map(|i| { //0 = Earning, 1 = Spending, 2 = Net
		let timeseries = Timeseries::build(
			transactions.iter()
				.filter(|&x| {
					match i {
						0 => x.amount > 0,
						1 => x.amount < 0,
						_ => x.amount != 0,
					}
				})
				.collect()
		);

		let data = match period {
			Period::Monthly => timeseries.create_sum_aggregate_monthly(),
			Period::Quarterly => timeseries.create_sum_aggregate_quarterly(),
			Period::Yearly => timeseries.create_sum_aggregate_yearly(),
		};

		if from_date.is_some() && to_date.is_some() {
			return(i, TimestampedOutput::from_data(retain_date_range(data.data, from_date.unwrap(), to_date.unwrap())));
		} else {
			return(i, TimestampedOutput::from_data(data.data));
		}
	}).collect();
	timeseries_output.drain_filter(|_, v| v.len() == 0);
	
	return Ok(limit_results_timeseries(add_ranks_timeseries(timeseries_output), 3, 3));
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
	pool: &Pool, from_date: NaiveDate, to_date: NaiveDate
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

	return Ok(limit_results(add_ranks(output), 5));
}

pub async fn spending_per_tag_in_date_range(
	pool: &Pool, from_date: NaiveDate, to_date: NaiveDate, only_parents: bool
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

	return Ok(limit_results(add_ranks(output), 5));
}

pub async fn daily_valuation_of_asset(pool: &Pool, asset_id: AssetId) 
-> Result<BTreeMap<NaiveDate, (u32, f64)>, Box<dyn Error>> {
	let mut output: BTreeMap<NaiveDate, (u32, f64)> = BTreeMap::new();

	let value_history = asset::get_value_per_unit_history(&pool, asset_id).await?;
	let amount_history = asset::get_amount_history(&pool, asset_id).await?;

	if value_history.len() == 0 || amount_history.len() == 0 {
		return Ok(output);
	}

	let mut first_day: NaiveDate = Utc::now().date().naive_utc();
	if value_history.first_key_value().unwrap().0.date().naive_utc().signed_duration_since(first_day).num_seconds() < 0 {
		first_day = value_history.first_key_value().unwrap().0.date().naive_utc();	
	}
	if amount_history.first_key_value().unwrap().0.date().naive_utc().signed_duration_since(first_day).num_seconds() < 0 {
		first_day = amount_history.first_key_value().unwrap().0.date().naive_utc();	
	}

	let tomorrow: NaiveDate = Utc::now().date().naive_utc().checked_add_signed(chrono::Duration::days(1)).unwrap();

	let mut current_day = first_day;
	while tomorrow.signed_duration_since(current_day).num_seconds() > 0 {
		let no_future_values: BTreeMap<&DateTime<Utc>, &u32> = value_history.iter().filter(|(x, _)| x.date().naive_utc().signed_duration_since(current_day).num_seconds() <= 0).collect();
		let no_future_amounts: BTreeMap<&DateTime<Utc>, &f64> = amount_history.iter().filter(|(x, _)| x.date().naive_utc().signed_duration_since(current_day).num_seconds() <= 0).collect();

		output.insert(current_day, (no_future_values.last_key_value().unwrap().1.clone().clone(), no_future_amounts.last_key_value().unwrap().1.clone().clone()));
		current_day = current_day + Duration::days(1);
	}

	return Ok(output);
}