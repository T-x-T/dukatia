use std::env;

#[derive(Debug, Clone, Default)]
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
	pub session_expiry_days: u32,
}

pub fn initialize_config() -> Config {
	return Config {
		db_user: 									env::var("POSTGRES_USER").unwrap_or(String::from("postgres")),
		db_password: 							env::var("POSTGRES_PASSWORD").unwrap_or(String::from("password")),
		db_host: 									env::var("POSTGRES_HOST").unwrap_or(String::from("127.0.0.1")),
		db_port: 									env::var("POSTGRES_PORT").unwrap_or(String::from("5432")).parse::<u16>().unwrap(),
		db_database: 							env::var("DATABASE_NAME").unwrap_or(String::from("txts_treasury_staging")),
		admin_username: 					env::var("ADMIN_USERNAME").unwrap_or(String::from("admin")),
		admin_password: 					env::var("ADMIN_PASSWORD").unwrap_or(String::from("password")),
		pepper: 									env::var("PEPPER").unwrap_or(String::from("supersecret")),
		api_port: 								env::var("PORT").unwrap_or(String::from("4000")).parse::<u16>().unwrap_or(4000),
		session_expiry_days: 			env::var("SESSION_EXPIRY_DAYS").unwrap_or(String::from("30")).parse::<u32>().unwrap_or(30),
	}
}