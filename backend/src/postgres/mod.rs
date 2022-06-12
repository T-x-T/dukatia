use deadpool_postgres::{Config as PgConfig, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use std::fs;
use super::config::Config;

pub async fn get_postgres_connection(config: &Config) -> Pool {	
	if !database_exists(&config, &config.db_database).await {
		create_database(&config).await;
	}
	
	return get_pool(&config).await;
}

async fn database_exists(config: &Config, database_name: &String) -> bool {
	let mut config = config.clone();
	config.db_database = String::from("postgres");
	return get_pool(&config)
		.await
		.get()
		.await
		.unwrap()
		.query("SELECT datname FROM pg_catalog.pg_database WHERE lower(datname) = lower($1);", &[database_name])
		.await
		.expect("error trying to to figure out if database exists")
		.len() == 1;
}

async fn create_database(config: &Config) {
	let mut config_with_postgres_db = config.clone();
	config_with_postgres_db.db_database = String::from("postgres");
	get_pool(&config_with_postgres_db)
		.await
		.get()
		.await
		.unwrap()
		.query(&format!("CREATE DATABASE {} WITH OWNER = {} ENCODING = 'UTF8' CONNECTION LIMIT = -1;", config.db_database, config.db_user), &[])
		.await
		.unwrap();

	get_pool(&config)
		.await
		.get()
		.await
		.unwrap()
		.simple_query(
			&fs::read_to_string("./sql/init.sql")
			.unwrap_or_else(|_| 
				fs::read_to_string("/app/sql/init.sql")
				.expect("error trying to read init.sql")
			)
		)
		.await
		.expect("error trying to load init.sql into newly created database");
}

async fn get_pool(config: &Config) -> Pool {
	let mut cfg = PgConfig::new();
	cfg.user = Some(config.db_user.clone());
	cfg.password = Some(config.db_password.clone());
	cfg.host = Some(config.db_host.clone());
	cfg.port = Some(config.db_port);
	cfg.dbname = Some(config.db_database.clone());
	cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

	return cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
}


#[allow(dead_code)]
pub async fn delete_database(config: &Config) {
	if cfg!(test) {
		let mut config_with_postgres_db = config.clone();
		config_with_postgres_db.db_database = String::from("postgres");
		let pool = get_pool(&config_with_postgres_db).await;
		let client = pool.get().await.unwrap();
		client.query(&format!("DROP DATABASE IF EXISTS {} WITH (FORCE);", config.db_database), &[]).await.expect("error trying to remove database");
	} else {
		println!("tried to delete database while not in testing mode");
	}
}

#[allow(dead_code)]
pub async fn delete_testing_databases(config: &Config) {
	let mut config_with_postgres_db = config.clone();
	config_with_postgres_db.db_database = String::from("postgres");
	let pool = get_pool(&config_with_postgres_db).await;
	let client = pool.get().await.unwrap();
	let res = client.query("SELECT datname FROM pg_database WHERE datistemplate = false AND datname LIKE 'txts_treasury_testing%';", &[]).await.expect("error trying to get testing databases");

	for db in res {
		let db_name: String = db.get(0);
		let mut prepared_config = config.clone();
		prepared_config.db_database = db_name;
		delete_database(&prepared_config).await;
	}
}