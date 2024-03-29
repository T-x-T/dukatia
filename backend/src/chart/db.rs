use deadpool_postgres::Pool;
use std::error::Error;
use chrono::{DateTime, Utc};

use super::ChartOptions;
use crate::CustomError;

pub async fn get_by_id(pool: &Pool, id: u32) -> Result<ChartOptions, Box<dyn Error>> {
	let res = pool.get()
		.await
		.unwrap()
		.query(
			"SELECT * FROM public.charts WHERE id=$1", 
			&[&(id as i32)]
		).await?;

		if res.is_empty() {
			return Err(Box::new(CustomError::SpecifiedItemNotFound { item_type: String::from("chart"), filter: format!("id={id}") }));
		}

		return Ok(turn_row_into_chart(&res[0]));
}

pub async fn get_all_charts_in_dashboard(pool: &Pool, dashboard_id: u32) -> Result<Vec<ChartOptions>, Box<dyn Error>> {
	let res = pool.get()
	.await
	.unwrap()
	.query(
		"SELECT * FROM public.charts c LEFT JOIN public.dashboard_charts dc ON c.id = dc.chart_id WHERE dc.dashboard_id = $1", 
		&[&(dashboard_id as i32)]
	).await?;

	return Ok(
		res.into_iter()
		.map(
			|x| turn_row_into_chart(&x) 
		).collect()
	)
}

pub async fn add(pool: &Pool, chart: &ChartOptions) -> Result<(), Box<dyn Error>> {
	let user_id: Option<i32> = chart.user_id.map(|x| x as i32);
	let max_items: Option<i32> = chart.max_items.map(|x| x as i32);
	let date_range: Option<i32> = chart.date_range.map(|x| x as i32);
	let top_left_x: Option<i32> = chart.top_left_x.map(|x| x as i32);
	let top_left_y: Option<i32> = chart.top_left_y.map(|x| x as i32);
	let bottom_right_x: Option<i32> = chart.bottom_right_x.map(|x| x as i32);
	let bottom_right_y: Option<i32> = chart.bottom_right_y.map(|x| x as i32);

	let client = pool.get().await?;

	let id: i32 = client.query(
		"INSERT INTO public.charts 
			(id, user_id, chart_type, title, filter_from, filter_to, filter_collection, date_period, max_items, date_range, top_left_x, top_left_y, bottom_right_x, bottom_right_y, only_positive, only_negative)
			VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15) RETURNING id;", 
		&[&user_id, &chart.chart_type, &chart.title, &chart.filter_from, &chart.filter_to, &chart.filter_collection, &chart.date_period, &max_items, &date_range, &top_left_x, &top_left_y, &bottom_right_x, &bottom_right_y, &chart.only_positive, &chart.only_negative]
	).await?[0].get(0);

	client.query("INSERT INTO public.dashboard_charts (dashboard_id, chart_id) VALUES ($1, $2)", &[&0i32, &id]).await?;

	return Ok(());
}

pub async fn update(pool: &Pool, chart: &ChartOptions) -> Result<(), Box<dyn Error>> {
	if chart.id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("chart") }));
	}

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
			"UPDATE public.charts SET chart_type=$1, title=$2, filter_from=$3, filter_to=$4, filter_collection=$5, date_period=$6, max_items=$7, date_range=$8, top_left_x=$9, top_left_y=$10, bottom_right_x=$11, bottom_right_y=$12, only_positive=$13, only_negative=$14 WHERE id=$15", 
			&[&chart.chart_type, &chart.title, &chart.filter_from, &chart.filter_to, &chart.filter_collection, &chart.date_period, &max_items, &date_range, &top_left_x, &top_left_y, &bottom_right_x, &bottom_right_y , &chart.only_positive, &chart.only_negative, &(chart.id.unwrap() as i32)]
		).await?;
	
	return Ok(());
}

pub async fn delete(pool: &Pool, chart_id: u32) -> Result<(), Box<dyn Error>> {
	pool.get()
		.await
		.unwrap()
		.query("DELETE FROM public.charts WHERE id=$1", &[&(chart_id as i32)])
		.await?;

	return Ok(());
}

fn turn_row_into_chart(row: &tokio_postgres::Row) -> ChartOptions {
	let id: i32 = row.get(0);
	let user_id: Option<i32> = row.get(1);
	let chart_type: String = row.get(2);
	let title: String = row.get(3);
	let filter_from: Option<DateTime<Utc>> = row.get(4);
	let filter_to: Option<DateTime<Utc>> = row.get(5);
	let filter_collection: Option<String> = row.get(6);
	let date_period: Option<String> = row.get(7);
	let max_items: Option<i32> = row.get(8);
	let date_range: Option<i32> = row.get(9);
	let top_left_x: Option<i32> = row.get(10);
	let top_left_y: Option<i32> = row.get(11);
	let bottom_right_x: Option<i32> = row.get(12);
	let bottom_right_y: Option<i32> = row.get(13);
	let only_positive: Option<bool> = row.get(14);
	let only_negative: Option<bool> = row.get(15);

	return ChartOptions {
		id: Some(id as u32),
		user_id: user_id.map(|x| x as u32),
		chart_type,
		title,
		filter_from,
		filter_to,
		filter_collection,
		date_period,
		asset_id: None,
		budget_id: None,
		max_items: max_items.map(|x| x as u32),
		date_range: date_range.map(|x| x as u32),
		top_left_x: top_left_x.map(|x| x as u32),
		top_left_y: top_left_y.map(|x| x as u32),
		bottom_right_x: bottom_right_x.map(|x| x as u32),
		bottom_right_y: bottom_right_y.map(|x| x as u32),
		only_positive,
		only_negative,
	};
}