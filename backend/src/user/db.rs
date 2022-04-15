use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::User;

pub async fn login(pool: &Pool, user: &User, hashed_secret: String) -> Result<u32, Box<dyn Error>> {
	let rows = pool.get()
		.await?
	  .query(
			"SELECT id FROM public.\"Users\" WHERE name=$1 AND secret=$2",
			&[&user.name, &hashed_secret]
		).await?;

	if rows.is_empty() {
		return Err(Box::new(CustomError::InvalidCredentials));
	}

	let user_id: i32 = rows[0].get(0);
	return Ok(user_id as u32);
}

pub async fn user_count(pool: &Pool) -> Result<u32, Box<dyn Error>> {
	let user_count: i64 = pool.get()
		.await?
		.query("SELECT COUNT(1) FROM public.\"Users\";", &[])
		.await?
	 	[0].get(0);
		 
	return Ok(user_count as u32);
}

pub async fn add(pool: &Pool, user: &User, encrypted_secret: &String) -> Result<(), Box<dyn Error>> {
	pool.get()
		.await?
		.query(
			"INSERT INTO public.\"Users\" (id, name, secret, superuser) VALUES (DEFAULT, $1, $2, $3);",
			&[&user.name, &encrypted_secret, &user.superuser]
		).await?;
		
	return Ok(());
}