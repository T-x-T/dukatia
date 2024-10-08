#[cfg(test)]
mod test;

use deadpool_postgres::Pool;
use std::error::Error;
use std::collections::BTreeMap;
use chrono::prelude::*;
use uuid::Uuid;

use crate::chart::{Dataset, IntermediateChartData, DataPointMonetary, DataPoint, ChartOptions, get_relevant_time_sorted_transactions};
use super::{CurrencyLoader, Currency};
use crate::money::Money;
use crate::transaction::Transaction;
use crate::traits::*;

pub async fn get_per_currency_over_time(pool: &Pool, options: ChartOptions) -> Result<IntermediateChartData, Box<dyn Error>> {
	let transactions = get_relevant_time_sorted_transactions(pool, &options, false).await?;
	let currencies = CurrencyLoader::new(pool).get().await?;

	return Ok(calculate_get_per_currency_over_time(&options, transactions, &currencies));
}


fn calculate_get_per_currency_over_time(options: &ChartOptions, transactions: Vec<Transaction>, currencies: &[Currency]) -> IntermediateChartData {
	let mut output = IntermediateChartData::default();
	let mut datasets_monetary: BTreeMap<Uuid, Vec<DataPointMonetary>> = BTreeMap::new();

	let default = DataPointMonetary::default();
	for transaction in transactions {
		let mut data_point = datasets_monetary.entry(transaction.currency_id.unwrap()).or_default().last().unwrap_or(&default).clone();
		let transaction_total_amount = transaction.total_amount.unwrap();
		data_point.value = Money::from_amount(data_point.value.to_amount() + transaction_total_amount.to_amount(), transaction_total_amount.get_minor_in_major(), transaction_total_amount.get_symbol());

		let timestamp: NaiveDate = options.date_period.unwrap_or_default().get_date_at_timestamp(transaction.timestamp.date_naive());

		if timestamp == data_point.timestamp.unwrap_or_default() {
			datasets_monetary.entry(transaction.currency_id.unwrap()).or_default().last_mut().unwrap().label = data_point.value.to_string();
			datasets_monetary.entry(transaction.currency_id.unwrap()).or_default().last_mut().unwrap().value = data_point.value;
		} else {
			datasets_monetary.entry(transaction.currency_id.unwrap()).or_default().push(
				DataPointMonetary { 
					name: None,
					timestamp: Some(timestamp),
					value: data_point.value.clone(),
					label: data_point.value.to_string(),
				}
			);
		}
	}

	let mut datasets: BTreeMap<Uuid, Vec<DataPoint>> = BTreeMap::new();

	for dataset in datasets_monetary {
		for data_point in dataset.1 {
			datasets.entry(dataset.0).or_default().push(
				DataPoint { 
					name: data_point.name,
					timestamp: data_point.timestamp,
					value: f64::from(data_point.value.to_amount()) / f64::from(data_point.value.get_minor_in_major()),
					label: data_point.label,
				}
			);
		}
	}

	output.datasets = BTreeMap::new();
	for dataset in datasets {
		let name: String = currencies.iter()
			.filter(|x| x.id == dataset.0)
			.map(|x| x.name.clone())
			.collect();

		output.datasets.insert(dataset.0, Dataset { label: name, data: dataset.1 });
	}


	return output;
}