#[cfg(test)]
mod test;

use std::error::Error;
use std::collections::BTreeMap;
use deadpool_postgres::Pool;
use chrono::{MIN_DATETIME, MAX_DATETIME};
use chrono::prelude::*;

use super::{Chart, ChartData};

use crate::CustomError;
use crate::transaction::{Transaction, TransactionLoader};
use crate::traits::*;
use crate::currency;
use crate::recipient;
use crate::tag;
use crate::budget;
use crate::money::Money;

pub async fn get_chart_data(pool: &Pool, chart: Chart) -> Result<ChartData, Box<dyn Error>> {
	if chart.filter_collection.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("filter_collection"), item_type: String::from("chart") }));
	}

	let output = match chart.filter_collection.as_ref().unwrap().as_str() {
		"recipients" => compute_recipients(pool, chart).await?,
		"tags" => compute_tags(pool, chart).await?,
		"single_budget_current_period" => compute_single_budget_current_period(pool, chart).await?,
		"single_budget_previous_period" => compute_single_budget_previous_period(pool, chart).await?,
		_ => return Err(Box::new(CustomError::InvalidItem { reason: format!("Pie chart collection {} is not recognized", chart.filter_collection.unwrap()) })),
	};

	return Ok(ChartData { text: None, pie: Some(output), line: None });
}

async fn compute_recipients(pool: &Pool, chart: Chart) -> Result<Vec<(String, (String, f64))>, Box<dyn Error>> {
	let mut output: BTreeMap<String, (String, f64)> = BTreeMap::new();
	let currencies = currency::CurrencyLoader::new(pool).get().await?;
	let transactions = get_relevant_transactions(pool, &chart).await?;
	let recipients = recipient::RecipientLoader::new(pool).get().await?;

	for recipient in recipients {
		let mut amount_per_currency: BTreeMap<u32, i32> = BTreeMap::new();
	
		transactions.iter()
			.filter(|x| x.recipient_id == recipient.id.unwrap())
			.for_each(|transaction| {
				*amount_per_currency.entry(transaction.currency_id.unwrap()).or_insert(0) += transaction.total_amount.clone().unwrap_or_default().to_amount();
			});
	
		output.insert(recipient.name, build_label_amount(amount_per_currency, &currencies));
	}

	output.retain(|_, v| v.1 != 0.0);

	let mut sorted_output = Vec::from_iter(output);
	sorted_output.sort_by(|a, b| a.1.1.total_cmp(&b.1.1));
	let limited_output: Vec<(String, (String, f64))> = sorted_output.into_iter().take(chart.max_items.unwrap_or(u32::MAX) as usize).collect();

	return Ok(limited_output);
}

async fn compute_tags(pool: &Pool, chart: Chart) -> Result<Vec<(String, (String, f64))>, Box<dyn Error>> {
	let mut output: BTreeMap<String, (String, f64)> = BTreeMap::new();
	let currencies = currency::CurrencyLoader::new(pool).get().await?;
	let transactions = get_relevant_transactions(pool, &chart).await?;
	let tags = tag::TagLoader::new(pool).get().await?;

	for tag in tags {
		let mut amount_per_currency: BTreeMap<u32, i32> = BTreeMap::new();
	
		transactions.iter()
		.filter(|x| x.tag_ids.as_ref().unwrap().contains(&tag.id.unwrap()))
		.for_each(|transaction| {
			*amount_per_currency.entry(transaction.currency_id.unwrap()).or_insert(0) += transaction.total_amount.clone().unwrap_or_default().to_amount();
		});
	
		output.insert(tag.name, build_label_amount(amount_per_currency, &currencies));
	}

	output.retain(|_, v| v.1 != 0.0);

	let mut sorted_output = Vec::from_iter(output);
	sorted_output.sort_by(|a, b| a.1.1.total_cmp(&b.1.1));
	let limited_output: Vec<(String, (String, f64))> = sorted_output.into_iter().take(chart.max_items.unwrap_or(u32::MAX) as usize).collect();

	return Ok(limited_output);
}

async fn compute_single_budget_current_period(pool: &Pool, chart: Chart) -> Result<Vec<(String, (String, f64))>, Box<dyn Error>> {
	if chart.budget_id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("budget_id"), item_type: String::from("chart") }));
	}
	
	let budget: budget::Budget = budget::BudgetLoader::new(pool)
		.set_filter_id(chart.budget_id.unwrap(), NumberFilterModes::Exact)
		.get_first()
		.await?
		.calculate_utilization_of_current_period(pool)
		.await?;

	return Ok(actually_compute_single_budget_period(budget));
}

async fn compute_single_budget_previous_period(pool: &Pool, chart: Chart) -> Result<Vec<(String, (String, f64))>, Box<dyn Error>> {
	if chart.budget_id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("budget_id"), item_type: String::from("chart") }));
	}
	
	let budget: budget::Budget = budget::BudgetLoader::new(pool)
		.set_filter_id(chart.budget_id.unwrap(), NumberFilterModes::Exact)
		.get_first()
		.await?;

	let timestamp_for_calculation: DateTime<Utc> = match budget.period {
    budget::Period::Daily => Utc::now().checked_sub_days(chrono::Days::new(1)).unwrap(),
    budget::Period::Weekly => Utc::now().checked_sub_days(chrono::Days::new(7)).unwrap(),
    budget::Period::Monthly => Utc::now().checked_sub_months(chrono::Months::new(1)).unwrap(),
    budget::Period::Quarterly => Utc::now().checked_sub_months(chrono::Months::new(3)).unwrap(),
    budget::Period::Yearly => Utc::now().checked_sub_months(chrono::Months::new(12)).unwrap(),
	};

	let budget = budget.calculate_utilization_of_period_at(pool, timestamp_for_calculation).await?;

	return Ok(actually_compute_single_budget_period(budget));
}

fn actually_compute_single_budget_period(budget: budget::Budget) -> Vec<(String, (String, f64))> {
	let mut output: Vec<(String, (String, f64))> = Vec::new();

	let used_amount: Money = budget.used_amount.unwrap_or(Money::from_amount(0, budget.amount.get_minor_in_major(), budget.amount.get_symbol()));
	let raw_available_amount: Money = budget.available_amount.unwrap_or(Money::from_amount(0, budget.amount.get_minor_in_major(), budget.amount.get_symbol()));
	let available_amount: Money = if raw_available_amount.to_amount().is_negative() { Money::from_amount(0, budget.amount.get_minor_in_major(), budget.amount.get_symbol()) } else { raw_available_amount.clone() };

	output.push(("used".to_string(), (used_amount.to_string(), f64::from(used_amount.to_amount()))));
	output.push(("available".to_string(), (available_amount.to_string(), f64::from(available_amount.to_amount()))));

	return output;
}

async fn get_relevant_transactions(pool: &Pool, chart: &Chart) -> Result<Vec<Transaction>, Box<dyn Error>> {
	let from_date = chart.filter_from.unwrap_or(MIN_DATETIME);
	let to_date = chart.filter_to.unwrap_or(MAX_DATETIME);

	return Ok(TransactionLoader::new(pool).get().await?.into_iter().filter(|x| {
		return from_date.signed_duration_since(x.timestamp).num_seconds() <= 0 
				&& to_date.signed_duration_since(x.timestamp).num_seconds() >= 0;
		}).collect());
}

fn build_label_amount(amount_per_currency: BTreeMap<u32, i32>, currencies: &[currency::Currency]) -> (String, f64) {
	let mut amount_per_currency = amount_per_currency;

	amount_per_currency.retain(|_, v| v != &mut 0);

	let mut amount: f64 = 0.0;
	let mut label = String::new();

	for x in amount_per_currency {
		let currency: &currency::Currency = currencies.iter().find(|c| c.id.unwrap() == x.0).unwrap();
		amount += f64::from(x.1) / f64::from(currency.minor_in_major);
		label.push_str(format!("{}{} ", f64::from(x.1) / f64::from(currency.minor_in_major), currency.symbol).as_str());
	}

	return (label, amount);
}