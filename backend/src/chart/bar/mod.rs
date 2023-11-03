#[cfg(test)]
mod test;

use std::error::Error;
use deadpool_postgres::Pool;
use serde::Serialize;

use super::{Chart, ChartData};

use crate::money::Money;
use crate::CustomError;
use crate::traits::*;
use crate::budget;


#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Bar {
	pub name: String,
	pub value: f64,
	pub label: String,
}

pub async fn get_chart_data(pool: &Pool, chart: Chart) -> Result<ChartData, Box<dyn Error>> {
	if chart.filter_collection.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("filter_collection"), item_type: String::from("chart") }));
	}

	let output = match chart.filter_collection.as_ref().unwrap().as_str() {
		"compute_all_budget_utilization_overview" => compute_all_budget_utilization_overview(pool, chart).await?,
		_ => return Err(Box::new(CustomError::InvalidItem { reason: format!("Line chart collection {} is not recognized", chart.filter_collection.unwrap()) })),
	};

	return Ok(ChartData { text: None, pie: None, line: None, bar: Some(output) });
}

async fn compute_all_budget_utilization_overview(pool: &Pool, _chart: Chart) -> Result<Vec<(String, Vec<Bar>)>, Box<dyn Error>> {
	let mut output: Vec<(String, Vec<Bar>)> = vec![("used".to_string(), Vec::new()), ("available".to_string(), Vec::new())];
	let budgets = budget::BudgetLoader::new(pool).get_full().await?;

	for budget in budgets {
		let res = actually_compute_single_budget_utilization(budget);

		output[0].1.push(res.0);
		output[1].1.push(res.1);
	}

	return Ok(output);
}

fn actually_compute_single_budget_utilization(budget: budget::Budget) -> (Bar, Bar) {
	let used_amount: Money = budget.clone().used_amount.unwrap_or(Money::from_amount(0, budget.amount.get_minor_in_major(), budget.amount.get_symbol()));
	let available_amount: Money = budget.available_amount.unwrap_or(Money::from_amount(0, budget.amount.get_minor_in_major(), budget.amount.get_symbol()));

	return (
		Bar {
			name: budget.name.clone(),
			value: f64::from(used_amount.to_amount()) / if used_amount.get_minor_in_major() == 0 { 1.0 } else { f64::from(used_amount.get_minor_in_major()) },
			label: used_amount.to_string()
		},
		Bar {
			name: budget.name,
			value: f64::from(available_amount.to_amount()) / if available_amount.get_minor_in_major() == 0 { 1.0 } else { f64::from(available_amount.get_minor_in_major()) },
			label: available_amount.to_string()
		}
	);
}