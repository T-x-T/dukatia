#[cfg(test)]
mod test;

use deadpool_postgres::Pool;
use std::error::Error;
use std::collections::BTreeMap;
use chrono::prelude::*;
use uuid::Uuid;

use crate::chart::{Dataset, IntermediateChartData, DataPointMonetaryMultiCurrency, DataPoint, ChartOptions, get_relevant_time_sorted_transactions, get_date_for_period};
use super::Transaction;
use crate::money::Money;

pub async fn get_earning_spending_net_over_time(pool: &Pool, options: ChartOptions) -> Result<IntermediateChartData, Box<dyn Error>> {
	let transactions = get_relevant_time_sorted_transactions(pool, &options, false).await?;

	return Ok(calculate_get_earning_spending_net_over_time(&options, transactions));
}

fn calculate_get_earning_spending_net_over_time(options: &ChartOptions, transactions: Vec<Transaction>) -> IntermediateChartData {
	let mut output = IntermediateChartData::default();
	let mut datasets_monetary: BTreeMap<Uuid, Vec<DataPointMonetaryMultiCurrency>> = BTreeMap::new();

	let default = DataPointMonetaryMultiCurrency::default();
	for transaction in transactions {
		let mut data_point_earning = datasets_monetary.entry(Uuid::from_u128(0)).or_default().last().unwrap_or(&default).clone();
		let mut data_point_spending = datasets_monetary.entry(Uuid::from_u128(1)).or_default().last().unwrap_or(&default).clone();
		let mut data_point_net = datasets_monetary.entry(Uuid::from_u128(2)).or_default().last().unwrap_or(&default).clone();

		let transaction_total_amount = transaction.total_amount.unwrap();

		let timestamp: NaiveDate = get_date_for_period(options.date_period.clone().unwrap_or_default().as_str(), transaction.timestamp.date_naive());

		if transaction_total_amount.to_amount().is_positive() {
			if timestamp == data_point_earning.timestamp.unwrap_or_default() {
				data_point_earning.value.insert(transaction.currency_id.unwrap_or_default(), data_point_earning.value.get(&transaction.currency_id.unwrap_or_default()).unwrap_or(&Money::from_amount(0, transaction_total_amount.get_minor_in_major(), transaction_total_amount.get_symbol())).clone() + transaction_total_amount.clone());
				let mut sorted_datapoint: Vec<Money> = data_point_earning.value.values().cloned().collect();
				sorted_datapoint.sort_by_key(|x| std::cmp::Reverse(x.to_string()));
				
				datasets_monetary.entry(Uuid::from_u128(0)).or_default().last_mut().unwrap().label = sorted_datapoint.iter().map(std::string::ToString::to_string).collect::<Vec<String>>().join(" ");
				datasets_monetary.entry(Uuid::from_u128(0)).or_default().last_mut().unwrap().value = data_point_earning.value;
			} else {
				datasets_monetary.entry(Uuid::from_u128(0)).or_default().push(
					DataPointMonetaryMultiCurrency { 
						name: None,
						timestamp: Some(timestamp),
						value: vec![(transaction.currency_id.unwrap_or_default(), transaction_total_amount.clone())].into_iter().collect(),
						label: transaction_total_amount.clone().to_string(),
					}
				);
			}
		}

		if transaction_total_amount.to_amount().is_negative() {
			if timestamp == data_point_spending.timestamp.unwrap_or_default() {
				data_point_spending.value.insert(transaction.currency_id.unwrap_or_default(), data_point_spending.value.get(&transaction.currency_id.unwrap_or_default()).unwrap_or(&Money::from_amount(0, transaction_total_amount.get_minor_in_major(), transaction_total_amount.get_symbol())).clone() + transaction_total_amount.clone());
				let mut sorted_datapoint: Vec<Money> = data_point_spending.value.values().cloned().collect();
				sorted_datapoint.sort_by_key(|x| std::cmp::Reverse(x.to_string()));
				
				datasets_monetary.entry(Uuid::from_u128(1)).or_default().last_mut().unwrap().label = sorted_datapoint.iter().map(std::string::ToString::to_string).collect::<Vec<String>>().join(" ");
				datasets_monetary.entry(Uuid::from_u128(1)).or_default().last_mut().unwrap().value = data_point_spending.value;
			} else {
				datasets_monetary.entry(Uuid::from_u128(1)).or_default().push(
					DataPointMonetaryMultiCurrency { 
						name: None,
						timestamp: Some(timestamp),
						value: vec![(transaction.currency_id.unwrap_or_default(), transaction_total_amount.clone())].into_iter().collect(),
						label: transaction_total_amount.clone().to_string(),
					}
				);
			}
		}

		if timestamp == data_point_net.timestamp.unwrap_or_default() {
			data_point_net.value.insert(transaction.currency_id.unwrap_or_default(), data_point_net.value.get(&transaction.currency_id.unwrap_or_default()).unwrap_or(&Money::from_amount(0, transaction_total_amount.get_minor_in_major(), transaction_total_amount.get_symbol())).clone() + transaction_total_amount);
			let mut sorted_datapoint: Vec<Money> = data_point_net.value.values().cloned().collect();
			sorted_datapoint.sort_by_key(|x| std::cmp::Reverse(x.to_string()));
			
			datasets_monetary.entry(Uuid::from_u128(2)).or_default().last_mut().unwrap().label = sorted_datapoint.iter().map(std::string::ToString::to_string).collect::<Vec<String>>().join(" ");
			datasets_monetary.entry(Uuid::from_u128(2)).or_default().last_mut().unwrap().value = data_point_net.value;
		} else {
			datasets_monetary.entry(Uuid::from_u128(2)).or_default().push(
				DataPointMonetaryMultiCurrency { 
					name: None,
					timestamp: Some(timestamp),
					value: vec![(transaction.currency_id.unwrap_or_default(), transaction_total_amount.clone())].into_iter().collect(),
					label: transaction_total_amount.clone().to_string(),
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
					value: data_point.value.values().map(|x| f64::from(x.to_amount()) / f64::from(x.get_minor_in_major())).sum(),
					label: data_point.label,
				}
			);
		}
	}

	output.datasets = BTreeMap::new();
	for dataset in datasets {
		let name: &str = if dataset.0 == Uuid::from_u128(0) {
			"Earning"
		} else if dataset.0 == Uuid::from_u128(1) {
			"Spending"
		} else {
			"Net"
		};

		output.datasets.insert(dataset.0, Dataset { label: name.to_string(), data: dataset.1 });
	}

	return output;
}
