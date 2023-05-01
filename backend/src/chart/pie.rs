use std::error::Error;
use std::collections::BTreeMap;
use deadpool_postgres::Pool;
use chrono::{MIN_DATETIME, MAX_DATETIME};

use super::{Chart, ChartData};

use crate::CustomError;
use crate::transaction;
use crate::currency;
use crate::recipient;

pub async fn get_chart_data(pool: &Pool, chart: Chart) -> Result<ChartData, Box<dyn Error>> {
	if chart.filter_collection.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("filter_collection"), item_type: String::from("chart") }));
	}

	let output = match chart.filter_collection.as_ref().unwrap().as_str() {
		"recipients" => compute_recipients(pool, chart).await?,
		_ => return Err(Box::new(CustomError::InvalidItem { reason: format!("Pie chart collection {} is not recognized", chart.filter_collection.unwrap()) })),
	};

	return Ok(ChartData { text: None, pie: Some(output) });
}

async fn compute_recipients(pool: &Pool, chart: Chart) -> Result<BTreeMap<String, (String, i32)>, Box<dyn Error>> {
	let mut output: BTreeMap<String, (String, i32)> = BTreeMap::new();
	let currencies = currency::get_all(&pool).await?;
	let from_date = chart.filter_from.unwrap_or(MIN_DATETIME);
	let to_date = chart.filter_to.unwrap_or(MAX_DATETIME);

	let transactions: Vec<transaction::Transaction> = transaction::get_all(&pool).await?.into_iter().filter(|x| {
		return &from_date.signed_duration_since(x.timestamp).num_seconds() <= &0 
				&& &to_date.signed_duration_since(x.timestamp).num_seconds() >= &0;
		}).collect();

	let recipients = recipient::get_all(&pool).await?;

	recipients.into_iter().for_each(|recipient| {
		let mut amount_per_currency: BTreeMap<u32, i32> = BTreeMap::new();

		transactions.iter()
			.filter(|x| x.recipient_id == recipient.id.unwrap())
			.for_each(|transaction| {
				*amount_per_currency.entry(transaction.currency_id.unwrap()).or_insert(0) += transaction.amount
			});

		amount_per_currency.retain(|_, v| v > &mut 0);

		let mut amount: i32 = 0;
		let mut label = String::new();

		amount_per_currency.into_iter().for_each(|x| {
			let currency: currency::Currency = currencies.clone().into_iter().filter(|c| c.id.unwrap() == x.0).next().unwrap();
			amount += x.1 / currency.minor_in_mayor as i32;
			label.push_str(format!("{}{} ", x.1 / currency.minor_in_mayor as i32, currency.symbol).as_str());
		});

		output.insert(recipient.name, (label, amount));
	});

	output.retain(|_, v| v.1 > 0);

	return Ok(output)
}