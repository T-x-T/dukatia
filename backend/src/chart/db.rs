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

fn turn_row_into_chart(row: &tokio_postgres::Row) -> Chart {
	let id: i32 = row.get(0);
	let user_id: Option<i32> = row.get(1);
	let grid_size: String = row.get(2);
	let chart_type: String = row.get(3);
	let title: String = row.get(4);
	let text_template: Option<String> = row.get(5);
	let default_filter_from: Option<DateTime<Utc>> = row.get(6);
	let default_filter_to: Option<DateTime<Utc>> = row.get(7);
	let default_filter_collection: Option<String> = row.get(8);

	return Chart {
		id: Some(id as u32),
		user_id: user_id.map(|x| x as u32),
		grid_size,
		chart_type,
		title,
		text_template,
		filter_from: default_filter_from,
		filter_to: default_filter_to,
		filter_collection: default_filter_collection,
	};
}