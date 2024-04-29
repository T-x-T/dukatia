use deadpool_postgres::Pool;
use std::error::Error;
use uuid::Uuid;
use super::Dashboard;

pub async fn get_all_of_user(pool: &Pool, user_id: Uuid) -> Result<Vec<Dashboard>, Box<dyn Error>> {
	let res = pool.get()
		.await
		.unwrap()
		.query(
			"SELECT user_id, name, description, id FROM public.dashboards WHERE user_id=$1",
			&[&user_id]
		).await?;

	return Ok(
		res.into_iter()
			.map(|x| turn_row_into_dashboard(&x))
			.collect()
	);
}

fn turn_row_into_dashboard(row: &tokio_postgres::Row) -> Dashboard {
	let user_id: Uuid = row.get(0);
	let name: String = row.get(1);
	let description: Option<String> = row.get(2);
	let id: Uuid = row.get(3);
	
	return Dashboard { 
		id,
		user_id,
		name,
		description,
	};
}