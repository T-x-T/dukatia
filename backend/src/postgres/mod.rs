use deadpool_postgres::{Config as PgConfig, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use std::fs;
use super::config::Config;

pub async fn get_connection(config: &Config) -> Pool {	
	if !database_exists(config, &config.db_database).await {
		create_database(config).await;
	}

	upgrade_schema_if_necessary(config).await;
	
	return get_pool(config);
}

async fn upgrade_schema_if_necessary(config: &Config) {
	println!("Checking if database schema requires an update");

	let current_schema_version = get_schema_version(config).await;
	let newest_schema_version = get_newest_schema_version();
	println!("Current version: {current_schema_version} Newest version: {newest_schema_version}");

	if newest_schema_version > current_schema_version {
		println!("Start update");
		upgrade_schema(current_schema_version, newest_schema_version, config).await;
		println!("Finished update");
	} else {
		println!("Everything up to date!");
	}
}

async fn database_exists(config: &Config, database_name: &String) -> bool {
	let mut config = config.clone();
	config.db_database = String::from("postgres");
	return get_pool(&config)
		.get()
		.await
		.unwrap()
		.query("SELECT datname FROM pg_catalog.pg_database WHERE lower(datname) = lower($1);", &[database_name])
		.await
		.expect("error trying to to figure out if database exists")
		.len() == 1;
}

async fn get_schema_version(config: &Config) -> u32 {
	let pool = get_pool(config)
		.get()
		.await
		.unwrap();

	let table_meta_exists: bool = pool
		.query("SELECT EXISTS (SELECT FROM pg_tables WHERE schemaname = 'public' AND tablename=$1);", &[&"Meta"])
		.await
		.expect("error trying to find out if database schema is version 0")
		.first()
		.unwrap()
		.get(0);

	if table_meta_exists {
		let version: i32 = pool
			.query("SELECT schema_version FROM public.\"Meta\";", &[])
			.await
			.unwrap()
			.first()
			.unwrap()
			.get(0);
		return version as u32;
	}
	
	return 0;
}

fn get_newest_schema_version() -> u32 {
	let mut version = 0;
	
	let files = fs::read_dir("./sql").unwrap_or_else(|_| fs::read_dir("/app/sql").expect("error trying to read the sql directory"));
	
	files.for_each(|entry| {
		let file_name = entry.unwrap().file_name().into_string().unwrap();

		if file_name.starts_with("upgrade_") {
			let cur_version = file_name.replace("upgrade_", "").replace(".sql", "").parse::<u32>().unwrap();
			if cur_version > version {
				version = cur_version;
			}
		}
	});

	return version;
}

async fn upgrade_schema(current_version: u32, newest_version: u32, config: &Config) {
	let mut next_version = current_version + 1;
	let pool = get_pool(config).get().await.unwrap();
	
	while next_version <= newest_version {
		println!("Update to version {next_version}");
		pool.simple_query(
			&fs::read_to_string(format!("./sql/upgrade_{next_version}.sql"))
			.unwrap_or_else(|_| 
				fs::read_to_string(format!("/app/sql/upgrade_{next_version}.sql"))
				.expect("error trying to read upgrade sql script")
			)
		).await
		.expect("error trying to upgrade database version");

		pool.query("UPDATE public.\"Meta\" SET schema_version=$1", &[&(next_version as i32)]).await.unwrap();

		next_version += 1;
	}
}

async fn create_database(config: &Config) {
	let mut config_with_postgres_db = config.clone();
	config_with_postgres_db.db_database = String::from("postgres");
	get_pool(&config_with_postgres_db)
		.get()
		.await
		.unwrap()
		.query(&format!("CREATE DATABASE {} WITH OWNER = {} ENCODING = 'UTF8' CONNECTION LIMIT = -1;", config.db_database, config.db_user), &[])
		.await
		.unwrap();

	get_pool(config)
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

fn get_pool(config: &Config) -> Pool {
	let mut cfg = PgConfig::new();
	cfg.user = Some(config.db_user.clone());
	cfg.password = Some(config.db_password.clone());
	cfg.host = Some(config.db_host.clone());
	cfg.port = Some(config.db_port);
	cfg.dbname = Some(config.db_database.clone());
	cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
	cfg.application_name = Some("Dukatia".to_string());
	
	let mut pool_config = cfg.get_pool_config();
	pool_config.max_size = 50;
	cfg.pool = Some(pool_config);

	println!("Connecting to database {} on host {}", config.db_database, config.db_host);
	
	return cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
}