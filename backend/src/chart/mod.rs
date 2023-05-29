mod db;
mod text;
mod pie;
mod line;
pub mod rest_api;

use crate::CustomError;

use serde::Serialize;
use std::error::Error;
use deadpool_postgres::Pool;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize)]
pub struct Chart {
	pub id: Option<u32>,
	pub user_id: Option<u32>,
	pub chart_type: String,
	pub title: String,
	pub text_template: Option<String>,
	pub filter_from: Option<DateTime<Utc>>,
	pub filter_to: Option<DateTime<Utc>>,
	pub filter_collection: Option<String>,
	pub date_period: Option<String>,
	pub asset_id: Option<u32>,
	pub max_items: Option<u32>,
	pub date_range: Option<u32>,
	pub top_left_x: Option<u32>,
	pub top_left_y: Option<u32>,
	pub bottom_right_x: Option<u32>,
	pub bottom_right_y: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChartData {
	pub text: Option<String>,
	pub pie: Option<Vec<(String, (String, f64))>>,
	pub line: Option<Vec<(std::string::String, Vec<line::Point>)>>,
}

pub async fn get_by_id(pool: &Pool, id: u32) -> Result<Chart, Box<dyn Error>> {
	return db::get_by_id(pool, id).await;
}

pub async fn get_all_charts_in_dashboard(pool: &Pool, dashboard_id: u32) -> Result<Vec<Chart>, Box<dyn Error>> {
	return db::get_all_charts_in_dashboard(pool, dashboard_id).await;
}

pub async fn add(pool: &Pool, chart: &Chart) -> Result<(), Box<dyn Error>> {
	return db::add(pool, chart).await;
}

pub async fn update(pool: &Pool, chart: &Chart) -> Result<(), Box<dyn Error>> {
	return db::update(pool, chart).await;
}

pub async fn delete(pool: &Pool, chart_id: u32) -> Result<(), Box<dyn Error>> {
	return db::delete(pool, chart_id).await;
}

pub async fn get_chart_contents_by_id(pool: &Pool, chart_id: u32, options: rest_api::ChartOptions) -> Result<ChartData, Box<dyn Error>> {
	let mut chart = get_by_id(pool, chart_id).await?;
	
	if options.from_date.is_some() {
		chart.filter_from = options.from_date;
	}
	if options.to_date.is_some() {
		chart.filter_to = options.to_date;
	}
	if options.date_period.is_some() {
		chart.date_period = options.date_period;
	}
	if options.asset_id.is_some() {
		chart.asset_id = options.asset_id;
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