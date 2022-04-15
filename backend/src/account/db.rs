use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::Account;

pub async fn add(pool: &Pool, account: &Account) -> Result<(), Box<dyn Error>> {
	let client = pool.get().await?;
	let id: i32 = client
		.query(
			"INSERT INTO public.\"Accounts\" (id, name, defaultcurrency, \"user\") VALUES (DEFAULT, $1, $2, $3) RETURNING id;",
			&[&account.name, &(account.default_currency_id as i32), &(account.user_id as i32)]
		)
		.await?
	 	[0].get(0);
		 
	if account.tag_ids.is_some() {
		for tag_id in account.clone().tag_ids.unwrap() {
			client.query("INSERT INTO public.\"AccountTags\" (account, tag) VALUES ($1, $2);", &[&id, &(tag_id as i32)]).await?;
		}
	}
	return Ok(());
}

pub async fn get_all(pool: &Pool) -> Result<Vec<Account>, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query(
			"SELECT a.id, a.name, a.defaultcurrency, a.user, array_agg(t.tag) as tags FROM public.\"Accounts\" a LEFT JOIN public.\"AccountTags\" t ON a.id = t.account GROUP BY a.id;",
			&[]
		)
		.await?;
	
	if rows.is_empty() {
		return Err(Box::new(CustomError::NoItemFound{item_type: String::from("account")}));
	}

	return Ok(
		rows
			.iter()
			.map(|x| turn_row_into_account(&x))
			.collect()
	);
}

pub async fn get_by_id(pool: &Pool, account_id: u32) -> Result<Account, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query(
			"SELECT a.id, a.name, a.defaultcurrency, a.user, array_agg(t.tag) as tags FROM public.\"Accounts\" a LEFT JOIN public.\"AccountTags\" t ON a.id = t.account WHERE a.id=$1 GROUP BY a.id;",
			&[&(account_id as i32)]
		)
		.await?;

	if rows.is_empty() {
		return Err(Box::new(CustomError::SpecifiedItemNotFound { item_type: String::from("account"), filter: format!("id={}", account_id) }));
	}

	return Ok(turn_row_into_account(&rows[0]));
}

pub async fn update(pool: &Pool, account: &Account) -> Result<(), Box<dyn Error>> {
	if account.id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("account") }));
	}

	get_by_id(&pool, account.id.unwrap()).await?;

	let client = pool.get().await?;

	client.query(
		"UPDATE public.\"Accounts\" SET name=$1, defaultcurrency=$2 WHERE id=$3;",
		&[&account.name, &(account.default_currency_id as i32), &(account.id.unwrap() as i32)]
	)
	.await?;

	client.query(
		"DELETE FROM public.\"AccountTags\" WHERE account=$1;",
		&[&(account.id.unwrap() as i32)]
	)
	.await?;

	if account.tag_ids.is_some() {
		for tag_id in account.tag_ids.clone().unwrap() {
			client.query(
				"INSERT INTO public.\"AccountTags\" (account, tag) VALUES ($1, $2);",
				&[&(account.id.unwrap() as i32), &(tag_id as i32)]
			)
			.await?;
		}
	}

	return Ok(());
}

fn turn_row_into_account(row: &tokio_postgres::Row) -> Account {
	let id: i32 = row.get(0);
	let default_currency_id: i32 = row.get(2);
	let user_id: i32 = row.try_get(3).unwrap_or(0);
	let tag_ids: Vec<u32> = row
		.try_get(4)
		.unwrap_or(Vec::new())
		.into_iter()
		.map(|x: i32| x as u32)
		.collect();

	return Account {
		id: Some(id as u32),
		name: row.get(1),
		default_currency_id: default_currency_id as u32,
		user_id: user_id as u32,
		tag_ids: Some(tag_ids)
	};
}