mod db;
pub mod rest_api;

use std::error::Error;
use serde::Deserialize;
use sha2::{Sha512, Digest};
use deadpool_postgres::Pool;
use super::Config;
use super::access_token;
use super::CustomError;

pub struct User {
	pub id: Option<u32>,
	pub name: String,
	pub secret: Option<String>,
	pub superuser: bool,
}

#[derive(Deserialize)]
pub struct LoginCredentials {
	pub name: String,
	pub secret: String,
}

pub async fn init(config: &Config, pool: &Pool) {
	if db::user_count(&pool).await.expect("failed to get user count in user::init") == 0 {
		let admin_user: User = User {
			id: None,
			name: config.admin_username.clone(),
			secret: Some(config.admin_password.clone()),
			superuser: true
		};

		let mut hasher = Sha512::new();
		hasher.update(format!("{}{}{}", admin_user.name, admin_user.secret.clone().unwrap(), config.pepper));
		let hashed_secret = format!("{:x}", hasher.finalize());

		db::add(&pool, &admin_user, &hashed_secret).await.expect("failed to add initial admin user to db");
	}
}

pub async fn login(config: &Config, pool: &Pool, credentials: LoginCredentials) -> Result<String, Box<dyn Error>> {
	let hashed_secret = create_hash(format!("{}{}{}", credentials.name, credentials.secret, config.pepper));
	
	let user = User {
		id: Some(
			db::login(&pool, &credentials, hashed_secret).await?
		),
		name: credentials.name,
		secret: Some(credentials.secret),
		superuser: false
	};
	
	return access_token::add(&pool, &user).await;
}

pub async fn update_secret(config: &Config, pool: &Pool, old_secret: String, new_secret: String, input_user_id: u32) -> Result<(), Box<dyn Error>> {
	let user_from_db = db::get_by_id(&pool, &input_user_id).await?;

	let old_hashed_secret = create_hash(format!("{}{}{}", user_from_db.name, old_secret, config.pepper));
	let user_id = db::login(&pool, &LoginCredentials { name: user_from_db.name.clone(), secret: old_secret }, old_hashed_secret).await?;
	if user_id != input_user_id {
		return Err(Box::new(CustomError::InvalidItem { reason: String::from("trying to update the wrong user") }));
	}

	let new_hashed_secret = create_hash(format!("{}{}{}", user_from_db.name, new_secret, config.pepper));
	db::update_secret(&pool, user_id, new_hashed_secret).await?;

	return Ok(());
}

fn create_hash(value_to_hash: String) -> String {
	let mut hasher = Sha512::new();
	hasher.update(value_to_hash);
	return format!("{:x}", hasher.finalize());
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::{setup, teardown};

	#[tokio::test(flavor = "multi_thread")]
	async fn can_login_correct_admin_user() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		let credentials = LoginCredentials {
			name: config.admin_username.clone(),
			secret: config.admin_password.clone(),
		};

		login(&config, &pool, credentials).await?;

		teardown(&config).await;
		return Ok(());
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn cant_login_admin_user_with_wrong_username() -> Result<(), String> {
		let (config, pool) = setup().await;

		let credentials = LoginCredentials {
			name: format!("{}{}", config.admin_username, "a"),
			secret: config.admin_password.clone(),
		};

		let res = match login(&config, &pool, credentials).await {
			Ok(_) => Err(String::from("managed to log in admin user with wrong username")),
			Err(_) => Ok(()),
		};

		teardown(&config).await;
		return res;
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn cant_login_admin_user_with_wrong_password() -> Result<(), String> {
		let (config, pool) = setup().await;

		let credentials = LoginCredentials {
			name: config.admin_username.clone(),
			secret: format!("{}{}", config.admin_password, "a"),
		};

		let res = match login(&config, &pool, credentials).await {
			Ok(_) => Err(String::from("managed to log in admin user with wrong password")),
			Err(_) => Ok(()),
		};
		
		teardown(&config).await;
		return res;
	}
}