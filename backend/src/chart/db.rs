use deadpool_postgres::Pool;
use std::error::Error;
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

fn turn_row_into_chart(row: &tokio_postgres::Row) -> Chart {
	let id: i32 = row.get(0);
	let user_id: Option<i32> = row.get(1);
	let grid_size: String = row.get(2);
	let chart_type: String = row.get(3);
	let title: String = row.get(4);
	let text_template: Option<String> = row.get(5);

	return Chart {
		id: Some(id as u32),
		user_id: user_id.map(|x| x as u32),
		grid_size,
		chart_type,
		title,
		text_template,
	};
}