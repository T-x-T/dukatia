#[cfg(test)]
mod test;

use deadpool_postgres::Pool;
use std::error::Error;
use std::collections::BTreeMap;
use chrono::prelude::*;
use uuid::Uuid;

use crate::chart::{Dataset, IntermediateChartData, DataPointMonetaryMultiCurrency, DataPoint, ChartOptions, get_relevant_time_sorted_transactions, get_date_for_period};
use super::{TagLoader, Tag};
use crate::money::Money;
use crate::transaction::Transaction;
use crate::traits::*;

pub async fn get_per_tag_over_time(pool: &Pool, options: ChartOptions) -> Result<IntermediateChartData, Box<dyn Error>> {
	let transactions = get_relevant_time_sorted_transactions(pool, &options, false).await?;
	let tags = TagLoader::new(pool).get().await?;

	return Ok(calculate_get_per_tag_over_time(&options, transactions, &tags));
}


fn calculate_get_per_tag_over_time(options: &ChartOptions, transactions: Vec<Transaction>, tags: &[Tag]) -> IntermediateChartData {
	let mut output = IntermediateChartData::default();
	let mut datasets_multi_currency: BTreeMap<Uuid, Vec<DataPointMonetaryMultiCurrency>> = BTreeMap::new();

	let default = DataPointMonetaryMultiCurrency::default();
	for transaction in transactions {
		for tag_id in transaction.tag_ids {
			let mut data_point = datasets_multi_currency.entry(tag_id).or_default().last().unwrap_or(&default).clone();
			let transaction_total_amount = transaction.total_amount.clone().unwrap();
			data_point.value.insert(transaction.currency_id.unwrap_or_default(), data_point.value.get(&transaction.currency_id.unwrap_or_default()).unwrap_or(&Money::from_amount(0, transaction_total_amount.get_minor_in_major(), transaction_total_amount.get_symbol())).clone() + transaction_total_amount);

			let timestamp: NaiveDate = get_date_for_period(options.date_period.clone().unwrap_or_default().as_str(), transaction.timestamp.date_naive());

			let mut data_point_values: Vec<&Money> = data_point.value.values().collect();
			data_point_values.sort_by_key(|b| std::cmp::Reverse(b.to_string()));

			if timestamp == data_point.timestamp.unwrap_or_default() {
				datasets_multi_currency.entry(tag_id).or_default().last_mut().unwrap().label = data_point_values.into_iter().map(std::string::ToString::to_string).collect::<Vec<String>>().join(" ");
				datasets_multi_currency.entry(tag_id).or_default().last_mut().unwrap().value = data_point.value;
			} else {
				datasets_multi_currency.entry(tag_id).or_default().push(
					DataPointMonetaryMultiCurrency { 
						name: None,
						timestamp: Some(timestamp),
						value: data_point.value.clone(),
						label: data_point_values.into_iter().map(std::string::ToString::to_string).collect::<Vec<String>>().join(" "),
				});
			}
		}
	}

	let mut datasets: BTreeMap<Uuid, Vec<DataPoint>> = BTreeMap::new();

	for dataset in datasets_multi_currency {
		for data_point in dataset.1 {
			datasets.entry(dataset.0).or_default().push(
				DataPoint { 
					name: data_point.name,
					timestamp: data_point.timestamp,
					value: data_point.value.values().map(|x| f64::from(x.to_amount()) / f64::from(x.get_minor_in_major())).sum(),
					label: data_point.label,
				}
			);
		}
	}

	output.datasets = BTreeMap::new();
	for dataset in datasets {
		let name: String = tags.iter()
			.filter(|x| x.id == dataset.0)
			.map(|x| x.name.clone())
			.collect();
		
		output.datasets.insert(dataset.0, Dataset {label: name, data: dataset.1});
	}

	return output;
}