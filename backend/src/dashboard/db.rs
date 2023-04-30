use deadpool_postgres::Pool;
use std::error::Error;
use super::Dashboard;

pub async fn get_all_of_user(pool: &Pool, user_id: u32) -> Result<Vec<Dashboard>, Box<dyn Error>> {
	let res = pool.get()
		.await
		.unwrap()
		.query(
			"SELECT * FROM public.dashboards WHERE user_id=$1",
			&[&(user_id as i32)]
		).await?;

	return Ok(
		res.into_iter()
			.map(|x| turn_row_into_dashboard(&x))
			.collect()
	);
}

fn turn_row_into_dashboard(row: &tokio_postgres::Row) -> Dashboard {
	let id: i32 = row.get(0);
	let user_id: i32 = row.get(1);
	let name: String = row.get(2);
	let description: Option<String> = row.get(3);

	return Dashboard { 
		id: Some(id as u32),
		user_id: user_id as u32,
		name,
		description,
	};
}