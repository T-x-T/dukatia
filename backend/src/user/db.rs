use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::{User, LoginCredentials};

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

pub async fn user_count(pool: &Pool) -> Result<u32, Box<dyn Error>> {
	let user_count: i64 = pool.get()
		.await?
		.query("SELECT COUNT(1) FROM public.users;", &[])
		.await?
	 	[0].get(0);
		 
	return Ok(user_count as u32);
}

pub async fn add(pool: &Pool, user: &User, encrypted_secret: &String) -> Result<(), Box<dyn Error>> {
	pool.get()
		.await?
		.query(
			"INSERT INTO public.users (id, name, secret, superuser) VALUES (DEFAULT, $1, $2, $3);",
			&[&user.name, &encrypted_secret, &user.superuser]
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
		superuser: rows[0].get(1)
	});
}