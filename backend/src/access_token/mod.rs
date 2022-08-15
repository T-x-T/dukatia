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
	db::add(&pool, user.id.expect("user id is not allowed to be None here"), &access_token).await?;
	return Ok(access_token);
}

pub async fn get_user_of_token(pool: &Pool, access_token: &String) -> Result<u32, Box<dyn Error>> {
	return db::get_user_of_token(&pool, &access_token).await;
}

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::config::Config;
	use super::super::{setup, teardown};

	fn get_user(config: &Config) -> User {
		return User {
			id: Some(0),
			name: config.admin_username.clone(),
			secret: Some(config.admin_password.clone()),
			superuser: true,
		};
	}

	mod add_access_token {
		use super::*;

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic() -> Result<(), Box<dyn Error>> {
			let(config, pool) = setup().await;
		
			add(&pool, &get_user(&config)).await?;

			teardown(&config).await;
			return Ok(());
		}
	
		#[tokio::test(flavor = "multi_thread")]
		async fn return_access_token_of_correct_length() -> Result<(), Box<dyn Error>> {
			let(config, pool) = setup().await;
		
			let res = add(&pool, &get_user(&config)).await?;
			assert_eq!(res.len(), 128);

			teardown(&config).await;
			return Ok(());
		}
	
		#[tokio::test(flavor = "multi_thread")]
		#[should_panic]
		async fn using_user_with_no_id_should_panic() {
			let(config, pool) = setup().await;
	
			let mut user = get_user(&config);
			user.id = None;
			let _ = add(&pool, &user).await;

			teardown(&config).await;
			return ();
		}
	}

	mod get_user_of_token {
		use super::*;

		async fn get_access_token(config: &Config, pool: &Pool) -> Result<String, Box<dyn Error>> {
			return add(&pool, &get_user(&config)).await;
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn doesnt_panic() -> Result<(), Box<dyn Error>> {
			let(config, pool) = setup().await;
	
			get_user_of_token(&pool, &get_access_token(&config, &pool).await?).await?;

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_id_of_only_user() -> Result<(), Box<dyn Error>> {
			let(config, pool) = setup().await;
	
			let user_id = get_user_of_token(&pool, &get_access_token(&config, &pool).await?).await?;
			assert_eq!(user_id, 0);

			teardown(&config).await;
			return Ok(());
		}

		#[tokio::test(flavor = "multi_thread")]
		async fn returns_error_with_non_existent_token() -> Result<(), Box<dyn Error>> {
			let(config, pool) = setup().await;
	
			let res = get_user_of_token(&pool, &String::from("nonsense")).await;
			assert_eq!(res.expect_err("specified item of type user not found with filter access_token").to_string(), String::from("specified item of type user not found with filter access_token"));
			
			teardown(&config).await;
			return Ok(());
		}
	}
}