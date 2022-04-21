mod db;
pub mod rest_api;

use std::error::Error;
use sha2::{Sha512, Digest};
use deadpool_postgres::Pool;
use super::Config;
use super::access_token;

pub struct User {
	pub id: Option<u32>,
	pub name: String,
	pub secret: String,
	pub superuser: bool,
}

pub async fn init(config: &Config, pool: &Pool) {
	if db::user_count(&pool).await.expect("failed to get user count in user::init") == 0 {
		let admin_user: User = User {
			id: None,
			name: config.admin_username.clone(),
			secret: config.admin_password.clone(),
			superuser: true
		};

		let mut hasher = Sha512::new();
		hasher.update(format!("{}{}{}", admin_user.name, admin_user.secret, config.pepper));
		let hashed_secret = format!("{:x}", hasher.finalize());

		db::add(&pool, &admin_user, &hashed_secret).await.expect("failed to add initial admin user to db");
	}
}

pub async fn login(config: &Config, pool: &Pool, mut user: User) -> Result<String, Box<dyn Error>> {
	let mut hasher = Sha512::new();
	hasher.update(format!("{}{}{}", user.name, user.secret, config.pepper));
	let hashed_secret = format!("{:x}", hasher.finalize());
	
	user.id = Some(
		db::login(&pool, &user, hashed_secret).await?
	);
	
	return access_token::add(&pool, &user).await;
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::{setup, teardown};

	#[tokio::test(flavor = "multi_thread")]
	async fn can_login_correct_admin_user() -> Result<(), Box<dyn Error>> {
		let (config, pool) = setup().await;

		let user = User {
			id: None,
			name: config.admin_username.clone(),
			secret: config.admin_password.clone(),
			superuser: true,
		};

		login(&config, &pool, user).await?;

		teardown(&config).await;
		return Ok(());
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn cant_login_admin_user_with_wrong_username() -> Result<(), String> {
		let (config, pool) = setup().await;

		let user = User {
			id: None,
			name: format!("{}{}", config.admin_username, "a"),
			secret: config.admin_password.clone(),
			superuser: true,
		};

		let res = match login(&config, &pool, user).await {
			Ok(_) => Err(String::from("managed to log in admin user with wrong username")),
			Err(_) => Ok(()),
		};

		teardown(&config).await;
		return res;
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn cant_login_admin_user_with_wrong_password() -> Result<(), String> {
		let (config, pool) = setup().await;

		let user = User {
			id: None,
			name: config.admin_username.clone(),
			secret: format!("{}{}", config.admin_password, "a"),
			superuser: true,
		};

		let res = match login(&config, &pool, user).await {
			Ok(_) => Err(String::from("managed to log in admin user with wrong password")),
			Err(_) => Ok(()),
		};
		
		teardown(&config).await;
		return res;
	}
}