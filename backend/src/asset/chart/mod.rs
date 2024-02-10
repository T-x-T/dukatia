#[cfg(test)]
mod test;

use deadpool_postgres::Pool;
use chrono::prelude::*;
use chrono::Duration;
use std::error::Error;
use std::collections::BTreeMap;

use crate::CustomError;
use crate::money::Money;
use crate::chart::{Dataset, OldIntermediateChartData, DataPoint, ChartOptions};
use crate::traits::*;
use super::{Asset, AssetLoader, AssetValuation, AssetValuationLoader};



pub async fn get_single_asset_total_value_over_time(pool: &Pool, options: ChartOptions) -> Result<OldIntermediateChartData, Box<dyn Error>> {
	if options.asset_id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("asset_id"), item_type: String::from("chart") }));
	}

	let asset = AssetLoader::new(pool)
		.set_filter_id(options.asset_id.unwrap(), NumberFilterModes::Exact)
		.set_filter_user_id(options.user_id, NumberFilterModes::Exact)
		.get_first()
		.await?;

	let asset_valuation_history = AssetValuationLoader::new(pool)
		.set_filter_asset_id(options.asset_id.unwrap(), NumberFilterModes::Exact)
		.get()
		.await?;

	return Ok(calculate_get_single_asset_total_value_over_time(asset, &asset_valuation_history));
}

pub async fn get_single_asset_single_value_over_time(pool: &Pool, options: ChartOptions) -> Result<OldIntermediateChartData, Box<dyn Error>> {
	if options.asset_id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("asset_id"), item_type: String::from("chart") }));
	}

	let asset = AssetLoader::new(pool)
		.set_filter_id(options.asset_id.unwrap(), NumberFilterModes::Exact)
		.set_filter_user_id(options.user_id, NumberFilterModes::Exact)
		.get_first()
		.await?;

	let asset_valuation_history = AssetValuationLoader::new(pool)
		.set_filter_asset_id(options.asset_id.unwrap(), NumberFilterModes::Exact)
		.get()
		.await?;

	return Ok(calculate_get_single_asset_single_value_over_time(asset, &asset_valuation_history));
}

pub async fn get_single_asset_amount_over_time(pool: &Pool, options: ChartOptions) -> Result<OldIntermediateChartData, Box<dyn Error>> {
	if options.asset_id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("asset_id"), item_type: String::from("chart") }));
	}

	let asset = AssetLoader::new(pool)
		.set_filter_id(options.asset_id.unwrap(), NumberFilterModes::Exact)
		.set_filter_user_id(options.user_id, NumberFilterModes::Exact)
		.get_first()
		.await?;

	let asset_valuation_history = AssetValuationLoader::new(pool)
		.set_filter_asset_id(options.asset_id.unwrap(), NumberFilterModes::Exact)
		.get()
		.await?;

	return Ok(calculate_get_single_asset_amount_over_time(asset, &asset_valuation_history));
}








fn calculate_get_single_asset_total_value_over_time(asset: Asset, asset_valuation_history: &[AssetValuation]) -> OldIntermediateChartData {
	let value_history: BTreeMap<NaiveDate, Money> = asset_valuation_history.iter()
		.map(|x| (x.timestamp.naive_utc().date(), x.value_per_unit.clone()))
		.collect();
	
	let amount_history: BTreeMap<NaiveDate, f64> = asset_valuation_history.iter()
		.map(|x| (x.timestamp.naive_utc().date(), x.amount))
		.collect();

	if value_history.is_empty() || amount_history.is_empty() {
		return OldIntermediateChartData::default();
	}	

	let mut dataset = Dataset { 
		label: asset.name,
		..Default::default()
	};

	let mut first_day: NaiveDate = Utc::now().date_naive();
	if !value_history.is_empty() && value_history.first_key_value().unwrap().0.signed_duration_since(first_day).num_seconds() < 0 {
		first_day = *value_history.first_key_value().unwrap().0;	
	}
	if !amount_history.is_empty() && amount_history.first_key_value().unwrap().0.signed_duration_since(first_day).num_seconds() < 0 {
		first_day = *amount_history.first_key_value().unwrap().0;	
	}
	let tomorrow: NaiveDate = Utc::now().date_naive().checked_add_signed(chrono::Duration::days(1)).unwrap();

	let mut last_value = f64::MIN;
	let mut current_day = first_day;

	while tomorrow.signed_duration_since(current_day).num_seconds() > 0 {
		let no_future_values: BTreeMap<&NaiveDate, &Money> = value_history.iter().filter(|(x, _)| x.signed_duration_since(current_day).num_seconds() <= 0).collect();
		let no_future_amounts: BTreeMap<&NaiveDate, &f64> = amount_history.iter().filter(|(x, _)| x.signed_duration_since(current_day).num_seconds() <= 0).collect();
		let minor_in_major = no_future_values.last_key_value().unwrap().1.get_minor_in_major();
		let value = (f64::from(no_future_values.last_key_value().unwrap().1.to_amount()) * **no_future_amounts.last_key_value().unwrap().1) / f64::from(minor_in_major);

		if (last_value - value).abs() > 0.0001 {
			dataset.data.push(DataPoint {
				name: None,
				timestamp: Some(current_day),
				value,
				label: Money::from_amount((value * f64::from(minor_in_major)).round() as i32, minor_in_major, no_future_values.last_key_value().unwrap().1.get_symbol()).to_string(),
			});
		}

		last_value = value;
		current_day += Duration::days(1);
	}

	return OldIntermediateChartData {
		datasets: vec![(asset.id.unwrap_or_default(), dataset)].into_iter().collect(),
	};
}

fn calculate_get_single_asset_single_value_over_time(asset: Asset, asset_valuation_history: &[AssetValuation]) -> OldIntermediateChartData {
	let value_history: BTreeMap<NaiveDate, Money> = asset_valuation_history.iter()
		.map(|x| (x.timestamp.naive_utc().date(), x.value_per_unit.clone()))
		.collect();

	if value_history.is_empty() {
		return OldIntermediateChartData::default();
	}	

	let mut dataset = Dataset {
		label: asset.name,
		..Default::default()
	};

	let mut first_day: NaiveDate = Utc::now().date_naive();
	if !value_history.is_empty() && value_history.first_key_value().unwrap().0.signed_duration_since(first_day).num_seconds() < 0 {
		first_day = *value_history.first_key_value().unwrap().0;	
	}
	let tomorrow: NaiveDate = Utc::now().date_naive().checked_add_signed(chrono::Duration::days(1)).unwrap();

	let mut last_value = f64::MIN;
	let mut current_day = first_day;

	while tomorrow.signed_duration_since(current_day).num_seconds() > 0 {
		let no_future_values: BTreeMap<&NaiveDate, &Money> = value_history.iter().filter(|(x, _)| x.signed_duration_since(current_day).num_seconds() <= 0).collect();
		let minor_in_major = no_future_values.last_key_value().unwrap().1.get_minor_in_major();
		let value = f64::from(no_future_values.last_key_value().unwrap().1.to_amount()) / f64::from(minor_in_major);

		if (last_value - value).abs() > 0.0001 {
			dataset.data.push(DataPoint {
				name: None,
				timestamp: Some(current_day),
				value,
				label: Money::from_amount((value * f64::from(minor_in_major)).round() as i32, minor_in_major, no_future_values.last_key_value().unwrap().1.get_symbol()).to_string(),
			});
		}

		last_value = value;
		current_day += Duration::days(1);
	}

	return OldIntermediateChartData {
		datasets: vec![(asset.id.unwrap_or_default(), dataset)].into_iter().collect(),
	};
}

fn calculate_get_single_asset_amount_over_time(asset: Asset, asset_valuation_history: &[AssetValuation]) -> OldIntermediateChartData {
	let amount_history: BTreeMap<NaiveDate, f64> = asset_valuation_history.iter()
		.map(|x| (x.timestamp.naive_utc().date(), x.amount))
		.collect();

	if amount_history.is_empty() {
		return OldIntermediateChartData::default();
	}	

	let mut dataset = Dataset {
		label: asset.name,
		..Default::default()
	};

	let mut first_day: NaiveDate = Utc::now().date_naive();
	if !amount_history.is_empty() && amount_history.first_key_value().unwrap().0.signed_duration_since(first_day).num_seconds() < 0 {
		first_day = *amount_history.first_key_value().unwrap().0;	
	}
	let tomorrow: NaiveDate = Utc::now().date_naive().checked_add_signed(chrono::Duration::days(1)).unwrap();

	let mut last_value = f64::MIN;
	let mut current_day = first_day;

	while tomorrow.signed_duration_since(current_day).num_seconds() > 0 {
		let no_future_amounts: BTreeMap<&NaiveDate, &f64> = amount_history.iter().filter(|(x, _)| x.signed_duration_since(current_day).num_seconds() <= 0).collect();
	
		let value = **no_future_amounts.last_key_value().unwrap().1;

		if (last_value - value).abs() > 0.0001 {
			dataset.data.push(DataPoint {
				name: None,
				timestamp: Some(current_day),
				value,
				label: value.to_string(),
			});
		}

		last_value = value;
		current_day += Duration::days(1);
	}

	return OldIntermediateChartData {
		datasets: vec![(asset.id.unwrap_or_default(), dataset)].into_iter().collect(),
	};
}