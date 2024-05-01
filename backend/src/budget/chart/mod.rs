#[cfg(test)]
mod test;

use deadpool_postgres::Pool;
use chrono::prelude::*;
use std::error::Error;
use uuid::Uuid;

use crate::CustomError;
use crate::chart::{Dataset, IntermediateChartData, DataPoint, ChartOptions};
use super::{BudgetLoader, Budget, Period};
use crate::money::Money;
use crate::traits::*;

pub async fn get_all_budget_utilization_overview(pool: &Pool, options: ChartOptions) -> Result<IntermediateChartData, Box<dyn Error>>	{
	let budgets = BudgetLoader::new(pool)
		.set_filter_user_id(options.user_id, NumberFilterModes::Exact)
		.get_full().await?;

	return Ok(calculate_get_all_budget_utilization_overview(budgets));
}

pub async fn get_single_budget_current_period_utilization(pool: &Pool, options: ChartOptions) -> Result<IntermediateChartData, Box<dyn Error>> {
	if options.budget_id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("budget_id"), item_type: String::from("chart") }));
	}
	
	let budget = BudgetLoader::new(pool)
		.set_filter_id(options.budget_id.unwrap(), NumberFilterModes::Exact)
		.set_filter_user_id(options.user_id, NumberFilterModes::Exact)
		.get_first_full()
		.await?;

	return Ok(calculate_get_single_budget_current_period_utilization(budget));
}

pub async fn get_single_budget_previous_period_utilization(pool: &Pool, options: ChartOptions) -> Result<IntermediateChartData, Box<dyn Error>> {
	if options.budget_id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("budget_id"), item_type: String::from("chart") }));
	}

	let budget = BudgetLoader::new(pool)
		.set_filter_id(options.budget_id.unwrap(), NumberFilterModes::Exact)
		.set_filter_user_id(options.user_id, NumberFilterModes::Exact)
		.get_first()
		.await?;

	let timestamp_for_calculation: DateTime<Utc> = match budget.period {
    Period::Daily => Utc::now().checked_sub_days(chrono::Days::new(1)).unwrap(),
    Period::Weekly => Utc::now().checked_sub_days(chrono::Days::new(7)).unwrap(),
    Period::Monthly => Utc::now().checked_sub_months(chrono::Months::new(1)).unwrap(),
    Period::Quarterly => Utc::now().checked_sub_months(chrono::Months::new(3)).unwrap(),
    Period::Yearly => Utc::now().checked_sub_months(chrono::Months::new(12)).unwrap(),
	};

	let budget = budget.calculate_utilization_of_period_at(pool, timestamp_for_calculation).await?;

	return Ok(calculate_get_single_budget_current_period_utilization(budget));
}

pub async fn get_single_budget_utilization_history(pool: &Pool, options: ChartOptions) -> Result<IntermediateChartData, Box<dyn Error>> {
	if options.budget_id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("budget_id"), item_type: String::from("chart") }));
	}

	let mut output: IntermediateChartData = IntermediateChartData::default();

	let budget = BudgetLoader::new(pool)
		.set_filter_id(options.budget_id.unwrap(), NumberFilterModes::Exact)
		.set_filter_user_id(options.user_id, NumberFilterModes::Exact)
		.get_first()
		.await?;

	let periods = budget.get_past_and_current_periods(Utc::now());

	for period in periods {
		let local_budget = budget.clone().calculate_utilization_of_period_at(pool, period.0).await?;
		let res = calculate_get_single_budget_utilization_history(&local_budget, period);

		output.datasets.entry(Uuid::from_u128(0)).or_default().label = "used".to_string();
		output.datasets.entry(Uuid::from_u128(0)).or_default().data.append(&mut res.datasets.get(&Uuid::from_u128(0)).unwrap().data.clone());
		output.datasets.entry(Uuid::from_u128(1)).or_default().label = "available".to_string();
		output.datasets.entry(Uuid::from_u128(1)).or_default().data.append(&mut res.datasets.get(&Uuid::from_u128(1)).unwrap().data.clone());
		output.datasets.entry(Uuid::from_u128(2)).or_default().label = "total".to_string();
		output.datasets.entry(Uuid::from_u128(2)).or_default().data.append(&mut res.datasets.get(&Uuid::from_u128(2)).unwrap().data.clone());
	}

	return Ok(output);
}






pub fn calculate_get_all_budget_utilization_overview(budgets: Vec<Budget>) -> IntermediateChartData {
	let mut output: IntermediateChartData = IntermediateChartData::default();

	for budget in budgets {
		let res = calculate_get_single_budget_current_period_utilization(budget.clone());

		output.datasets.entry(Uuid::from_u128(0)).or_default().label = "used".to_string();
		output.datasets.entry(Uuid::from_u128(0)).or_default().data.append(&mut res.datasets.get(&Uuid::from_u128(0)).unwrap().data.clone());
		output.datasets.entry(Uuid::from_u128(1)).or_default().label = "available".to_string();
		output.datasets.entry(Uuid::from_u128(1)).or_default().data.append(&mut res.datasets.get(&Uuid::from_u128(1)).unwrap().data.clone());
	}

	return output;
}

pub fn calculate_get_single_budget_current_period_utilization(budget: Budget) -> IntermediateChartData {
	let mut output: IntermediateChartData = IntermediateChartData::default();

	let used_amount: Money = budget.clone().used_amount.unwrap_or(Money::from_amount(0, budget.amount.get_minor_in_major(), budget.amount.get_symbol()));
	let available_amount: Money = budget.available_amount.unwrap_or(Money::from_amount(0, budget.amount.get_minor_in_major(), budget.amount.get_symbol()));

	output.datasets.insert(Uuid::from_u128(0), 
		Dataset { 
			label: "used".to_string(),
			data: vec![
				DataPoint { 
					name: Some(budget.name.clone()),
					timestamp: None,
					value: f64::from(used_amount.to_amount()) / if used_amount.get_minor_in_major() == 0 { 1.0 } else { f64::from(used_amount.get_minor_in_major()) },
					label: used_amount.to_string(),
				},
			] 
		}
	);

	output.datasets.insert(Uuid::from_u128(1), 
		Dataset { 
			label: "available".to_string(),
			data: vec![
				DataPoint { 
					name: Some(budget.name),
					timestamp: None,
					value: f64::from(available_amount.to_amount()) / if available_amount.get_minor_in_major() == 0 { 1.0 } else { f64::from(available_amount.get_minor_in_major()) },
					label: available_amount.to_string(),
				},
			] 
		}
	);

	return output;
}

pub fn calculate_get_single_budget_utilization_history(budget: &Budget, period: (DateTime<Utc>, DateTime<Utc>)) -> IntermediateChartData {
	let mut output: IntermediateChartData = IntermediateChartData::default();

	let used_amount: Money = budget.clone().used_amount.unwrap_or(Money::from_amount(0, budget.amount.get_minor_in_major(), budget.amount.get_symbol()));
	let available_amount: Money = budget.clone().available_amount.unwrap_or(Money::from_amount(0, budget.amount.get_minor_in_major(), budget.amount.get_symbol()));
	let total_amount: Money = budget.clone().amount * budget.get_period_count(if budget.rollover {budget.active_from} else {period.0}, period.1);

	output.datasets.insert(Uuid::from_u128(0), 
		Dataset { 
			label: "used".to_string(),
			data: vec![
				DataPoint { 
					name: None,
					timestamp: Some(period.0.naive_utc().date()),
					value: f64::from(used_amount.to_amount()) / if used_amount.get_minor_in_major() == 0 { 1.0 } else { f64::from(used_amount.get_minor_in_major()) },
					label: used_amount.to_string(),
				},
			] 
		}
	);

	output.datasets.insert(Uuid::from_u128(1), 
		Dataset { 
			label: "available".to_string(),
			data: vec![
				DataPoint { 
					name: None,
					timestamp: Some(period.0.naive_utc().date()),
					value: f64::from(available_amount.to_amount()) / if available_amount.get_minor_in_major() == 0 { 1.0 } else { f64::from(available_amount.get_minor_in_major()) },
					label: available_amount.to_string(),
				},
			] 
		}
	);

	output.datasets.insert(Uuid::from_u128(2), 
		Dataset { 
			label: "total".to_string(),
			data: vec![
				DataPoint { 
					name: None,
					timestamp: Some(period.0.naive_utc().date()),
					value: f64::from(total_amount.to_amount()) / if total_amount.get_minor_in_major() == 0 { 1.0 } else { f64::from(total_amount.get_minor_in_major()) },
					label: total_amount.to_string(),
				},
			] 
		}
	);

	return output;
}