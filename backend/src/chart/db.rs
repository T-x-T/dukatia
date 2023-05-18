use deadpool_postgres::Pool;
use std::error::Error;
use chrono::{DateTime, Utc};

use super::Chart;
use crate::CustomError;

pub async fn get_by_id(pool: &Pool, id: u32) -> Result<Chart, Box<dyn Error>> {
	let res = pool.get()
		.await
		.unwrap()
		.query(
			"SELECT * FROM public.charts WHERE id=$1", 
			&[&(id as i32)]
		).await?;

		if res.len() == 0 {
			return Err(Box::new(CustomError::SpecifiedItemNotFound { item_type: String::from("chart"), filter: format!("id={id}") }));
		}

		return Ok(turn_row_into_chart(&res[0]));
}

pub async fn get_all_charts_in_dashboard(pool: &Pool, dashboard_id: u32) -> Result<Vec<Chart>, Box<dyn Error>> {
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

pub async fn add(pool: &Pool, chart: &Chart) -> Result<(), Box<dyn Error>> {
	let user_id: Option<i32> = match chart.user_id {
    Some(x) => Some(x as i32),
    None => None,
	};

	pool.get()
		.await
		.unwrap()
		.query(
			"INSERT INTO public.charts 
				(id, user_id, grid_size, chart_type, title, text_template, filter_from, filter_to, filter_collection, date_period)
				VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, $9);", 
			&[&user_id, &chart.grid_size, &chart.chart_type, &chart.title, &chart.text_template, &chart.filter_from, &chart.filter_to, &chart.filter_collection, &chart.date_period]
		).await?;

	return Ok(());
}

fn turn_row_into_chart(row: &tokio_postgres::Row) -> Chart {
	let id: i32 = row.get(0);
	let user_id: Option<i32> = row.get(1);
	let grid_size: String = row.get(2);
	let chart_type: String = row.get(3);
	let title: String = row.get(4);
	let text_template: Option<String> = row.get(5);
	let filter_from: Option<DateTime<Utc>> = row.get(6);
	let filter_to: Option<DateTime<Utc>> = row.get(7);
	let filter_collection: Option<String> = row.get(8);
	let date_period: Option<String> = row.get(9);

	return Chart {
		id: Some(id as u32),
		user_id: user_id.map(|x| x as u32),
		grid_size,
		chart_type,
		title,
		text_template,
		filter_from,
		filter_to,
		filter_collection,
		date_period,
	};
}