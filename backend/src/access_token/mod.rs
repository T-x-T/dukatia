mod db;

use std::error::Error;
use deadpool_postgres::Pool;
use sha2::{Sha512, Digest};
use chrono::Utc;
use rand::Rng;
use super::user::User;

pub async fn add(pool: &Pool, user: &User) -> Result<String, Box<dyn Error>> {
	let mut hasher = Sha512::new();
	hasher.update(
		format!(
			"{}{}{}", 
			user.name, 
			user.secret.clone().unwrap(), 
			Utc::now().timestamp_millis() * rand::thread_rng().gen_range(2..8192)
		)
	);
	let access_token = format!("{:x}", hasher.finalize());
	db::add(pool, user.id.expect("user id is not allowed to be None here"), &access_token).await?;
	return Ok(access_token);
}

pub async fn get_user_of_token(pool: &Pool, access_token: &String, session_expiry_days: u32) -> Result<u32, Box<dyn Error>> {
	return db::get_user_of_token(pool, access_token, session_expiry_days).await;
}

pub async fn delete_token(pool: &Pool, user_id: u32, access_token: &String) -> Result<(), Box<dyn Error>> {
	return db::delete_token(pool, user_id, access_token).await;
}