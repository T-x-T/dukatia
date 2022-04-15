use std::env;
use rand::{Rng, thread_rng};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct Config {
	pub db_user: String,
	pub db_password: String,
	pub db_host: String,
	pub db_port: u16,
	pub db_database: String,
	pub admin_username: String,
	pub admin_password: String,
	pub pepper: String,
	pub api_port: u16,
}

pub fn initialize_config() -> Config {
	if cfg!(test) {
		let mut rng = thread_rng();
		return Config {
			db_user: 				env::var("POSTGRES_USER").unwrap_or(String::from("postgres")),
			db_password: 		env::var("POSTGRES_PASSWORD").unwrap_or(String::from("password")),
			db_host: 				env::var("POSTGRES_HOST").unwrap_or(String::from("127.0.0.1")),
			db_port: 				env::var("POSTGRES_PORT").unwrap_or(String::from("5432")).parse::<u16>().unwrap(),
			db_database: 		env::var("DATABASE_NAME").unwrap_or(format!("txts_treasury_testing_{}", Utc::now().timestamp_millis() * rng.gen_range(1..1000))),
			admin_username: env::var("ADMIN_USERNAME").unwrap_or(String::from("admin")),
			admin_password: env::var("ADMIN_PASSWORD").unwrap_or(String::from("password")),
			pepper: 				env::var("pepper").unwrap_or(String::from("supersecret")),
			api_port: 			env::var("PORT").unwrap_or(String::from("4000")).parse::<u16>().unwrap(),
		}
	} else {
		return Config {
			db_user: 				env::var("POSTGRES_USER").unwrap_or(String::from("postgres")),
			db_password: 		env::var("POSTGRES_PASSWORD").unwrap_or(String::from("password")),
			db_host: 				env::var("POSTGRES_HOST").unwrap_or(String::from("127.0.0.1")),
			db_port: 				env::var("POSTGRES_PORT").unwrap_or(String::from("5432")).parse::<u16>().unwrap(),
			db_database: 		env::var("DATABASE_NAME").unwrap_or(String::from("txts_treasury_staging")),
			admin_username: env::var("ADMIN_USERNAME").unwrap_or(String::from("admin")),
			admin_password: env::var("ADMIN_PASSWORD").unwrap_or(String::from("password")),
			pepper: 				env::var("pepper").unwrap_or(String::from("supersecret")),
			api_port: 			env::var("PORT").unwrap_or(String::from("4000")).parse::<u16>().unwrap(),
		}
	}
}