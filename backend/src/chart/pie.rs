use std::error::Error;
use std::collections::BTreeMap;
use deadpool_postgres::Pool;
use chrono::{MIN_DATETIME, MAX_DATETIME};

use super::{Chart, ChartData};

use crate::CustomError;
use crate::transaction;
use crate::currency;
use crate::recipient;
use crate::tag;

pub async fn get_chart_data(pool: &Pool, chart: Chart) -> Result<ChartData, Box<dyn Error>> {
	if chart.filter_collection.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("filter_collection"), item_type: String::from("chart") }));
	}

	let output = match chart.filter_collection.as_ref().unwrap().as_str() {
		"recipients" => compute_recipients(pool, chart).await?,
		"tags" => compute_tags(pool, chart).await?,
		_ => return Err(Box::new(CustomError::InvalidItem { reason: format!("Pie chart collection {} is not recognized", chart.filter_collection.unwrap()) })),
	};

	return Ok(ChartData { text: None, pie: Some(output), line: None });
}

async fn compute_recipients(pool: &Pool, chart: Chart) -> Result<BTreeMap<String, (String, f64)>, Box<dyn Error>> {
	let mut output: BTreeMap<String, (String, f64)> = BTreeMap::new();
	let currencies = currency::get_all(&pool).await?;
	let transactions = get_relevant_transactions(&pool, &chart).await?;
	let recipients = recipient::get_all(&pool).await?;

	recipients.into_iter().for_each(|recipient| {
		let mut amount_per_currency: BTreeMap<u32, i32> = BTreeMap::new();

		transactions.iter()
			.filter(|x| x.recipient_id == recipient.id.unwrap())
			.for_each(|transaction| {
				*amount_per_currency.entry(transaction.currency_id.unwrap()).or_insert(0) += transaction.amount
			});

		output.insert(recipient.name, build_label_amount(amount_per_currency, &currencies));
	});

	output.retain(|_, v| v.1 > 0.0);

	return Ok(output)
}

async fn compute_tags(pool: &Pool, chart: Chart) -> Result<BTreeMap<String, (String, f64)>, Box<dyn Error>> {
	let mut output: BTreeMap<String, (String, f64)> = BTreeMap::new();
	let currencies = currency::get_all(&pool).await?;
	let transactions = get_relevant_transactions(&pool, &chart).await?;
	let tags = tag::get_all(&pool).await?;

	tags.into_iter().for_each(|tag| {
		let mut amount_per_currency: BTreeMap<u32, i32> = BTreeMap::new();

		transactions.iter()
		.filter(|x| x.tag_ids.as_ref().unwrap().contains(&tag.id.unwrap()))
		.for_each(|transaction| {
			*amount_per_currency.entry(transaction.currency_id.unwrap()).or_insert(0) += transaction.amount
		});

		output.insert(tag.name, build_label_amount(amount_per_currency, &currencies));
	});

	output.retain(|_, v| v.1 > 0.0);

	return Ok(output);
}

async fn get_relevant_transactions(pool: &Pool, chart: &Chart) -> Result<Vec<transaction::Transaction>, Box<dyn Error>> {
	let from_date = chart.filter_from.unwrap_or(MIN_DATETIME);
	let to_date = chart.filter_to.unwrap_or(MAX_DATETIME);

	return Ok(transaction::get_all(&pool).await?.into_iter().filter(|x| {
		return &from_date.signed_duration_since(x.timestamp).num_seconds() <= &0 
				&& &to_date.signed_duration_since(x.timestamp).num_seconds() >= &0;
		}).collect());
}

fn build_label_amount(amount_per_currency: BTreeMap<u32, i32>, currencies: &Vec<currency::Currency>) -> (String, f64) {
	let mut amount_per_currency = amount_per_currency;

	amount_per_currency.retain(|_, v| v > &mut 0);

	let mut amount: f64 = 0.0;
	let mut label = String::new();

	amount_per_currency.into_iter().for_each(|x| {
		let currency: currency::Currency = currencies.clone().into_iter().filter(|c| c.id.unwrap() == x.0).next().unwrap();
		amount += x.1 as f64 / currency.minor_in_mayor as f64;
		label.push_str(format!("{}{} ", x.1 as f64 / currency.minor_in_mayor as f64, currency.symbol).as_str());
	});

	return (label, amount);
}