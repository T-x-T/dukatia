mod db;
pub mod rest_api;

use std::error::Error;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Sha512, Digest};
use deadpool_postgres::Pool;
use uuid::Uuid;
use super::Config;
use super::access_token;
use super::CustomError;
use crate::traits::*;

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct User {
	pub id: Uuid,
	pub name: String,
	pub secret: Option<String>,
	pub encrypted_secret: Option<String>,
	pub superuser: bool,
	pub active: bool,
	pub last_logon: Option<DateTime<Utc>>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct LoginResult {
	pub user_id: Uuid,
	pub first_login: bool,
	pub access_token: Option<String>,
}

impl Default for User {
	fn default() -> Self {
		return Self {
			id: Uuid::new_v4(),
			name: String::new(),
			secret: None,
			encrypted_secret: None,
			superuser: false,
			active: true,
			last_logon: None,
		};
	}
}

#[derive(Deserialize)]
pub struct LoginCredentials {
	pub name: String,
	pub secret: String,
}

impl Create for User {
	async fn create(self, pool: &Pool) -> Result<Uuid, Box<dyn Error>> {
		return db::UserDbWriter::new(pool, self).insert().await;
	}
}

impl Update for User {
	async fn update(self, pool: &Pool) -> Result<(), Box<dyn Error>> {
		return db::UserDbWriter::new(pool, self).replace().await;
	}
}

impl User {
	pub fn set_id(mut self, id: Uuid) -> Self {
		self.id = id;
		return self;
	}

	pub fn set_name(mut self, name: String) -> Self {
		self.name = name;
		return self;
	}

	#[allow(unused)]
	pub fn set_secret(mut self, secret: String) -> Self {
		self.secret = Some(secret);
		return self;
	}

	pub fn set_secret_mut(&mut self, secret: String) {
		self.secret = Some(secret);
	}

	#[allow(unused)]
	pub fn set_secret_opt(mut self, secret: Option<String>) -> Self {
		self.secret = secret;
		return self;
	}

	pub fn set_encrypted_secret(mut self, encrypted_secret: String) -> Self {
		self.encrypted_secret = Some(encrypted_secret);
		return self;
	}

	#[allow(unused)]
	pub fn set_encrypted_secret_opt(mut self, encrypted_secret: Option<String>) -> Self {
		self.encrypted_secret = encrypted_secret;
		return self;
	}

	pub fn set_superuser(mut self, superuser: bool) -> Self {
		self.superuser = superuser;
		return self;
	}

	pub fn set_superuser_mut(&mut self, superuser: bool) {
		self.superuser = superuser;
	}

	#[allow(unused)]
	pub fn set_active(mut self, active: bool) -> Self {
		self.active = active;
		return self;
	}

	pub fn set_active_mut(&mut self, active: bool) {
		self.active = active;
	}

	pub fn encrypt_secret(mut self, pepper: &str) -> Self {
		self.encrypted_secret = Some(create_hash(format!("{}{}{}", self.name, self.secret.clone().unwrap_or_default(), pepper)));
		return self;
	}

	pub fn encrypt_secret_mut(&mut self, pepper: &str) {
		self.encrypted_secret = Some(create_hash(format!("{}{}{}", self.name, self.secret.clone().unwrap_or_default(), pepper)));
	}

	pub async fn update_secret(self, pool: &Pool, pepper: &str, old_secret: String, new_secret: String) -> Result<(), Box<dyn Error>> {
		let user_from_db = db::get_by_id(pool, &self.id).await?;
	
		let old_hashed_secret = create_hash(format!("{}{}{}", user_from_db.name, old_secret, pepper));
		let user_id = db::login(pool, &LoginCredentials { name: user_from_db.name.clone(), secret: old_secret }, old_hashed_secret).await?.user_id;
		if user_id != self.id {
			return Err(Box::new(CustomError::InvalidItem { reason: String::from("trying to update the wrong user") }));
		}
	
		let new_hashed_secret = create_hash(format!("{}{}{}", user_from_db.name, new_secret, pepper));
		db::update_secret(pool, user_id, new_hashed_secret).await?;
	
		return Ok(());
	}

	pub async fn logout(self, pool: &Pool, access_token: String) -> Result<(), Box<dyn Error>> {
		return access_token::delete_token(pool, self.id, &access_token).await;
	}
}

#[derive(Debug, Clone)]
pub struct UserLoader<'a> {
	pool: &'a Pool,
	query_parameters: QueryParameters,
}

impl<'a> Loader<'a, User> for UserLoader<'a> {
	fn new(pool: &'a Pool) -> Self {
		Self {
			pool,
			query_parameters: QueryParameters::default(),
		}
	}

	async fn get(self) -> Result<Vec<User>, Box<dyn Error>> {
		return db::UserDbReader::new(self.pool)
			.set_query_parameters(self.query_parameters)
			.execute()
			.await;
	}

	fn get_query_parameters(&self) -> &QueryParameters {
		return &self.query_parameters;
	}

	fn set_query_parameters(mut self, query_parameters: QueryParameters) -> Self {
		self.query_parameters = query_parameters;
		return self;
	}
}

pub async fn init(config: &Config, pool: &Pool) {
	let user = UserLoader::new(pool).get_first().await.expect("failed to get dummy user in user::init");
	let hashed_secret = create_hash(format!("{}{}{}", config.admin_username.clone(), config.admin_password.clone(), config.pepper));

	User::default()
		.set_id(user.id)
		.set_name(config.admin_username.clone())
		.set_encrypted_secret(hashed_secret)
		.set_superuser(true)
		.update(pool)
		.await
		.expect("failed to add initial admin user to db");

	db::insert_charts_and_stuff(pool, &user).await.expect("failed to insert charts and stuff for admin user");
}

pub async fn login(config: &Config, pool: &Pool, credentials: LoginCredentials) -> Result<LoginResult, Box<dyn Error>> {
	let hashed_secret = create_hash(format!("{}{}{}", credentials.name, credentials.secret, config.pepper));
	let user = UserLoader::new(pool)
		.set_filter_name(credentials.name.clone(), StringFilterModes::Exact)
		.get_first().await?;

	let user = User {
		id: user.id,
		name: user.name,
		secret: Some(credentials.secret.clone()),
		..Default::default()
	};

	let login_result = db::login(pool, &credentials, hashed_secret).await?;
	
	#[allow(clippy::needless_question_mark)]
	return Ok(LoginResult {
		access_token: Some(access_token::add(pool, &user).await?),
		..login_result
	});
}

fn create_hash(value_to_hash: String) -> String {
	let mut hasher = Sha512::new();
	hasher.update(value_to_hash);
	return format!("{:x}", hasher.finalize());
}