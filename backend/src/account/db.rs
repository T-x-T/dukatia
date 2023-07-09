use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::{Account, DeepAccount};

pub async fn add(pool: &Pool, account: &Account) -> Result<(), Box<dyn Error>> {
	let client = pool.get().await?;
	let id: i32 = client
		.query(
			"INSERT INTO public.accounts (id, name, default_currency_id, user_id) VALUES (DEFAULT, $1, $2, $3) RETURNING id;",
			&[&account.name, &(account.default_currency_id as i32), &(account.user_id as i32)]
		)
		.await?
	 	[0].get(0);
		 
	if account.tag_ids.is_some() {
		for tag_id in account.clone().tag_ids.unwrap() {
			client.query("INSERT INTO public.account_tags (account_id, tag_id) VALUES ($1, $2);", &[&id, &(tag_id as i32)]).await?;
		}
	}
	return Ok(());
}

pub async fn get_all(pool: &Pool) -> Result<Vec<Account>, Box<dyn Error>> {
	return Ok(
		pool.get()
		.await?
		.query(
			"SELECT * FROM public.account_data;",
			&[]
		).await?
		.iter()
		.map(turn_row_into_account)
		.collect()
	);
}

pub async fn get_all_deep(pool: &Pool) -> Result<Vec<DeepAccount>, Box<dyn Error>> {
	return Ok(
		pool.get()
			.await?
			.query(
				"SELECT * FROM deep_accounts;",
				&[]
			).await?
			.iter()
			.map(turn_row_into_deep_account)
			.collect()
	);
}

pub async fn get_by_id(pool: &Pool, account_id: u32) -> Result<Account, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query(
			"SELECT * FROM public.account_data WHERE id=$1;",
			&[&(account_id as i32)]
		)
		.await?;

	if rows.is_empty() {
		return Err(Box::new(CustomError::SpecifiedItemNotFound { item_type: String::from("account"), filter: format!("id={account_id}") }));
	}

	return Ok(turn_row_into_account(&rows[0]));
}

pub async fn update(pool: &Pool, account: &Account) -> Result<(), Box<dyn Error>> {
	if account.id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("account") }));
	}

	get_by_id(pool, account.id.unwrap()).await?;

	let client = pool.get().await?;

	client.query(
		"UPDATE public.accounts SET name=$1, default_currency_id=$2 WHERE id=$3;",
		&[&account.name, &(account.default_currency_id as i32), &(account.id.unwrap() as i32)]
	)
	.await?;

	client.query(
		"DELETE FROM public.account_tags WHERE account_id=$1;",
		&[&(account.id.unwrap() as i32)]
	)
	.await?;

	if account.tag_ids.is_some() {
		for tag_id in account.tag_ids.clone().unwrap() {
			client.query(
				"INSERT INTO public.account_tags (account_id, tag_id) VALUES ($1, $2);",
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

fn turn_row_into_deep_account(row: &tokio_postgres::Row) -> DeepAccount {
	let id: i32 = row.get(0);
	let name: String = row.get(1);
	let default_currency_id: i32 = row.get(2);
	let default_currency_minor_in_mayor: i32 = row.get(3);
	let default_currency_name: String = row.get(4);
	let default_currency_symbol: String = row.get(5);
	let user_id: Option<i32> = row.get(6);
	let user_name: Option<String> = row.get(7);
	let user_superuser: Option<bool> = row.get(8);
	let tag_ids: Vec<Option<i32>> = row.get(9);
	let tag_names: Vec<Option<String>> = row.get(10);
	let tag_parent_ids: Vec<Option<i32>> = row.get(11);
	let tag_parent_names: Vec<Option<String>> = row.get(12);
	let tag_parent_parent_ids: Vec<Option<i32>> = row.get(13);
	let tag_parent_user_ids: Vec<Option<i32>> = row.get(14);
	let tag_user_ids: Vec<Option<i32>> = row.get(15);
	let tag_user_names: Vec<Option<String>> = row.get(16);
	let tag_user_superusers: Vec<Option<bool>> = row.get(17);

	let default_currency = crate::currency::Currency {
		id: Some(default_currency_id as u32),
		name: default_currency_name,
		minor_in_mayor: default_currency_minor_in_mayor as u32,
		symbol: default_currency_symbol
	};

	let user = user_id.map(|_| crate::user::User {
		id: Some(user_id.unwrap() as u32),
		name: user_name.unwrap(),
		secret: None,
		superuser: user_superuser.unwrap(),
	});
	
	let tags: Vec<crate::tag::DeepTag> = tag_ids
		.into_iter()
		.filter(Option::is_some)
		.enumerate()
		.map(|(i, tag_id)| {
			let parent: Option<crate::tag::Tag> = match tag_parent_ids.get(i) {
				Some(x) => {
					x.as_ref().map(|_| crate::tag::Tag {
						id: tag_parent_ids[i].map(|x| x as u32),
						name: tag_parent_names[i].clone().unwrap(),
						user_id: tag_parent_user_ids[i].unwrap() as u32,
						parent_id: tag_parent_parent_ids[i].map(|x| x as u32),
					})
				},
				None => None,
			};
			
			crate::tag::DeepTag {
				id: tag_id.unwrap() as u32,
				name: tag_names[i].clone().unwrap(),
				user: crate::user::User {
					id: tag_user_ids[i].map(|x| x as u32),
					name: tag_user_names[i].clone().unwrap(),
					secret: None,
					superuser: tag_user_superusers[i].unwrap(),
				},
				parent,
			}
		}).collect();
		

	return DeepAccount {
		id: id as u32,
		name,
		default_currency,
		user,
		tags,
	}
}