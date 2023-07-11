use deadpool_postgres::Pool;
use std::error::Error;
use super::super::CustomError;
use super::{Transaction, TransactionStatus, Asset, DeepTransaction, Position};
use crate::traits::*;

#[derive(Debug)]
pub struct TransactionDbSelecter<'a> {
	query_parameters: super::QueryParameters,
	pool: &'a Pool,
}

impl<'a> DbReader<'a, Transaction> for TransactionDbSelecter<'a> {
	fn new(pool: &'a Pool) -> Self {
		return Self {
			query_parameters: super::QueryParameters::default(),
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

	async fn execute(self) -> Result<Vec<Transaction>, Box<dyn Error>> {
		let query = "SELECT * FROM public.transaction_data";
		return Ok(
			self.actually_execute(query)
			.await?
			.into_iter()
			.map(Into::into)
			.collect()
		);
	}
}

#[derive(Debug)]
pub struct DeepTransactionDbSelecter<'a> {
	query_parameters: super::QueryParameters,
	pool: &'a Pool,
}

impl<'a> DbReader<'a, DeepTransaction> for DeepTransactionDbSelecter<'a> {
	fn new(pool: &'a Pool) -> Self {
		return Self {
			query_parameters: super::QueryParameters::default(),
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

	async fn execute(self) -> Result<Vec<DeepTransaction>, Box<dyn Error>> {
		let query = "SELECT * FROM public.deep_transactions";
		return Ok(
			self.actually_execute(query)
			.await?
			.into_iter()
			.map(Into::into)
			.collect()
		);
	}
}

#[derive(Debug)]
pub struct TransactionDbWriter<'a> {
	pool: &'a Pool,
	transaction: Transaction,
}

impl<'a> DbWriter<'a, Transaction> for TransactionDbWriter<'a> {
	fn new(pool: &'a Pool, item: Transaction) -> Self {
		return Self {
			pool,
			transaction: item,
		}
	}

	async fn insert(self) -> Result<(), Box<dyn Error>> {
		let transaction_id: i32 = self.pool.get()
			.await?
			.query(
				"INSERT INTO public.transactions (id, user_id, account_id, currency_id, recipient_id, status, timestamp, comment) VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7) RETURNING id;",
				&[
					&(self.transaction.user_id as i32),
					&(self.transaction.account_id as i32),
					&(self.transaction.currency_id.expect("no currency_id passed into transaction::db::add") as i32),
					&(self.transaction.recipient_id as i32),
					&(self.transaction.status as i32),
					&self.transaction.timestamp,
					&self.transaction.comment
				])
				.await?
				[0].get(0);
		
		if self.transaction.tag_ids.is_some() {
			for tag_id in self.transaction.tag_ids.clone().unwrap() {
				self.pool.get()
					.await?
					.query(
						"INSERT INTO public.transaction_tags (transaction_id, tag_id) VALUES ($1, $2);",
						&[&transaction_id, &(tag_id as i32)]
					).await?;
			}
		}

		if self.transaction.asset.is_some() && self.transaction.asset.clone().unwrap().id.is_some() {
			self.pool.get()
				.await?
				.query(
					"INSERT INTO public.asset_transactions (transaction_id, asset_id) VALUES ($1, $2);", 
				&[&transaction_id, &(self.transaction.asset.clone().unwrap().id.unwrap() as i32)]
			).await?;
		}

		for position in self.transaction.positions {
			self.pool.get()
				.await?
				.query(
					"INSERT INTO public.transaction_positions (id, transaction_id, amount, comment, tag_id) VALUES (DEFAULT, $1, $2, $3, $4);", 
					&[&transaction_id, &position.amount, &position.comment, &position.tag_id.map(|x| x as i32)]
				).await?;
		}

		return Ok(());
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		if self.transaction.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("transaction") }));
		}
	
		super::TransactionLoader::new(self.pool).set_filter_id(self.transaction.id.unwrap()).get_first().await?;
	
		let client = self.pool.get().await?;
	
		client.query(
			"UPDATE public.transactions SET account_id=$1, currency_id=$2, recipient_id=$3, status=$4, timestamp=$5, comment=$6 WHERE id=$7;", 
			&[&(self.transaction.account_id as i32),
				&(self.transaction.currency_id.expect("no currency_id passed into transaction::db::update") as i32),
				&(self.transaction.recipient_id as i32),
				&(self.transaction.status as i32),
				&self.transaction.timestamp,
				&self.transaction.comment,
				&(self.transaction.id.unwrap() as i32)
			]
		)
		.await?;
		
		client.query(
			"DELETE FROM public.transaction_tags WHERE transaction_id=$1;",
			&[&(self.transaction.id.unwrap() as i32)]
		)
		.await?;
	
		if self.transaction.tag_ids.is_some() {
			for tag_id in self.transaction.tag_ids.clone().unwrap() {
				client.query(
					"INSERT INTO public.transaction_tags (transaction_id, tag_id) VALUES ($1, $2);",
					&[&(self.transaction.id.unwrap() as i32), &(tag_id as i32)]
				)
				.await?;
			}
		}
	
		client.query(
			"DELETE FROM public.asset_transactions WHERE transaction_id=$1;",
			&[&(self.transaction.id.unwrap() as i32)]
		).await?;
	
		if self.transaction.asset.is_some() && self.transaction.asset.clone().unwrap().id.is_some() {
			client.query(
					"INSERT INTO public.asset_transactions (transaction_id, asset_id) VALUES ($1, $2);", 
				&[&(self.transaction.id.unwrap() as i32), &(self.transaction.asset.clone().unwrap().id.unwrap() as i32)]
			).await?;
		}
	
		client.query(
			"DELETE FROM public.transaction_positions WHERE transaction_id=$1;", 
			&[&(self.transaction.id.unwrap() as i32)]
		).await?;
	
		for position in self.transaction.positions {
			client.query(
					"INSERT INTO public.transaction_positions (id, transaction_id, amount, comment, tag_id) VALUES (DEFAULT, $1, $2, $3, $4);", 
					&[&(self.transaction.id.unwrap() as i32), &position.amount, &position.comment, &position.tag_id.map(|x| x as i32)]
				).await?;
		}
	
		return Ok(());
	}

	async fn delete(self) -> Result<(), Box<dyn Error>> {
		if self.transaction.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("transaction") }));
		}

		self.pool.get()
			.await?
			.query(
				"DELETE FROM public.transactions WHERE id=$1;", 
				&[&(self.transaction.id.unwrap() as i32)]
			)
			.await?;

			self.pool.get()
				.await?
				.query(
				"DELETE FROM public.transaction_positions WHERE transaction_id=$1;", 
				&[&(self.transaction.id.unwrap() as i32)]
			).await?;

		return Ok(());
	}
}

impl From<tokio_postgres::Row> for Transaction {
	fn from(value: tokio_postgres::Row) -> Transaction {
		let id: i32 = value.get(0);
		let account_id: i32 = value.get(1);
		let currency_id: i32 = value.get(2);
		let recipient_id: i32 = value.get(3);
		let status: i32 = value.get(4);
		let user_id: i32 = value.get(5);
		let timestamp: chrono::DateTime<chrono::Utc> = value.get(6);
		let comment: Option<String> = value.get(7);
		let tag_ids: Vec<u32> = value.try_get(8)
			.unwrap_or(Vec::new())
			.into_iter()
			.map(|x: i32| x as u32)
			.collect();
		let asset_id: Option<i32> = value.get(9);
		let asset_name: Option<String> = value.get(10);
		let asset_description: Option<String> = value.get(11);
		let transaction_position_ids: Vec<Option<i32>> = value.get(12);
		let transaction_position_amounts: Vec<Option<i32>> = value.get(13);
		let transaction_position_comments: Vec<Option<String>> = value.get(14);
		let transaction_position_tag_ids: Vec<Option<i32>> = value.get(15);

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
			.filter(Option::is_some)
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
			.filter(Option::is_some)
			.for_each(|x| total_amount += x.unwrap());

		return Transaction::default()
			.set_id(id as u32)
			.set_user_id(user_id as u32)
			.set_account_id(account_id as u32)
			.set_currency_id(currency_id as u32)
			.set_recipient_id(recipient_id as u32)
			.set_status(match status {
				0 => TransactionStatus::Withheld,
				1 => TransactionStatus::Completed,
				_ => panic!("invalid transaction status found in row from database")
			})
			.set_timestamp(timestamp)
			.set_total_amount(total_amount)
			.set_comment_opt(comment)
			.set_tag_ids(tag_ids)
			.set_asset_opt(asset)
			.set_positions(positions);
	}
}

impl From<tokio_postgres::Row> for DeepTransaction {
	#[allow(clippy::too_many_lines)]
	fn from(value: tokio_postgres::Row) -> DeepTransaction {
		let id: i32 = value.get(0);
		let status: i32 = value.get(1);
		let timestamp: chrono::DateTime<chrono::Utc> = value.get(2);
		let comment: Option<String> = value.get(3);
		let currency_id: i32 = value.get(4);
		let currency_minor_in_mayor: i32 = value.get(5);
		let currency_name: String = value.get(6);
		let currency_symbol: String = value.get(7);
		let user_id: i32 = value.get(8);
		let user_name: String = value.get(9);
		let user_superuser: bool = value.get(10);
		let account_id: i32 = value.get(11);
		let account_name: String = value.get(12);
		let account_default_currency_id: i32 = value.get(13);
		let account_default_currency_name: String = value.get(14);
		let account_default_currency_minor_in_mayor: i32 = value.get(15);
		let account_default_currency_symbol: String = value.get(16);
		let account_user_id: Option<i32> = value.get(17);
		let account_user_name: Option<String> = value.get(18);
		let account_user_superuser: Option<bool> = value.get(19);
		let account_tag_ids: Vec<Option<i32>> = value.get(20);
		let account_tag_names: Vec<Option<String>> = value.get(21);
		let account_tag_parent_ids: Vec<Option<i32>> = value.get(22);
		let account_tag_parent_names: Vec<Option<String>> = value.get(23);
		let account_tag_parent_parent_ids: Vec<Option<i32>> = value.get(24);
		let account_tag_parent_user_ids: Vec<Option<i32>> = value.get(25);
		let account_tag_user_ids: Vec<Option<i32>> = value.get(26);
		let account_tag_user_names: Vec<Option<String>> = value.get(27);
		let account_tag_user_superusers: Vec<Option<bool>> = value.get(28);
		let recipient_id: i32 = value.get(29);
		let recipient_name: String = value.get(30);
		let recipient_user_id: Option<i32> = value.get(31);
		let recipient_user_name: Option<String> = value.get(32);
		let recipient_user_superuser: Option<bool> = value.get(33);
		let recipient_tag_ids: Vec<Option<i32>> = value.get(34);
		let recipient_tag_names: Vec<Option<String>> = value.get(35);
		let recipient_tag_parent_ids: Vec<Option<i32>> = value.get(36);
		let recipient_tag_parent_names: Vec<Option<String>> = value.get(37);
		let recipient_tag_parent_parent_ids: Vec<Option<i32>> = value.get(38);
		let recipient_tag_parent_user_ids: Vec<Option<i32>> = value.get(39);
		let recipient_tag_user_ids: Vec<Option<i32>> = value.get(40);
		let recipient_tag_user_names: Vec<Option<String>> = value.get(41);
		let recipient_tag_user_superusers: Vec<Option<bool>> = value.get(42);
		let tag_ids: Vec<Option<i32>> = value.get(43);
		let tag_names: Vec<Option<String>> = value.get(44);
		let tag_parent_ids: Vec<Option<i32>> = value.get(45);
		let tag_parent_names: Vec<Option<String>> = value.get(46);
		let tag_parent_parent_ids: Vec<Option<i32>> = value.get(47);
		let tag_parent_user_ids: Vec<Option<i32>> = value.get(48);
		let tag_user_ids: Vec<Option<i32>> = value.get(49);
		let tag_user_names: Vec<Option<String>> = value.get(50);
		let tag_user_superusers: Vec<Option<bool>> = value.get(51);
		let asset_id: Option<i32> = value.get(52);
		let asset_name: Option<String> = value.get(53);
		let asset_description: Option<String> = value.get(54);
		let asset_value_per_unit: i32 = value.try_get(55).unwrap_or(0);
		let asset_amount: f64 = value.try_get(56).unwrap_or(0.0);
		let asset_currency_id: Option<i32> = value.get(57);
		let asset_currency_minor_in_mayor: Option<i32> = value.get(58);
		let asset_currency_name: Option<String> = value.get(59);
		let asset_currency_symbol: Option<String> = value.get(60);
		let asset_user_id: Option<i32> = value.get(61);
		let asset_user_name: Option<String> = value.get(62);
		let asset_user_superuser: Option<bool> = value.get(63);
		let asset_tag_ids: Option<Vec<Option<i32>>> = value.get(64);
		let asset_tag_names: Option<Vec<Option<String>>> = value.get(65);
		let asset_tag_parent_ids: Option<Vec<Option<i32>>> = value.get(66);
		let asset_tag_parent_names: Option<Vec<Option<String>>> = value.get(67);
		let asset_tag_parent_parent_ids: Option<Vec<Option<i32>>> = value.get(68);
		let asset_tag_parent_user_ids: Option<Vec<Option<i32>>> = value.get(69);
		let asset_tag_user_ids: Option<Vec<Option<i32>>> = value.get(70);
		let asset_tag_user_names: Option<Vec<Option<String>>> = value.get(71);
		let asset_tag_user_superusers: Option<Vec<Option<bool>>> = value.get(72);
		let transaction_position_ids: Vec<Option<i32>> = value.get(73);
		let transaction_position_amounts: Vec<Option<i32>> = value.get(74);
		let transaction_position_comments: Vec<Option<String>> = value.get(75);
		let transaction_position_tag_ids: Vec<Option<i32>> = value.get(76);

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

		let account_default_currency = crate::currency::Currency {
			id: Some(account_default_currency_id as u32),
			name: account_default_currency_name,
			minor_in_mayor: account_default_currency_minor_in_mayor as u32,
			symbol: account_default_currency_symbol
		};

		let account_user = account_user_id.map(|_| crate::user::User {
			id: Some(account_user_id.unwrap() as u32),
			name: account_user_name.unwrap(),
			secret: None,
			superuser: account_user_superuser.unwrap(),
		});

		let account_tags: Vec<crate::tag::DeepTag> = account_tag_ids
			.into_iter()
			.filter(Option::is_some)
			.enumerate()
			.map(|(i, tag_id)| {
				let parent: Option<crate::tag::Tag> = match account_tag_parent_ids.get(i) {
					Some(x) => {
						x.as_ref().map(|_| crate::tag::Tag {
							id: account_tag_parent_ids[i].map(|x| x as u32),
							name: account_tag_parent_names[i].clone().unwrap(),
							user_id: account_tag_parent_user_ids[i].unwrap() as u32,
							parent_id: account_tag_parent_parent_ids[i].map(|x| x as u32),
						})
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

		let recipient_user = recipient_user_id.map(|_| crate::user::User {
			id: Some(recipient_user_id.unwrap() as u32),
			name: recipient_user_name.unwrap(),
			secret: None,
			superuser: recipient_user_superuser.unwrap()
		});

		let recipient_tags: Vec<crate::tag::DeepTag> = recipient_tag_ids
			.into_iter()
			.filter(Option::is_some)
			.enumerate()
			.map(|(i, tag_id)| {
				let parent: Option<crate::tag::Tag> = match recipient_tag_parent_ids.get(i) {
					Some(x) => {
						x.as_ref().map(|_| crate::tag::Tag {
							id: recipient_tag_parent_ids[i].map(|x| x as u32),
							name: recipient_tag_parent_names[i].clone().unwrap(),
							user_id: recipient_tag_parent_user_ids[i].unwrap() as u32,
							parent_id: recipient_tag_parent_parent_ids[i].map(|x| x as u32),
						})
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

		let asset_currency = asset_id.map(|_| crate::currency::Currency {
			id: Some(asset_currency_id.unwrap() as u32),
			name: asset_currency_name.unwrap(),
			minor_in_mayor: asset_currency_minor_in_mayor.unwrap() as u32,
			symbol: asset_currency_symbol.unwrap()
		});

		let asset_user = asset_id.map(|_| crate::user::User {
			id: Some(asset_user_id.unwrap() as u32),
			name: asset_user_name.unwrap(),
			secret: None,
			superuser: asset_user_superuser.unwrap()
		});

		let asset_tags: Option<Vec<crate::tag::DeepTag>> = asset_id.map(|_| asset_tag_ids
			.unwrap()
			.into_iter()
			.filter(Option::is_some)
			.enumerate()
			.map(|(i, tag_id)| {
				let parent: Option<crate::tag::Tag> = match asset_tag_parent_ids.clone().unwrap().get(i) {
					Some(x) => {
						x.as_ref().map(|_| crate::tag::Tag {
							id: asset_tag_parent_ids.clone().unwrap()[i].map(|x| x as u32),
							name: asset_tag_parent_names.clone().unwrap()[i].clone().unwrap(),
							user_id: asset_tag_parent_user_ids.clone().unwrap()[i].unwrap() as u32,
							parent_id: asset_tag_parent_parent_ids.clone().unwrap()[i].map(|x| x as u32),
						})
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
			}).collect());

		let asset = asset_id.map(|_| crate::asset::DeepAsset {
			id: asset_id.unwrap() as u32,
			name: asset_name.unwrap(),
			description: asset_description,
			value_per_unit: asset_value_per_unit as u32,
			amount: asset_amount,
			user: asset_user.unwrap(),
			currency: asset_currency.unwrap(),
			tags: asset_tags.unwrap(),
			total_cost_of_ownership: None,
		});

		let positions: Vec<Position> = transaction_position_ids
			.into_iter()
			.filter(Option::is_some)
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
			.filter(Option::is_some)
			.for_each(|x| total_amount += x.unwrap());

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
}