use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;

pub async fn add(pool: &Pool, user_id: u32, access_token: &String) -> Result<(), Box<dyn Error>> {
	pool.get()
		.await?
		.query(
			"INSERT INTO public.\"AccessTokens\" (id, \"user\", token) VALUES (DEFAULT, $1, $2);",
			&[&(user_id as i32), &access_token]
		)
		.await?;
	return Ok(());
}

pub async fn get_user_of_token(pool: &Pool, access_token: &String) -> Result<u32, Box<dyn Error>> {
	let res = pool.get().await?
		.query("SELECT \"user\" FROM public.\"AccessTokens\" WHERE token=$1;", &[access_token])
		.await?;

	if res.len() != 1 {
		return Err(Box::new(CustomError::SpecifiedItemNotFound{item_type: String::from("user"), filter: String::from("access_token")}));
	}

	let user_id: i32 = res[0].get(0);
	return Ok(user_id as u32);
}