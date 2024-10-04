use deadpool_postgres::Pool;
use std::error::Error;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::ChartOptions;
use crate::CustomError;

pub async fn get_by_id(pool: &Pool, id: Uuid, user_id: Uuid) -> Result<ChartOptions, Box<dyn Error>> {
	let res = pool.get()
		.await
		.unwrap()
		.query(
			"SELECT * FROM public.charts WHERE id=$1", 
			&[&id]
		).await?;

		if res.is_empty() {
			return Err(Box::new(CustomError::SpecifiedItemNotFound { item_type: String::from("chart"), filter: format!("id={id}") }));
		}

		let chart_options = turn_row_into_chart(&res[0]);

		if chart_options.user_id != user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
		}

		return Ok(chart_options);
}

pub async fn get_all_charts_in_dashboard(pool: &Pool, dashboard_id: Uuid, user_id: Uuid) -> Result<Vec<ChartOptions>, Box<dyn Error>> {
	let res = pool.get()
		.await
		.unwrap()
		.query(
			"SELECT * FROM public.charts c LEFT JOIN public.dashboard_charts dc ON c.id = dc.chart_id WHERE dc.dashboard_id = $1", 
			&[&dashboard_id]
		).await?;

	return Ok(
		res.into_iter()
		.map(|x| turn_row_into_chart(&x))
		.filter(|x| x.user_id == user_id)
		.collect()
	)
}

pub async fn add(pool: &Pool, chart: &ChartOptions) -> Result<Uuid, Box<dyn Error>> {
	let user_id: Uuid = chart.user_id;
	let max_items: Option<i32> = chart.max_items.map(|x| x as i32);
	let date_range: Option<i32> = chart.date_range.map(std::convert::Into::into);
	let top_left_x: Option<i32> = chart.top_left_x.map(|x| x as i32);
	let top_left_y: Option<i32> = chart.top_left_y.map(|x| x as i32);
	let bottom_right_x: Option<i32> = chart.bottom_right_x.map(|x| x as i32);
	let bottom_right_y: Option<i32> = chart.bottom_right_y.map(|x| x as i32);

	let client = pool.get().await?;

	client.query(
		"INSERT INTO public.charts 
			(id, user_id, chart_type, title, filter_from, filter_to, filter_collection, date_period, max_items, date_range, top_left_x, top_left_y, bottom_right_x, bottom_right_y, only_positive, only_negative, start_at_zero)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17);", 
		&[&chart.id, &user_id, &chart.chart_type.to_string(), &chart.title, &chart.filter_from, &chart.filter_to, &chart.filter_collection.as_ref().map(std::string::ToString::to_string), &chart.date_period.as_ref().map(std::string::ToString::to_string), &max_items, &date_range, &top_left_x, &top_left_y, &bottom_right_x, &bottom_right_y, &chart.only_positive, &chart.only_negative, &chart.start_at_zero]
	).await?;

	if chart.dashboard_id.is_some() {
		client.query("INSERT INTO public.dashboard_charts (dashboard_id, chart_id) VALUES ($1, $2)", &[&chart.dashboard_id.unwrap(), &chart.id]).await?;
	}

	return Ok(chart.id);
}

pub async fn update(pool: &Pool, chart: &ChartOptions) -> Result<(), Box<dyn Error>> {
	let max_items: Option<i32> = chart.max_items.map(|x| x as i32);
	let date_range: Option<i32> = chart.date_range.map(|x| x as i32);
	let top_left_x: Option<i32> = chart.top_left_x.map(|x| x as i32);
	let top_left_y: Option<i32> = chart.top_left_y.map(|x| x as i32);
	let bottom_right_x: Option<i32> = chart.bottom_right_x.map(|x| x as i32);
	let bottom_right_y: Option<i32> = chart.bottom_right_y.map(|x| x as i32);

	pool.get()
		.await
		.unwrap()
		.query(
			"UPDATE public.charts SET chart_type=$1, title=$2, filter_from=$3, filter_to=$4, filter_collection=$5, date_period=$6, max_items=$7, date_range=$8, top_left_x=$9, top_left_y=$10, bottom_right_x=$11, bottom_right_y=$12, only_positive=$13, only_negative=$14, start_at_zero=$15 WHERE id=$16", 
			&[&chart.chart_type.to_string(), &chart.title, &chart.filter_from, &chart.filter_to, &chart.filter_collection.as_ref().map(std::string::ToString::to_string), &chart.date_period.as_ref().map(std::string::ToString::to_string), &max_items, &date_range, &top_left_x, &top_left_y, &bottom_right_x, &bottom_right_y , &chart.only_positive, &chart.only_negative, &chart.start_at_zero, &chart.id]
		).await?;
	
	return Ok(());
}

pub async fn delete(pool: &Pool, chart_id: Uuid) -> Result<(), Box<dyn Error>> {
	pool.get()
		.await
		.unwrap()
		.query("DELETE FROM public.charts WHERE id=$1", &[&chart_id])
		.await?;

	return Ok(());
}

fn turn_row_into_chart(row: &tokio_postgres::Row) -> ChartOptions {
	let chart_type: String = row.get(0);
	let title: String = row.get(1);
	let filter_from: Option<DateTime<Utc>> = row.get(2);
	let filter_to: Option<DateTime<Utc>> = row.get(3);
	let filter_collection: Option<String> = row.get(4);
	let date_period: Option<String> = row.get(5);
	let max_items: Option<i32> = row.get(6);
	let date_range: Option<i32> = row.get(7);
	let top_left_x: Option<i32> = row.get(8);
	let top_left_y: Option<i32> = row.get(9);
	let bottom_right_x: Option<i32> = row.get(10);
	let bottom_right_y: Option<i32> = row.get(11);
	let only_positive: Option<bool> = row.get(12);
	let only_negative: Option<bool> = row.get(13);
	let id: Uuid = row.get(14);
	let user_id: Uuid = row.get(15);
	let start_at_zero: Option<bool> = row.get(16);
	
	return ChartOptions {
		id,
		user_id,
		chart_type: chart_type.as_str().into(),
		title,
		filter_from,
		filter_to,
		filter_collection: filter_collection.map(|x| x.as_str().into()),
		date_period: date_period.map(|x| x.as_str().into()),
		asset_id: None,
		budget_id: None,
		max_items: max_items.map(|x| x as u32),
		date_range: date_range.map(std::convert::Into::into),
		top_left_x: top_left_x.map(|x| x as u32),
		top_left_y: top_left_y.map(|x| x as u32),
		bottom_right_x: bottom_right_x.map(|x| x as u32),
		bottom_right_y: bottom_right_y.map(|x| x as u32),
		only_positive,
		only_negative,
		dashboard_id: None,
		start_at_zero,
	};
}