use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::{Transaction, TransactionStatus, Asset, DeepTransaction, Position};

pub async fn add(pool: &Pool, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
	let transaction_id: i32 = pool.get()
		.await?
		.query(
			"INSERT INTO public.transactions (id, user_id, account_id, currency_id, recipient_id, status, timestamp, comment) VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7) RETURNING id;",
			&[
				&(transaction.user_id as i32),
				&(transaction.account_id as i32),
				&(transaction.currency_id.expect("no currency_id passed into transaction::db::add") as i32),
				&(transaction.recipient_id as i32),
				&(transaction.status as i32),
				&transaction.timestamp,
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
					&[&transaction_id, &(tag_id as i32)]
				).await?;
		}
	}

	if transaction.asset.is_some() && transaction.asset.clone().unwrap().id.is_some() {
		pool.get()
			.await?
			.query(
				"INSERT INTO public.asset_transactions (transaction_id, asset_id) VALUES ($1, $2);", 
			&[&(transaction_id as i32), &(transaction.asset.clone().unwrap().id.unwrap() as i32)]
		).await?;
	}

	for position in transaction.positions.iter() {
		pool.get()
			.await?
			.query(
				"INSERT INTO public.transaction_positions (id, transaction_id, amount, comment, tag_id) VALUES (DEFAULT, $1, $2, $3, $4);", 
				&[&transaction_id, &position.amount, &position.comment, &position.tag_id.map(|x| x as i32)]
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

pub async fn get_all_deep(pool: &Pool) -> Result<Vec<DeepTransaction>, Box<dyn Error>> {
	return Ok(
		pool.get()
			.await?
			.query("SELECT * FROM deep_transactions", &[])
			.await?
			.iter()
			.map(|x| turn_row_into_deep_transaction(x))
			.collect()
	);
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

pub async fn get_by_asset_id(pool: &Pool, asset_id: u32) -> Result<Vec<Transaction>, Box<dyn Error>> {
	return Ok( 
		pool.get()
			.await?
			.query(
				"SELECT * FROM public.transaction_data WHERE asset_id=$1;", 
				&[&(asset_id as i32)]
			)
			.await?
			.iter()
			.map(|x| turn_row_into_transaction(x))
			.collect()
		);
}

pub async fn update(pool: &Pool, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
	if transaction.id.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("transaction") }));
	}

	get_by_id(&pool, transaction.id.unwrap()).await?;

	let client = pool.get().await?;

	client.query(
		"UPDATE public.transactions SET account_id=$1, currency_id=$2, recipient_id=$3, status=$4, timestamp=$5, comment=$6 WHERE id=$7;", 
		&[&(transaction.account_id as i32),
			&(transaction.currency_id.expect("no currency_id passed into transaction::db::update") as i32),
			&(transaction.recipient_id as i32),
			&(transaction.status as i32),
			&transaction.timestamp,
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
		"DELETE FROM public.asset_transactions WHERE transaction_id=$1;",
		&[&(transaction.id.unwrap() as i32)]
	).await?;

	if transaction.asset.is_some() && transaction.asset.clone().unwrap().id.is_some() {
		client.query(
				"INSERT INTO public.asset_transactions (transaction_id, asset_id) VALUES ($1, $2);", 
			&[&(transaction.id.unwrap() as i32), &(transaction.asset.clone().unwrap().id.unwrap() as i32)]
		).await?;
	}

	client.query(
		"DELETE FROM public.transaction_positions WHERE transaction_id=$1;", 
		&[&(transaction.id.unwrap() as i32)]
	).await?;

	for position in transaction.positions.iter() {
		client.query(
				"INSERT INTO public.transaction_positions (id, transaction_id, amount, comment, tag_id) VALUES (DEFAULT, $1, $2, $3, $4);", 
				&[&(transaction.id.unwrap() as i32), &position.amount, &position.comment, &position.tag_id.map(|x| x as i32)]
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

		pool.get()
			.await?
			.query(
			"DELETE FROM public.transaction_positions WHERE transaction_id=$1;", 
			&[&(transaction_id as i32)]
		).await?;
	
	return Ok(());
}

fn turn_row_into_transaction(row: &tokio_postgres::Row) -> Transaction {
	let id: i32 = row.get(0);
	let account_id: i32 = row.get(1);
	let currency_id: i32 = row.get(2);
	let recipient_id: i32 = row.get(3);
	let status: i32 = row.get(4);
	let user_id: i32 = row.get(5);
	let timestamp: chrono::DateTime<chrono::Utc> = row.get(6);
	let comment: Option<String> = row.get(7);
	let tag_ids: Vec<u32> = row.try_get(8)
		.unwrap_or(Vec::new())
		.into_iter()
		.map(|x: i32| x as u32)
		.collect();
	let asset_id: Option<i32> = row.get(9);
	let asset_name: Option<String> = row.get(10);
	let asset_description: Option<String> = row.get(11);
	let transaction_position_ids: Vec<Option<i32>> = row.get(12);
	let transaction_position_amounts: Vec<Option<i32>> = row.get(13);
	let transaction_position_comments: Vec<Option<String>> = row.get(14);
	let transaction_position_tag_ids: Vec<Option<i32>> = row.get(15);

	let mut asset: Option<Asset> = None;
	if asset_id.is_some() {
		asset = Some(Asset {
			id: Some(asset_id.unwrap() as u32),
			name: asset_name.unwrap(),
			description: asset_description,
			user_id: user_id as u32,
			currency_id: currency_id as u32,
			value_per_unit: None,
			amount: None,
			tag_ids: None,
			total_cost_of_ownership: None,
		});
	}

	let positions: Vec<Position> = transaction_position_ids
		.into_iter()
		.filter(|x| x.is_some())
		.enumerate()
		.map(|(i, transaction_position_id)| {
			Position {
				id: Some(transaction_position_id.unwrap() as u32),
				amount: transaction_position_amounts[i].unwrap(),
				comment: transaction_position_comments[i].clone(),
				tag_id: transaction_position_tag_ids[i].map(|x| x as u32),
			}
		}).collect();

	let mut total_amount: i32 = 0;
	transaction_position_amounts
		.into_iter()
		.filter(|x| x.is_some())
		.for_each(|x| total_amount = total_amount + x.unwrap());

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
		timestamp,
		total_amount: Some(total_amount),
		comment,
		tag_ids: Some(tag_ids),
		asset,
		positions,
	};
}

fn turn_row_into_deep_transaction(row: &tokio_postgres::Row) -> DeepTransaction {
	let id: i32 = row.get(0);
	let status: i32 = row.get(1);
	let timestamp: chrono::DateTime<chrono::Utc> = row.get(2);
	let comment: Option<String> = row.get(3);
	let currency_id: i32 = row.get(4);
	let currency_minor_in_mayor: i32 = row.get(5);
	let currency_name: String = row.get(6);
	let currency_symbol: String = row.get(7);
	let user_id: i32 = row.get(8);
	let user_name: String = row.get(9);
	let user_superuser: bool = row.get(10);
	let account_id: i32 = row.get(11);
	let account_name: String = row.get(12);
	let account_default_currency_id: i32 = row.get(13);
	let account_default_currency_name: String = row.get(14);
	let account_default_currency_minor_in_mayor: i32 = row.get(15);
	let account_default_currency_symbol: String = row.get(16);
	let account_user_id: Option<i32> = row.get(17);
	let account_user_name: Option<String> = row.get(18);
	let account_user_superuser: Option<bool> = row.get(19);
	let account_tag_ids: Vec<Option<i32>> = row.get(20);
	let account_tag_names: Vec<Option<String>> = row.get(21);
	let account_tag_parent_ids: Vec<Option<i32>> = row.get(22);
	let account_tag_parent_names: Vec<Option<String>> = row.get(23);
	let account_tag_parent_parent_ids: Vec<Option<i32>> = row.get(24);
	let account_tag_parent_user_ids: Vec<Option<i32>> = row.get(25);
	let account_tag_user_ids: Vec<Option<i32>> = row.get(26);
	let account_tag_user_names: Vec<Option<String>> = row.get(27);
	let account_tag_user_superusers: Vec<Option<bool>> = row.get(28);
	let recipient_id: i32 = row.get(29);
	let recipient_name: String = row.get(30);
	let recipient_user_id: Option<i32> = row.get(31);
	let recipient_user_name: Option<String> = row.get(32);
	let recipient_user_superuser: Option<bool> = row.get(33);
	let recipient_tag_ids: Vec<Option<i32>> = row.get(34);
	let recipient_tag_names: Vec<Option<String>> = row.get(35);
	let recipient_tag_parent_ids: Vec<Option<i32>> = row.get(36);
	let recipient_tag_parent_names: Vec<Option<String>> = row.get(37);
	let recipient_tag_parent_parent_ids: Vec<Option<i32>> = row.get(38);
	let recipient_tag_parent_user_ids: Vec<Option<i32>> = row.get(39);
	let recipient_tag_user_ids: Vec<Option<i32>> = row.get(40);
	let recipient_tag_user_names: Vec<Option<String>> = row.get(41);
	let recipient_tag_user_superusers: Vec<Option<bool>> = row.get(42);
	let tag_ids: Vec<Option<i32>> = row.get(43);
	let tag_names: Vec<Option<String>> = row.get(44);
	let tag_parent_ids: Vec<Option<i32>> = row.get(45);
	let tag_parent_names: Vec<Option<String>> = row.get(46);
	let tag_parent_parent_ids: Vec<Option<i32>> = row.get(47);
	let tag_parent_user_ids: Vec<Option<i32>> = row.get(48);
	let tag_user_ids: Vec<Option<i32>> = row.get(49);
	let tag_user_names: Vec<Option<String>> = row.get(50);
	let tag_user_superusers: Vec<Option<bool>> = row.get(51);
	let asset_id: Option<i32> = row.get(52);
	let asset_name: Option<String> = row.get(53);
	let asset_description: Option<String> = row.get(54);
	let asset_value_per_unit: i32 = row.try_get(55).unwrap_or(0);
	let asset_amount: f64 = row.try_get(56).unwrap_or(0.0);
	let asset_currency_id: Option<i32> = row.get(57);
	let asset_currency_minor_in_mayor: Option<i32> = row.get(58);
	let asset_currency_name: Option<String> = row.get(59);
	let asset_currency_symbol: Option<String> = row.get(60);
	let asset_user_id: Option<i32> = row.get(61);
	let asset_user_name: Option<String> = row.get(62);
	let asset_user_superuser: Option<bool> = row.get(63);
	let asset_tag_ids: Option<Vec<Option<i32>>> = row.get(64);
	let asset_tag_names: Option<Vec<Option<String>>> = row.get(65);
	let asset_tag_parent_ids: Option<Vec<Option<i32>>> = row.get(66);
	let asset_tag_parent_names: Option<Vec<Option<String>>> = row.get(67);
	let asset_tag_parent_parent_ids: Option<Vec<Option<i32>>> = row.get(68);
	let asset_tag_parent_user_ids: Option<Vec<Option<i32>>> = row.get(69);
	let asset_tag_user_ids: Option<Vec<Option<i32>>> = row.get(70);
	let asset_tag_user_names: Option<Vec<Option<String>>> = row.get(71);
	let asset_tag_user_superusers: Option<Vec<Option<bool>>> = row.get(72);
	let transaction_position_ids: Vec<Option<i32>> = row.get(73);
	let transaction_position_amounts: Vec<Option<i32>> = row.get(74);
	let transaction_position_comments: Vec<Option<String>> = row.get(75);
	let transaction_position_tag_ids: Vec<Option<i32>> = row.get(76);

	let currency = crate::currency::Currency {
		id: Some(currency_id as u32),
		name: currency_name,
		minor_in_mayor: currency_minor_in_mayor as u32,
		symbol: currency_symbol
	};

	let user = crate::user::User {
		id: Some(user_id as u32),
		name: user_name,
		secret: None,
		superuser: user_superuser,
	};

	let tags: Vec<crate::tag::DeepTag> = tag_ids
	.into_iter()
	.filter(|x| x.is_some())
	.enumerate()
	.map(|(i, tag_id)| {
		let parent: Option<crate::tag::Tag> = match tag_parent_ids.get(i) {
			Some(x) => {
				match x {
					Some(_) => {
						Some(crate::tag::Tag {
							id: tag_parent_ids[i].map(|x| x as u32),
							name: tag_parent_names[i].clone().unwrap(),
							user_id: tag_parent_user_ids[i].unwrap() as u32,
							parent_id: tag_parent_parent_ids[i].map(|x| x as u32),
						})
					},
					None => None,
				}
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

	let account_default_currency = crate::currency::Currency {
		id: Some(account_default_currency_id as u32),
		name: account_default_currency_name,
		minor_in_mayor: account_default_currency_minor_in_mayor as u32,
		symbol: account_default_currency_symbol
	};

	let account_user = match account_user_id {
		Some(_) => {
			Some( crate::user::User {
				id: Some(account_user_id.unwrap() as u32),
				name: account_user_name.unwrap(),
				secret: None,
				superuser: account_user_superuser.unwrap(),
			} )
		},
		None => None,
	};

	let account_tags: Vec<crate::tag::DeepTag> = account_tag_ids
		.into_iter()
		.filter(|x| x.is_some())
		.enumerate()
		.map(|(i, tag_id)| {
			let parent: Option<crate::tag::Tag> = match account_tag_parent_ids.get(i) {
				Some(x) => {
					match x {
						Some(_) => {
							Some(crate::tag::Tag {
								id: account_tag_parent_ids[i].map(|x| x as u32),
								name: account_tag_parent_names[i].clone().unwrap(),
								user_id: account_tag_parent_user_ids[i].unwrap() as u32,
								parent_id: account_tag_parent_parent_ids[i].map(|x| x as u32),
							})
						},
						None => None,
					}
				},
				None => None,
			};
			
			crate::tag::DeepTag {
				id: tag_id.unwrap() as u32,
				name: account_tag_names[i].clone().unwrap(),
				user: crate::user::User {
					id: account_tag_user_ids[i].map(|x| x as u32),
					name: account_tag_user_names[i].clone().unwrap(),
					secret: None,
					superuser: account_tag_user_superusers[i].unwrap(),
				},
				parent,
			}
		}).collect();

	let account = crate::account::DeepAccount {
		id: account_id as u32,
		name: account_name,
		default_currency: account_default_currency,
		user: account_user,
		tags: account_tags,
	};

	let recipient_user = match recipient_user_id {
		Some(_) => {
			Some( crate::user::User {
				id: Some(recipient_user_id.unwrap() as u32),
				name: recipient_user_name.unwrap(),
				secret: None,
				superuser: recipient_user_superuser.unwrap()
			} )
		},
		None => None,
	};

	let recipient_tags: Vec<crate::tag::DeepTag> = recipient_tag_ids
		.into_iter()
		.filter(|x| x.is_some())
		.enumerate()
		.map(|(i, tag_id)| {
			let parent: Option<crate::tag::Tag> = match recipient_tag_parent_ids.get(i) {
				Some(x) => {
					match x {
						Some(_) => {
							Some(crate::tag::Tag {
								id: recipient_tag_parent_ids[i].map(|x| x as u32),
								name: recipient_tag_parent_names[i].clone().unwrap(),
								user_id: recipient_tag_parent_user_ids[i].unwrap() as u32,
								parent_id: recipient_tag_parent_parent_ids[i].map(|x| x as u32),
							})
						},
						None => None,
					}
				},
				None => None,
			};
			
			crate::tag::DeepTag {
				id: tag_id.unwrap() as u32,
				name: recipient_tag_names[i].clone().unwrap(),
				user: crate::user::User {
					id: recipient_tag_user_ids[i].map(|x| x as u32),
					name: recipient_tag_user_names[i].clone().unwrap(),
					secret: None,
					superuser: recipient_tag_user_superusers[i].unwrap(),
				},
				parent,
			}
		}).collect();

	let recipient = crate::recipient::DeepRecipient {
		id: recipient_id as u32,
		name: recipient_name,
		user: recipient_user,
		tags: recipient_tags,
	};

	let asset_currency = match asset_id {
		Some(_) => {
			Some( 
				crate::currency::Currency {
					id: Some(asset_currency_id.unwrap() as u32),
					name: asset_currency_name.unwrap(),
					minor_in_mayor: asset_currency_minor_in_mayor.unwrap() as u32,
					symbol: asset_currency_symbol.unwrap()
				}
			)
		},
		None => None,
	};

	let asset_user = match asset_id {
		Some(_) => {
			Some(
				crate::user::User {
					id: Some(asset_user_id.unwrap() as u32),
					name: asset_user_name.unwrap(),
					secret: None,
					superuser: asset_user_superuser.unwrap()
				}
			)
		},
		None => None,
	};

	let asset_tags: Option<Vec<crate::tag::DeepTag>> = match asset_id {
		Some(_) => {
			Some(
				asset_tag_ids
					.unwrap()
					.into_iter()
					.filter(|x| x.is_some())
					.enumerate()
					.map(|(i, tag_id)| {
						let parent: Option<crate::tag::Tag> = match asset_tag_parent_ids.clone().unwrap().get(i) {
							Some(x) => {
								match x {
									Some(_) => {
										Some(crate::tag::Tag {
											id: asset_tag_parent_ids.clone().unwrap()[i].map(|x| x as u32),
											name: asset_tag_parent_names.clone().unwrap()[i].clone().unwrap(),
											user_id: asset_tag_parent_user_ids.clone().unwrap()[i].unwrap() as u32,
											parent_id: asset_tag_parent_parent_ids.clone().unwrap()[i].map(|x| x as u32),
										})
									},
									None => None,
								}
							},
							None => None,
						};
						
						crate::tag::DeepTag {
							id: tag_id.unwrap() as u32,
							name: asset_tag_names.clone().unwrap()[i].clone().unwrap(),
							user: crate::user::User {
								id: asset_tag_user_ids.clone().unwrap()[i].map(|x| x as u32),
								name: asset_tag_user_names.clone().unwrap()[i].clone().unwrap(),
								secret: None,
								superuser: asset_tag_user_superusers.clone().unwrap()[i].unwrap(),
							},
							parent,
						}
					}).collect()
			)
		},
		None => None,
	};

	let asset = match asset_id {
		Some(_) => {
			Some(
				crate::asset::DeepAsset {
					id: asset_id.unwrap() as u32,
					name: asset_name.unwrap(),
					description: asset_description,
					value_per_unit: asset_value_per_unit as u32,
					amount: asset_amount,
					user: asset_user.unwrap(),
					currency: asset_currency.unwrap(),
					tags: asset_tags.unwrap(),
					total_cost_of_ownership: None,
				}
			)
		},
		None => None
	};

	let positions: Vec<Position> = transaction_position_ids
		.into_iter()
		.filter(|x| x.is_some())
		.enumerate()
		.map(|(i, transaction_position_id)| {
			Position {
				id: Some(transaction_position_id.unwrap() as u32),
				amount: transaction_position_amounts[i].unwrap(),
				comment: transaction_position_comments[i].clone(),
				tag_id: transaction_position_tag_ids[i].map(|x| x as u32),
			}
		}).collect();

	let mut total_amount: i32 = 0;
	transaction_position_amounts
		.into_iter()
		.filter(|x| x.is_some())
		.for_each(|x| total_amount = total_amount + x.unwrap());

	return DeepTransaction {
		id: id as u32,
		status: match status {
			0 => TransactionStatus::Withheld,
			1 => TransactionStatus::Completed,
			_ => panic!("invalid transaction status found in row from database")
		},
		timestamp,
		total_amount: Some(total_amount),
		comment,
		currency,
		user,
		account,
		recipient,
		tags,
		asset,
		positions,
	}
}