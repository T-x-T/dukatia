use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::{Transaction, TransactionStatus, Asset};

pub async fn add(pool: &Pool, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
	let id: i32 = pool.get()
		.await?
		.query(
			"INSERT INTO public.transactions (id, user_id, account_id, currency_id, recipient_id, status, timestamp, amount, comment) VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8) RETURNING id;",
			&[
				&(transaction.user_id as i32),
				&(transaction.account_id as i32),
				&(transaction.currency_id.expect("no currency_id passed into transaction::db::add") as i32),
				&(transaction.recipient_id as i32),
				&(transaction.status as i32),
				&transaction.timestamp,
				&transaction.amount,
				&transaction.comment
			])
			.await?
			[0].get(0);
	
	if transaction.tag_ids.is_some() {
		for tag_id in transaction.tag_ids.clone().unwrap() {
			pool.get()
				.await?
				.query(
					"INSERT INTO public.transaction_tags (transaction_id, tag_id) VALUES ($1, $2);",
					&[&id, &(tag_id as i32)]
				).await?;
		}
	}

	if transaction.asset.is_some() && transaction.asset.clone().unwrap().id.is_some() {
		pool.get()
			.await?
			.query(
				"INSERT INTO public.asset_transactions (transaction_id, asset_id) VALUES ($1, $2);", 
			&[&(id as i32), &(transaction.asset.clone().unwrap().id.unwrap() as i32)]
		).await?;
	}

	return Ok(());
}

pub async fn get_all(pool: &Pool) -> Result<Vec<Transaction>, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query("SELECT * from public.transaction_data", &[])
		.await?;
	
	return Ok(rows.into_iter().map(|x| turn_row_into_transaction(&x)).collect());
}

pub async fn get_by_id(pool: &Pool, transaction_id: u32) -> Result<Transaction, Box<dyn Error>> {
	let rows = pool.get()
		.await?
		.query(
			"SELECT * FROM public.transaction_data WHERE id=$1;", 
			&[&(transaction_id as i32)]
		)
		.await?;

	if rows.is_empty() {
		return Err(Box::new(CustomError::SpecifiedItemNotFound { item_type: String::from("transaction"), filter: format!("id={transaction_id}") }));
	}

	return Ok(turn_row_into_transaction(&rows[0]));
}

pub async fn update(pool: &Pool, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
	if transaction.id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("transaction") }));
	}

	get_by_id(&pool, transaction.id.unwrap()).await?;

	let client = pool.get().await?;

	client.query(
		"UPDATE public.transactions SET account_id=$1, currency_id=$2, recipient_id=$3, status=$4, timestamp=$5, amount=$6, comment=$7 WHERE id=$8;", 
		&[&(transaction.account_id as i32),
			&(transaction.currency_id.expect("no currency_id passed into transaction::db::update") as i32),
			&(transaction.recipient_id as i32),
			&(transaction.status as i32),
			&transaction.timestamp,
			&transaction.amount,
			&transaction.comment,
			&(transaction.id.unwrap() as i32)
		]
	)
	.await?;
	
	client.query(
		"DELETE FROM public.transaction_tags WHERE transaction_id=$1;",
		&[&(transaction.id.unwrap() as i32)]
	)
	.await?;

	if transaction.tag_ids.is_some() {
		for tag_id in transaction.tag_ids.clone().unwrap() {
			client.query(
				"INSERT INTO public.transaction_tags (transaction_id, tag_id) VALUES ($1, $2);",
				&[&(transaction.id.unwrap() as i32), &(tag_id as i32)]
			)
			.await?;
		}
	}

	client.query(
		"DELETE FROM public.asset_transactions WHERE transaction_id=$1",
		&[&(transaction.id.unwrap() as i32)]
	).await?;

	if transaction.asset.is_some() && transaction.asset.clone().unwrap().id.is_some() {
		client.query(
				"INSERT INTO public.asset_transactions (transaction_id, asset_id) VALUES ($1, $2);", 
			&[&(transaction.id.unwrap() as i32), &(transaction.asset.clone().unwrap().id.unwrap() as i32)]
		).await?;
	}

	return Ok(());
}

pub async fn delete_by_id(pool: &Pool, transaction_id: u32) -> Result<(), Box<dyn Error>> {
	pool.get()
		.await?
		.query(
			"DELETE FROM public.transactions WHERE id=$1;", 
			&[&(transaction_id as i32)]
		)
		.await?;
	
	return Ok(());
}

fn turn_row_into_transaction(row: &tokio_postgres::Row) -> Transaction {
	let id: i32 = row.get(0);
	let account_id: i32 = row.get(1);
	let currency_id: i32 = row.get(2);
	let recipient_id: i32 = row.get(3);
	let status: i32 = row.get(4);
	let user_id: i32 = row.get(5);
	let tag_ids: Vec<u32> = row.try_get(9)
		.unwrap_or(Vec::new())
		.into_iter()
		.map(|x: i32| x as u32)
		.collect();
	let asset_id: Option<i32> = row.get(10);
	let mut asset: Option<Asset> = None;
	if asset_id.is_some() {
		asset = Some(Asset {
			id: Some(asset_id.unwrap() as u32),
			name: row.get(11),
			description: row.get(12),
			user_id: user_id as u32,
			currency_id: currency_id as u32,
			value_per_unit: None,
			amount: None,
			tag_ids: None,
		});
	}

	return Transaction {
		id: Some(id as u32),
		user_id: user_id as u32,
		account_id: account_id as u32,
		currency_id: Some(currency_id as u32),
		recipient_id: recipient_id as u32,
		status: match status {
			0 => TransactionStatus::Withheld,
			1 => TransactionStatus::Completed,
			_ => panic!("invalid transaction status found in row from database")
		},
		timestamp: row.get(6),
		amount: row.get(7),
		comment: row.get(8),
		tag_ids: Some(tag_ids),
		asset,
	};
}