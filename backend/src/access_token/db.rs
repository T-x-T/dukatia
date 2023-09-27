use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;

pub async fn add(pool: &Pool, user_id: u32, access_token: &String) -> Result<(), Box<dyn Error>> {
	pool.get()
		.await?
		.query(
			"INSERT INTO public.access_tokens (id, user_id, token) VALUES (DEFAULT, $1, $2);",
			&[&(user_id as i32), &access_token]
		)
		.await?;
	return Ok(());
}

pub async fn get_user_of_token(pool: &Pool, access_token: &String, session_expiry_days: u32) -> Result<u32, Box<dyn Error>> {
	let res = pool.get().await?
		.query("SELECT user_id FROM public.access_tokens WHERE token=$1 AND created_at >= NOW() - ($2 || ' days')::interval;", &[access_token, &(session_expiry_days.to_string())])
		.await?;

	if res.len() != 1 {
		return Err(Box::new(CustomError::SpecifiedItemNotFound{item_type: String::from("user"), filter: String::from("access_token")}));
	}

	let user_id: i32 = res[0].get(0);
	return Ok(user_id as u32);
}

pub async fn delete_token(pool: &Pool, user_id: u32, access_token: &String) -> Result<(), Box<dyn Error>> {
	let res = pool
		.get()
		.await?
		.query("DELETE FROM public.access_tokens WHERE user_id=$1 AND token=$2 RETURNING *", &[&(user_id as i32), &access_token])
		.await?;

	if res.len() != 1 {
		return Err(Box::new(CustomError::SpecifiedItemNotFound{item_type: String::from("access_token"), filter: String::new()}));
	}

	return Ok(());
}