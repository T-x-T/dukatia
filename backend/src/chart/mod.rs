mod db;
pub mod rest_api;

#[cfg(test)]
mod test;

use crate::transaction::{self, Transaction, TransactionLoader};
use crate::traits::*;
use crate::money::Money;
use crate::recipient;
use crate::budget;
use crate::asset;
use crate::account;
use crate::currency;
use crate::tag;

use serde::Serialize;
use serde_repr::*;
use std::error::Error;
use std::collections::BTreeMap;
use std::fmt::Display;
use deadpool_postgres::Pool;
use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct ChartOptions {
	pub id: Uuid,
	pub user_id: Uuid,
	pub chart_type: ChartType,
	pub title: String,
	pub filter_from: Option<DateTime<Utc>>,
	pub filter_to: Option<DateTime<Utc>>,
	pub filter_collection: Option<FilterCollection>,
	pub date_period: Option<DatePeriod>,
	pub asset_id: Option<Uuid>,
	pub budget_id: Option<Uuid>,
	pub max_items: Option<u32>,
	pub date_range: Option<DateRange>,
	pub only_positive: Option<bool>,
	pub only_negative: Option<bool>,
	pub top_left_x: Option<u32>,
	pub top_left_y: Option<u32>,
	pub bottom_right_x: Option<u32>,
	pub bottom_right_y: Option<u32>,
	pub dashboard_id: Option<Uuid>,
}

impl Default for ChartOptions {
	fn default() -> Self {
		Self {
			id: Uuid::new_v4(),
			user_id: Uuid::nil(),
			chart_type: ChartType::Line,
			title: String::new(),
			filter_from: None,
			filter_to: None,
			filter_collection: None,
			date_period: None,
			asset_id: None,
			budget_id: None,
			max_items: None,
			date_range: None,
			only_positive: None,
			only_negative: None,
			top_left_x: None,
			top_left_y: None,
			bottom_right_x: None,
			bottom_right_y: None,
			dashboard_id: None,
		}		 
	}
}

#[derive(Debug, Clone, Copy, Serialize_repr, Default)]
#[repr(u32)]
pub enum DateRange {
	#[default] Last28Days = 0,
	LastMonth = 1,
	CurrentMonth = 2,
	Last90Days = 3,
	LastQuarter = 4,
	CurrentYear = 5,
	Last365days = 6,
	Total = 7,
}

impl From<u32> for DateRange {
	fn from(value: u32) -> Self {
		match value {
			1 => DateRange::LastMonth,
			2 => DateRange::CurrentMonth,
			3 => DateRange::Last90Days,
			4 => DateRange::LastQuarter,
			5 => DateRange::CurrentYear,
			6 => DateRange::Last365days,
			7 => DateRange::Total,
			_ => DateRange::Last28Days,
		}
	}
}

impl From<i32> for DateRange {
	fn from(value: i32) -> Self {
		assert!(value >= 0);
		return (value as u32).into();
	}
}

impl From<DateRange> for u32 {
	fn from(value: DateRange) -> Self {
		match value {
			DateRange::Last28Days => 0,
			DateRange::LastMonth => 1,
			DateRange::CurrentMonth => 2,
			DateRange::Last90Days => 3,
			DateRange::LastQuarter => 4,
			DateRange::CurrentYear => 5,
			DateRange::Last365days => 6,
			DateRange::Total => 7,
		}
	}
}

impl From<DateRange> for i32 {
	fn from(value: DateRange) -> Self {
		Into::<u32>::into(value) as i32
	}
}

#[derive(Debug, Clone, Copy, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DatePeriod {
	#[default] Daily,
	Monthly,
	Quarterly,
	Yearly,
}

impl DatePeriod {
	pub fn get_date_at_timestamp(self, timestamp: NaiveDate) -> NaiveDate {
		match self {
			DatePeriod::Yearly => {
				NaiveDate::from_ymd_opt(timestamp.year(), 1, 1).unwrap()
			},
			DatePeriod::Quarterly => {
				match timestamp.month() {
					1..=3 => NaiveDate::from_ymd_opt(timestamp.year(), 1, 1).unwrap(),
					4..=6 => NaiveDate::from_ymd_opt(timestamp.year(), 4, 1).unwrap(),
					7..=9 => NaiveDate::from_ymd_opt(timestamp.year(), 7, 1).unwrap(),
					_ => NaiveDate::from_ymd_opt(timestamp.year(), 10, 1).unwrap(),
				}
			},
			DatePeriod::Monthly => {
				NaiveDate::from_ymd_opt(timestamp.year(), timestamp.month(), 1).unwrap()
			},
			DatePeriod::Daily => {
				timestamp
			},
		}
	}
}

impl Display for DatePeriod {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DatePeriod::Daily => f.write_str("daily"),
			DatePeriod::Monthly => f.write_str("monthly"),
			DatePeriod::Quarterly => f.write_str("quarterly"),
			DatePeriod::Yearly => f.write_str("yearly"),
		}
	}
}

impl From<&str> for DatePeriod {
	fn from(value: &str) -> Self {
		match value {
			"monthly" => DatePeriod::Monthly,
			"quarterly" => DatePeriod::Quarterly,
			"yearly" => DatePeriod::Yearly,
			_ => DatePeriod::Daily,
		}
	}
}

#[derive(Debug, Clone, Copy, Serialize, Default)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum FilterCollection {
	GetPerRecipientOverTime,
	GetSingleBudgetUtilizationHistory,
	GetAllBudgetUtilizationOverview,
	GetSingleBudgetCurrentPeriodUtilization,
	GetSingleBudgetPreviousPeriodUtilization,
	GetSingleAssetTotalValueOverTime,
	GetSingleAssetSingleValueOverTime,
	GetSingleAssetAmountOverTime,
	GetPerAccountOverTime,
	#[default] GetEarningSpendingNetOverTime,
	GetPerTagOverTime,
	GetPerCurrencyOverTime,
}

impl Display for FilterCollection {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			FilterCollection::GetPerRecipientOverTime => f.write_str("get_per_recipient_over_time"),
			FilterCollection::GetSingleBudgetUtilizationHistory => f.write_str("get_single_budget_utilization_history"),
			FilterCollection::GetAllBudgetUtilizationOverview => f.write_str("get_all_budget_utilization_overview"),
			FilterCollection::GetSingleBudgetCurrentPeriodUtilization => f.write_str("get_single_budget_current_period_utilization"),
			FilterCollection::GetSingleBudgetPreviousPeriodUtilization => f.write_str("get_single_budget_previous_period_utilization"),
			FilterCollection::GetSingleAssetTotalValueOverTime => f.write_str("get_single_asset_total_value_over_time"),
			FilterCollection::GetSingleAssetSingleValueOverTime => f.write_str("get_single_asset_single_value_over_time"),
			FilterCollection::GetSingleAssetAmountOverTime => f.write_str("get_single_asset_amount_over_time"),
			FilterCollection::GetPerAccountOverTime => f.write_str("get_per_account_over_time"),
			FilterCollection::GetEarningSpendingNetOverTime => f.write_str("get_earning_spending_net_over_time"),
			FilterCollection::GetPerTagOverTime => f.write_str("get_per_tag_over_time"),
			FilterCollection::GetPerCurrencyOverTime => f.write_str("get_per_currency_over_time"),
		}
	}
}

impl From<&str> for FilterCollection {
	fn from(value: &str) -> Self {
		match value {
			"get_per_recipient_over_time" => FilterCollection::GetPerRecipientOverTime,
			"get_single_budget_utilization_history" => FilterCollection::GetSingleBudgetUtilizationHistory,
			"get_all_budget_utilization_overview" => FilterCollection::GetAllBudgetUtilizationOverview,
			"get_single_budget_current_period_utilization" => FilterCollection::GetSingleBudgetCurrentPeriodUtilization,
			"get_single_budget_previous_period_utilization" => FilterCollection::GetSingleBudgetPreviousPeriodUtilization,
			"get_single_asset_total_value_over_time" => FilterCollection::GetSingleAssetTotalValueOverTime,
			"get_single_asset_single_value_over_time" => FilterCollection::GetSingleAssetSingleValueOverTime,
			"get_single_asset_amount_over_time" => FilterCollection::GetSingleAssetAmountOverTime,
			"get_per_account_over_time" => FilterCollection::GetPerAccountOverTime,
			"get_per_tag_over_time" => FilterCollection::GetPerTagOverTime,
			"get_per_currency_over_time" => FilterCollection::GetPerCurrencyOverTime,
			_ => FilterCollection::GetEarningSpendingNetOverTime,
		}
	}
}

#[derive(Debug, Clone, Copy, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ChartType {
	#[default] Line,
	Pie,
	Table,
}

impl Display for ChartType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ChartType::Line => f.write_str("line"),
			ChartType::Pie => f.write_str("pie"),
			ChartType::Table => f.write_str("table"),
		}
	}
}

impl From<&str> for ChartType {
	fn from(value: &str) -> Self {
		match value {
			"pie" => ChartType::Pie,
			"table" => ChartType::Table,
			_ => ChartType::Line,
		}
	}
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ChartData {
	pub datasets: Vec<Dataset>,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd)]
pub struct IntermediateChartData {
	pub datasets: BTreeMap<Uuid, Dataset>,
}

impl IntermediateChartData {
	fn sort(self) -> Vec<(Uuid, Dataset)> {
		let mut datasets = Vec::from_iter(self.datasets);
		datasets.sort_by(|a, b| b.1.data.last().unwrap().value.total_cmp(&a.1.data.last().unwrap().value));
		return datasets;
	}
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd)]
pub struct Dataset {
	pub label: String,
	pub data: Vec<DataPoint>,
}

#[derive(Debug, Clone, Serialize, Default, PartialEq, PartialOrd)]
pub struct DataPoint {
	pub name: Option<String>,
	pub timestamp: Option<chrono::NaiveDate>,
	pub value: f64,
	pub label: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct DataPointMonetary {
	pub name: Option<String>,
	pub timestamp: Option<chrono::NaiveDate>,
	pub value: Money,
	pub label: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct DataPointMonetaryMultiCurrency {
	pub name: Option<String>,
	pub timestamp: Option<chrono::NaiveDate>,
	pub value: BTreeMap<Uuid, Money>,
	pub label: String,
}







pub async fn get_by_id(pool: &Pool, id: Uuid, user_id: Uuid) -> Result<ChartOptions, Box<dyn Error>> {
	return db::get_by_id(pool, id, user_id).await;
}

pub async fn get_all_charts_in_dashboard(pool: &Pool, dashboard_id: Uuid, user_id: Uuid) -> Result<Vec<ChartOptions>, Box<dyn Error>> {
	return db::get_all_charts_in_dashboard(pool, dashboard_id, user_id).await;
}

pub async fn add(pool: &Pool, chart: &ChartOptions) -> Result<Uuid, Box<dyn Error>> {
	return db::add(pool, chart).await;
}

pub async fn update(pool: &Pool, chart: &ChartOptions) -> Result<(), Box<dyn Error>> {
	return db::update(pool, chart).await;
}

pub async fn delete(pool: &Pool, chart_id: Uuid) -> Result<(), Box<dyn Error>> {
	return db::delete(pool, chart_id).await;
}

pub async fn get_chart_data(pool: &Pool, options: ChartOptions) -> Result<ChartData, Box<dyn Error>> {
	let output = match options.filter_collection.unwrap_or_default() {
		FilterCollection::GetPerRecipientOverTime => recipient::chart::get_per_recipient_over_time(pool, options.clone()).await?,
		FilterCollection::GetSingleBudgetUtilizationHistory => budget::chart::get_single_budget_utilization_history(pool, options.clone()).await?,
		FilterCollection::GetAllBudgetUtilizationOverview => budget::chart::get_all_budget_utilization_overview(pool, options.clone()).await?,
		FilterCollection::GetSingleBudgetCurrentPeriodUtilization => budget::chart::get_single_budget_current_period_utilization(pool, options.clone()).await?,
		FilterCollection::GetSingleBudgetPreviousPeriodUtilization => budget::chart::get_single_budget_previous_period_utilization(pool, options.clone()).await?,
		FilterCollection::GetSingleAssetTotalValueOverTime => asset::chart::get_single_asset_total_value_over_time(pool, options.clone()).await?,
		FilterCollection::GetSingleAssetSingleValueOverTime => asset::chart::get_single_asset_single_value_over_time(pool, options.clone()).await?,
		FilterCollection::GetSingleAssetAmountOverTime => asset::chart::get_single_asset_amount_over_time(pool, options.clone()).await?,
		FilterCollection::GetPerAccountOverTime => account::chart::get_per_account_over_time(pool, options.clone()).await?,
		FilterCollection::GetEarningSpendingNetOverTime => transaction::chart::get_earning_spending_net_over_time(pool, options.clone()).await?,
		FilterCollection::GetPerTagOverTime => tag::chart::get_per_tag_over_time(pool, options.clone()).await?,
		FilterCollection::GetPerCurrencyOverTime => currency::chart::get_per_currency_over_time(pool, options.clone()).await?,
	};
	
	let limited_output: Vec<(Uuid, Dataset)>;
	if options.only_positive.is_some() && options.only_positive.unwrap() {
		limited_output = limit_output_only_positive(output.sort(), options.max_items);
	} else if options.only_negative.is_some() && options.only_negative.unwrap() {
		limited_output = limit_output_only_negative(output.sort(), options.max_items);
	} else {
		limited_output = limit_output(output.sort(), options.max_items);
	}
	let datasets: Vec<Dataset> = limited_output.into_iter().map(|x| x.1).collect();

	return Ok(ChartData {datasets});
}




pub async fn get_relevant_time_sorted_transactions(pool: &Pool, chart: &ChartOptions, get_all: bool) -> Result<Vec<Transaction>, Box<dyn Error>> {
	let min_time = DateTime::parse_from_str("0000-01-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc);
	let max_time = DateTime::parse_from_str("9999-12-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc);
	
	let from_date = if get_all {
		min_time
	} else {
		chart.filter_from.unwrap_or(min_time)
	};
	let to_date = if get_all {
		max_time
	} else {
		chart.filter_to.unwrap_or(max_time)
	};

	let transactions = TransactionLoader::new(pool)
	.set_query_parameters(
		QueryParameters::default()
			.set_sort_property_opt(Some(FilterAndSortProperties::Timestamp))
			.set_sort_direction_opt(Some(SortDirection::Asc))
		)
		.set_filter_time_range(from_date, to_date, TimeRangeFilterModes::Between)
		.set_filter_user_id(chart.user_id, NumberFilterModes::Exact)
		.get().await?;

	return Ok(transactions);
}

fn limit_output(mut input: Vec<(Uuid, Dataset)>, limit: Option<u32>) -> Vec<(Uuid, Dataset)> {
	let mut output: Vec<(Uuid, Dataset)>;
	
	if limit.is_some() && input.len() > limit.unwrap() as usize {
		if limit.unwrap() == 1 {
			output = input.clone().into_iter().take(1).collect();
		} else {
			let n_from_top = (f64::from(limit.unwrap()) / 2.0).ceil() as usize;
			let top_limited_output: Vec<(Uuid, Dataset)> = input.clone().into_iter().take(n_from_top).collect();
			input.reverse();
			let mut bottom_limited_output: Vec<(Uuid, Dataset)> = input.into_iter().take(limit.unwrap() as usize - n_from_top).collect();
			bottom_limited_output.reverse();
			output = top_limited_output;
			output.append(&mut bottom_limited_output);
		}
	} else {
		output = input;
	}

	return output;
}

fn limit_output_only_positive(input: Vec<(Uuid, Dataset)>, limit: Option<u32>) -> Vec<(Uuid, Dataset)> {
	let default = DataPoint::default();
	let output: Vec<(Uuid, Dataset)> = if limit.is_some() && input.len() > limit.unwrap() as usize {
		input.into_iter().filter(|x| x.1.data.last().unwrap_or(&default).value.is_sign_positive()).take(limit.unwrap() as usize).collect()
	} else {
		input.into_iter().filter(|x| x.1.data.last().unwrap_or(&default).value.is_sign_positive()).collect()
	};

	return output;
}

fn limit_output_only_negative(mut input: Vec<(Uuid, Dataset)>, limit: Option<u32>) -> Vec<(Uuid, Dataset)> {
	let default = DataPoint::default();
	input.reverse();
	let output: Vec<(Uuid, Dataset)> = if limit.is_some() && input.len() > limit.unwrap() as usize {
		input.clone().into_iter().filter(|x| x.1.data.last().unwrap_or(&default).value.is_sign_negative()).take(limit.unwrap() as usize).collect()
	} else {
		input.into_iter().filter(|x| x.1.data.last().unwrap_or(&default).value.is_sign_negative()).collect()
	};

	return output;
}