use deadpool_postgres::Pool;
use std::error::Error;
use uuid::Uuid;
use super::super::CustomError;
use super::{Transaction, TransactionStatus, Asset, Position, TransactionSummary};
use crate::money::Money;
use crate::traits::*;

#[derive(Debug, Clone)]
pub struct TransactionDbReader<'a> {
	query_parameters: QueryParameters,
	pool: &'a Pool,
}

impl<'a> DbReader<'a, Transaction> for TransactionDbReader<'a> {
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


impl<'a> TransactionDbReader<'a> {
	pub async fn summarize(self) -> Result<TransactionSummary, Box<dyn Error>> {
		let count_query = "SELECT COUNT(*) FROM public.transaction_data";
		let temp = self.clone().actually_execute(count_query).await?;
		let count_result = temp.first().unwrap();
		let count: i64 = count_result.get(0);


		let parameters = self.get_formatted_query_parameters(Some("tr".to_string()));
		let parameter_values: Vec<_> = parameters.1.iter()
			.map(|x| &**x as &(dyn postgres_types::ToSql + Sync))
			.collect();

		let total_amount = self.get_pool()
			.get()
			.await?
			.query(
				format!("
					SELECT 
						tr.currency_id,
						concat(trunc(sum(tr.total_amount::numeric) / c.minor_in_major::numeric, 2)::text, c.symbol) AS total_amount
					FROM transaction_data tr
						LEFT JOIN currencies c ON tr.currency_id = c.id
					{}
					GROUP BY tr.currency_id, c.symbol, c.minor_in_major;"
				, parameters.0).as_str(), parameter_values.as_slice()
			).await?
		  .into_iter()
			.fold(String::new(), |a, b| a + " " + b.get(1))
			.trim()
			.to_string();

		return Ok(TransactionSummary { count: count as u32, total_amount });
	}
}

#[derive(Debug)]
pub struct TransactionDbWriter<'a> {
	pool: &'a Pool,
	transaction: Transaction,
}

impl<'a> OldDbWriter<'a, Transaction> for TransactionDbWriter<'a> {
	fn new(pool: &'a Pool, item: Transaction) -> Self {
		Self {
			pool,
			transaction: item,
		}
	}

	async fn insert(self) -> Result<u32, Box<dyn Error>> {
		let transaction_id: i32 = self.pool.get()
			.await?
			.query(
				"INSERT INTO public.transactions (id, user_id, account_id, currency_id, recipient_id, status, timestamp, comment) VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7) RETURNING id;",
				&[
					&(self.transaction.user_id as i32),
					&(self.transaction.account_id as i32),
					&(self.transaction.currency_id.expect("no currency_id passed into transaction::db::add") as i32),
					&self.transaction.recipient_id,
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

		if self.transaction.asset.is_some() {
			self.pool.get()
				.await?
				.query(
					"INSERT INTO public.asset_transactions (transaction_id, asset_id) VALUES ($1, $2);", 
				&[&transaction_id, &self.transaction.asset.clone().unwrap().id]
			).await?;
		}

		for position in self.transaction.positions {
			self.pool.get()
				.await?
				.query(
					"INSERT INTO public.transaction_positions (id, transaction_id, amount, comment, tag_id) VALUES (DEFAULT, $1, $2, $3, $4);", 
					&[&transaction_id, &position.amount.to_amount(), &position.comment, &position.tag_id.map(|x| x as i32)]
				).await?;
		}

		return Ok(transaction_id as u32);
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		if self.transaction.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("transaction") }));
		}
	
		let old = super::TransactionLoader::new(self.pool)
			.set_filter_id(self.transaction.id.unwrap(), NumberFilterModes::Exact)
			.get_first().await?;

		if old.user_id != self.transaction.user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
		}
	
		let client = self.pool.get().await?;
	
		client.query(
			"UPDATE public.transactions SET account_id=$1, currency_id=$2, recipient_id=$3, status=$4, timestamp=$5, comment=$6 WHERE id=$7;", 
			&[&(self.transaction.account_id as i32),
				&(self.transaction.currency_id.expect("no currency_id passed into transaction::db::update") as i32),
				&self.transaction.recipient_id,
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
	
		if self.transaction.asset.is_some() {
			client.query(
					"INSERT INTO public.asset_transactions (transaction_id, asset_id) VALUES ($1, $2);", 
				&[&(self.transaction.id.unwrap() as i32), &self.transaction.asset.clone().unwrap().id]
			).await?;
		}
	
		client.query(
			"DELETE FROM public.transaction_positions WHERE transaction_id=$1;", 
			&[&(self.transaction.id.unwrap() as i32)]
		).await?;
	
		for position in self.transaction.positions {
			client.query(
					"INSERT INTO public.transaction_positions (id, transaction_id, amount, comment, tag_id) VALUES (DEFAULT, $1, $2, $3, $4);", 
					&[&(self.transaction.id.unwrap() as i32), &position.amount.to_amount(), &position.comment, &position.tag_id.map(|x| x as i32)]
				).await?;
		}
	
		return Ok(());
	}
}

impl<'a> OldDbDeleter<'a, Transaction> for TransactionDbWriter<'a> {
	async fn delete(self) -> Result<(), Box<dyn Error>> {
		if self.transaction.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty { property: String::from("id"), item_type: String::from("transaction") }));
		}

		let old = super::TransactionLoader::new(self.pool)
			.set_filter_id(self.transaction.id.unwrap(), NumberFilterModes::Exact)
			.get_first().await?;

		if old.user_id != self.transaction.user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
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

#[allow(clippy::unwrap_or_default)]
impl From<tokio_postgres::Row> for Transaction {
	fn from(value: tokio_postgres::Row) -> Transaction {
		let id: i32 = value.get(0);
		let account_id: i32 = value.get(1);
		let currency_id: i32 = value.get(2);
		let recipient_id: Uuid = value.get(3);
		let status: i32 = value.get(4);
		let user_id: i32 = value.get(5);
		let timestamp: chrono::DateTime<chrono::Utc> = value.get(6);
		let comment: Option<String> = value.get(7);
		let tag_ids: Vec<u32> = value.try_get(8)
			.unwrap_or(Vec::new())
			.into_iter()
			.map(|x: i32| x as u32)
			.collect();
		let asset_id: Option<Uuid> = value.get(9);
		let asset_name: Option<String> = value.get(10);
		let asset_description: Option<String> = value.get(11);
		let transaction_position_ids: Vec<Option<i32>> = value.get(12);
		let transaction_position_amounts: Vec<Option<i32>> = value.get(13);
		let transaction_position_comments: Vec<Option<String>> = value.get(14);
		let transaction_position_tag_ids: Vec<Option<i32>> = value.get(15);
		let total_amount: i64 = value.get(16);
		let minor_in_major: i32 = value.get(17);
		let symbol: String = value.get(18);

		let mut asset: Option<Asset> = None;
		if asset_id.is_some() {
			asset = Some(Asset {
				id: asset_id.unwrap(),
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
			.flatten()
			.enumerate()
			.map(|(i, transaction_position_id)| {
				Position {
					id: Some(transaction_position_id as u32),
					amount: Money::from_amount(transaction_position_amounts[i].unwrap(), minor_in_major as u32, symbol.clone()),
					comment: transaction_position_comments[i].clone(),
					tag_id: transaction_position_tag_ids[i].map(|x| x as u32),
				}
			}).collect();

		return Transaction::default()
			.set_id(id as u32)
			.set_user_id(user_id as u32)
			.set_account_id(account_id as u32)
			.set_currency_id(currency_id as u32)
			.set_recipient_id(recipient_id)
			.set_status(match status {
				0 => TransactionStatus::Withheld,
				1 => TransactionStatus::Completed,
				_ => panic!("invalid transaction status found in row from database")
			})
			.set_timestamp(timestamp)
			.set_total_amount(Money::from_amount(total_amount.try_into().unwrap(), minor_in_major as u32, symbol))
			.set_comment_opt(comment)
			.set_tag_ids(tag_ids)
			.set_asset_opt(asset)
			.set_positions(positions);
	}
}