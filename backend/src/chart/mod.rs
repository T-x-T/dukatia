mod db;
mod text;
mod pie;
mod line;
pub mod rest_api;

use crate::CustomError;

use serde::{Serialize, Deserialize};
use std::error::Error;
use deadpool_postgres::Pool;
use std::collections::BTreeMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize)]
pub struct Chart {
	pub id: Option<u32>,
	pub user_id: Option<u32>,
	pub grid_size: String,
	pub chart_type: String,
	pub title: String,
	pub text_template: Option<String>,
	pub filter_from: Option<DateTime<Utc>>,
	pub filter_to: Option<DateTime<Utc>>,
	pub filter_collection: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChartOptions {
	pub from_date: Option<DateTime<Utc>>,
	pub to_date: Option<DateTime<Utc>>,
	pub only_parents: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChartData {
	pub text: Option<String>,
	pub pie: Option<BTreeMap<String, (String, f64)>>,
	pub line: Option<BTreeMap<String, Vec<line::Point>>>,
}

pub async fn get_by_id(pool: &Pool, id: u32) -> Result<Chart, Box<dyn Error>> {
	return db::get_by_id(pool, id).await;
}

pub async fn get_all_charts_in_dashboard(pool: &Pool, dashboard_id: u32) -> Result<Vec<Chart>, Box<dyn Error>> {
	return db::get_all_charts_in_dashboard(pool, dashboard_id).await;
}

pub async fn get_chart_contents_by_id(pool: &Pool, chart_id: u32, options: ChartOptions) -> Result<ChartData, Box<dyn Error>> {
	let mut chart = get_by_id(pool, chart_id).await.unwrap();
	
	if options.from_date.is_some() {
		chart.filter_from = options.from_date;
	}
	
	if options.to_date.is_some() {
		chart.filter_to = options.to_date;
	}

	if chart.chart_type == "text" {
		return text::get_chart_data(pool, chart).await;
	} else if chart.chart_type == "pie" {
		return pie::get_chart_data(pool, chart).await;
	} else if chart.chart_type == "line" {
		return line::get_chart_data(pool, chart).await;
	}

	return Err(Box::new(CustomError::InvalidItem { reason: String::from("chart_type is not equal to text, pie or line") }));
}