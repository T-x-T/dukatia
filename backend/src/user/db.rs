use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::{User, LoginCredentials};
use crate::traits::*;

#[derive(Debug)]
pub struct UserDbReader<'a> {
	query_parameters: QueryParameters,
	pool: &'a Pool,
}

impl<'a> DbReader<'a, User> for UserDbReader<'a> {
	fn new(pool: &'a Pool) -> Self {
		return Self {
			query_parameters: QueryParameters::default(),
			pool,
		}
	}

	fn get_pool(&self) -> &Pool {
		return self.pool;
	}

	fn get_query_parameters(&self) -> &QueryParameters {
		return &self.query_parameters;
	}

	fn set_query_parameters(mut self, query_parameters: QueryParameters) -> Self {
		self.query_parameters = query_parameters;
		return self;
	}

	async fn execute(self) -> Result<Vec<User>, Box<dyn Error>> {
		let query = "SELECT id, name, superuser FROM public.users";
		return Ok(
			self.actually_execute(query)
				.await?
				.into_iter()
				.map(Into::into)
				.collect()
		);
	}
}

impl<'a> UserDbReader<'a> {
	pub async fn get_first_with_encrypted_secret(self) -> Result<User, Box<dyn Error>> {
		let query = "SELECT id, name, superuser, secret FROM public.users";
		let users: Vec<User> = self.actually_execute(query)
			.await?
			.into_iter()
			.map(Into::into)
			.collect();
			
		if users.is_empty() {
			return Err(Box::new(CustomError::NoItemFound { item_type: "user".to_string() }))
		}
		
		return Ok(users[0].clone());
	}
}

#[derive(Debug)]
pub struct UserDbWriter<'a> {
	pool: &'a Pool,
	user: User,
}

impl<'a> DbWriter<'a, User> for UserDbWriter<'a> {
	fn new(pool: &'a Pool, item: User) -> Self {
		return Self {
			pool,
			user: item,
		}
	}

	async fn insert(self) -> Result<u32, Box<dyn Error>> {
		let client = self.pool.get().await.unwrap();
		let id: i32 = client
			.query(
				"INSERT INTO public.users (id, name, secret, superuser) VALUES (DEFAULT, $1, $2, $3) RETURNING id;", 
				&[&self.user.name, &self.user.encrypted_secret, &self.user.superuser]
			).await?
			[0].get(0);

		client
			.query(
				"INSERT INTO public.dashboards(id, user_id, name, description) VALUES (DEFAULT, $1, 'Default', 'The default Dashboard');",
				&[&id]
			).await?;

		return Ok(id as u32);
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		if self.user.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty{property: String::from("id"), item_type: String::from("user")}));
		}

		super::UserLoader::new(self.pool).get().await?;
	
		let client = self.pool.get().await?;
		
		client
			.query(
				"UPDATE public.users SET name=$1, secret=$2, superuser=$3 WHERE id=$4;",
				&[&self.user.name, &self.user.encrypted_secret, &self.user.superuser, &(self.user.id.unwrap() as i32)]
			)
			.await?;

		return Ok(());
	}
}

pub async fn login(pool: &Pool, credentials: &LoginCredentials, hashed_secret: String) -> Result<u32, Box<dyn Error>> {
	let rows = pool.get()
		.await?
	  .query(
			"SELECT id FROM public.users WHERE name=$1 AND secret=$2",
			&[&credentials.name, &hashed_secret]
		).await?;

	if rows.is_empty() {
		return Err(Box::new(CustomError::InvalidCredentials));
	}

	let user_id: i32 = rows[0].get(0);
	return Ok(user_id as u32);
}

pub async fn update_secret(pool: &Pool, user_id: u32, new_hashed_secret: String) -> Result<(), Box<dyn Error>> {
	pool.get()
		.await?
		.query(
			"UPDATE public.users SET secret=$1 WHERE id=$2", 
		&[&new_hashed_secret, &(user_id as i32)]
		).await?;

	return Ok(());
}

pub async fn get_by_id(pool: &Pool, id: &u32) -> Result<User, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query(
			"SELECT name, superuser FROM public.users WHERE id=$1",
			&[&(*id as i32)]
		).await?;

	return Ok(User {
		id: Some(*id),
		name: rows[0].get(0),
		secret: None,
		encrypted_secret: None,
		superuser: rows[0].get(1)
	});
}

impl From<tokio_postgres::Row> for User {
	fn from(value: tokio_postgres::Row) -> Self {
		let id: i32 = value.get(0);
		let name: String = value.get(1);
		let superuser: bool = value.get(2);
		let encrypted_secret: Option<String> = value.try_get(3).unwrap_or_default();

		return User {
			id: Some(id as u32),
			name,
			secret: None,
			encrypted_secret,
			superuser,
		};
	}
}