mod db;
pub mod rest_api;

use serde::Serialize;
use std::error::Error;
use deadpool_postgres::Pool;

#[derive(Debug, Clone, Serialize)]
pub struct Dashboard {
	pub id: Option<u32>,
	pub user_id: u32,
	pub name: String,
	pub description: Option<String>,
}

pub async fn get_all_of_user(pool: &Pool, user_id: u32) -> Result<Vec<Dashboard>, Box<dyn Error>> {
	return db::get_all_of_user(pool, user_id).await;
}
