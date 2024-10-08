use deadpool_postgres::Pool;
use postgres_types::ToSql;
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
					GROUP BY tr.currency_id, c.symbol, c.minor_in_major, c.name
					ORDER BY c.name ASC;"
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

impl<'a> DbWriter<'a, Transaction> for TransactionDbWriter<'a> {
	fn new(pool: &'a Pool, item: Transaction) -> Self {
		Self {
			pool,
			transaction: item,
		}
	}

	async fn insert(self) -> Result<Uuid, Box<dyn Error>> {
		self.pool.get()
			.await?
			.query(
				"INSERT INTO public.transactions (id, user_id, account_id, currency_id, recipient_id, status, timestamp, comment) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);",
				&[
					&self.transaction.id,
					&self.transaction.user_id,
					&self.transaction.account_id,
					&self.transaction.currency_id.expect("no currency_id passed into transaction::db::add"),
					&self.transaction.recipient_id,
					&(self.transaction.status as i32),
					&self.transaction.timestamp,
					&self.transaction.comment
				])
				.await?;
		
		for tag_id in self.transaction.tag_ids.clone() {
			self.pool.get()
				.await?
				.query(
					"INSERT INTO public.transaction_tags (transaction_id, tag_id) VALUES ($1, $2);",
					&[&self.transaction.id, &tag_id]
				).await?;
		}

		if self.transaction.asset.is_some() {
			self.pool.get()
				.await?
				.query(
					"INSERT INTO public.asset_transactions (transaction_id, asset_id) VALUES ($1, $2);", 
				&[&self.transaction.id, &self.transaction.asset.clone().unwrap().id]
			).await?;
		}

		for position in self.transaction.positions {
			self.pool.get()
				.await?
				.query(
					"INSERT INTO public.transaction_positions (id, transaction_id, amount, comment, tag_id) VALUES ($1, $2, $3, $4, $5);", 
					&[&position.id, &self.transaction.id, &position.amount.to_amount(), &position.comment, &position.tag_id]
				).await?;
		}

		return Ok(self.transaction.id);
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		let old = super::TransactionLoader::new(self.pool)
			.set_filter_id(self.transaction.id, NumberFilterModes::Exact)
			.get_first().await?;

		if old.user_id != self.transaction.user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
		}
	
		let client = self.pool.get().await?;
	
		client.query(
			"UPDATE public.transactions SET account_id=$1, currency_id=$2, recipient_id=$3, status=$4, timestamp=$5, comment=$6 WHERE id=$7;", 
			&[&self.transaction.account_id,
				&self.transaction.currency_id.expect("no currency_id passed into transaction::db::update"),
				&self.transaction.recipient_id,
				&(self.transaction.status as i32),
				&self.transaction.timestamp,
				&self.transaction.comment,
				&self.transaction.id,
			]
		)
		.await?;
		
		client.query(
			"DELETE FROM public.transaction_tags WHERE transaction_id=$1;",
			&[&self.transaction.id]
		)
		.await?;
	
		for tag_id in self.transaction.tag_ids.clone() {
			client.query(
				"INSERT INTO public.transaction_tags (transaction_id, tag_id) VALUES ($1, $2);",
				&[&self.transaction.id, &tag_id]
			)
			.await?;
		}
	
		client.query(
			"DELETE FROM public.asset_transactions WHERE transaction_id=$1;",
			&[&self.transaction.id]
		).await?;
	
		if self.transaction.asset.is_some() {
			client.query(
					"INSERT INTO public.asset_transactions (transaction_id, asset_id) VALUES ($1, $2);", 
				&[&self.transaction.id, &self.transaction.asset.clone().unwrap().id]
			).await?;
		}
	
		client.query(
			"DELETE FROM public.transaction_positions WHERE transaction_id=$1;", 
			&[&self.transaction.id]
		).await?;
	
		for position in self.transaction.positions {
			client.query(
					"INSERT INTO public.transaction_positions (id, transaction_id, amount, comment, tag_id) VALUES ($1, $2, $3, $4, $5);", 
					&[&position.id, &self.transaction.id, &position.amount.to_amount(), &position.comment, &position.tag_id]
				).await?;
		}
	
		return Ok(());
	}
}

#[derive(Debug)]
pub struct TransactionVecDbWriter<'a> {
	pool: &'a Pool,
	transactions: Vec<Transaction>,
}

impl<'a> DbWriter<'a, Vec<Transaction>> for TransactionVecDbWriter<'a> {
	fn new(pool: &'a Pool, item: Vec<Transaction>) -> Self {
		Self {
			pool,
			transactions: item,
		}
	}

	async fn insert(self) -> Result<Uuid, Box<dyn Error>> {
		let mut i = 1;
		let mut query = "INSERT INTO public.transactions (id, user_id, account_id, currency_id, recipient_id, status, timestamp, comment) VALUES".to_string();
		let mut query_options: Vec<Box<(dyn ToSql + Sync)>> = Vec::new();

		for transaction in &self.transactions {
			query = format!("{query} (${}, ${}, ${}, ${}, ${}, ${}, ${}, ${}),", i, i + 1, i + 2, i + 3, i + 4, i + 5, i + 6, i + 7);
			i += 8;
			query_options.push(Box::new(transaction.id));
			query_options.push(Box::new(transaction.user_id));
			query_options.push(Box::new(transaction.account_id));
			query_options.push(Box::new(transaction.currency_id.expect("no currency_id passed into transaction::db::add")));
			query_options.push(Box::new(transaction.recipient_id));
			query_options.push(Box::new(transaction.status as i32));
			query_options.push(Box::new(transaction.timestamp));
			query_options.push(Box::new(transaction.comment.clone()));
		}
		query.pop();
		query = format!("{query};");

		let parameter_values: Vec<_> = query_options.iter()
			.map(|x| &**x as &(dyn ToSql + Sync))
			.collect();

		self.pool.get()
			.await?
			.query(query.as_str(), &parameter_values)
			.await?;
		
		i = 1;
		query = "INSERT INTO public.transaction_tags (transaction_id, tag_id) VALUES".to_string();
		query_options = Vec::new();

		for transaction in &self.transactions {
			for tag_id in transaction.tag_ids.clone() {
				query = format!("{query} (${}, ${}),", i, i + 1);
				i += 2;
				query_options.push(Box::new(transaction.id));
				query_options.push(Box::new(tag_id));
			}
		}
		query.pop();
		query = format!("{query};");

		let parameter_values: Vec<_> = query_options.iter()
			.map(|x| &**x as &(dyn ToSql + Sync))
			.collect();

		self.pool.get()
			.await?
			.query(query.as_str(), &parameter_values)
			.await?;

		i = 1;
		query = "INSERT INTO public.asset_transactions (transaction_id, asset_id) VALUES".to_string();
		query_options = Vec::new();

		for transaction in &self.transactions {
			if transaction.asset.is_none() {
				continue;
			}
			query = format!("{query} (${}, ${}),", i, i + 1);
			i += 2;
			query_options.push(Box::new(transaction.id));
			query_options.push(Box::new(transaction.asset.clone().unwrap().id));
		}
		query.pop();
		query = format!("{query};");

		let parameter_values: Vec<_> = query_options.iter()
			.map(|x| &**x as &(dyn ToSql + Sync))
			.collect();

		self.pool.get()
			.await?
			.query(query.as_str(), &parameter_values)
			.await?;

		i = 1;
		query = "INSERT INTO public.transaction_positions (id, transaction_id, amount, comment, tag_id) VALUES ".to_string();
		query_options = Vec::new();

		for transaction in self.transactions {
			for position in transaction.positions {
				query = format!("{query} (${}, ${}, ${}, ${}, ${}),", i, i + 1, i + 2, i + 3, i + 4);
				i += 5;
				query_options.push(Box::new(position.id));
				query_options.push(Box::new(transaction.id));
				query_options.push(Box::new(position.amount.to_amount()));
				query_options.push(Box::new(position.comment));
				query_options.push(Box::new(position.tag_id));
			}
		}
		query.pop();
		query = format!("{query};");

		let parameter_values: Vec<_> = query_options.iter()
			.map(|x| &**x as &(dyn ToSql + Sync))
			.collect();

		self.pool.get()
			.await?
			.query(query.as_str(), &parameter_values)
			.await?;
		return Ok(Uuid::nil());
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		return Err(Box::new(CustomError::InvalidActionForItem { action: "replace".to_string(), item_type: "Vec<Transaction>".to_string() }));
	}
}

impl<'a> DbDeleter<'a, Transaction> for TransactionDbWriter<'a> {
	async fn delete(self) -> Result<(), Box<dyn Error>> {
		let old = super::TransactionLoader::new(self.pool)
			.set_filter_id(self.transaction.id, NumberFilterModes::Exact)
			.get_first().await?;

		if old.user_id != self.transaction.user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
		}

		self.pool.get()
			.await?
			.query(
				"DELETE FROM public.transactions WHERE id=$1;", 
				&[&self.transaction.id]
			)
			.await?;

			self.pool.get()
				.await?
				.query(
				"DELETE FROM public.transaction_positions WHERE transaction_id=$1;", 
				&[&self.transaction.id]
			).await?;

		return Ok(());
	}
}

#[allow(clippy::unwrap_or_default)]
impl From<tokio_postgres::Row> for Transaction {
	fn from(value: tokio_postgres::Row) -> Transaction {
		let id: Uuid = value.get(0);
		let account_id: Uuid = value.get(1);
		let currency_id: Uuid = value.get(2);
		let recipient_id: Uuid = value.get(3);
		let status: i32 = value.get(4);
		let user_id: Uuid = value.get(5);
		let timestamp: chrono::DateTime<chrono::Utc> = value.get(6);
		let comment: Option<String> = value.get(7);
		let tag_ids: Vec<Uuid> = value.try_get(8).unwrap_or_default();
		let asset_id: Option<Uuid> = value.get(9);
		let asset_name: Option<String> = value.get(10);
		let asset_description: Option<String> = value.get(11);
		let transaction_position_ids: Vec<Uuid> = value.get(12);
		let transaction_position_amounts: Vec<Option<i32>> = value.get(13);
		let transaction_position_comments: Vec<Option<String>> = value.get(14);
		let transaction_position_tag_ids: Vec<Option<Uuid>> = value.get(15);
		let total_amount: i64 = value.get(16);
		let minor_in_major: i32 = value.get(17);
		let symbol: String = value.get(18);

		let mut asset: Option<Asset> = None;
		if asset_id.is_some() {
			asset = Some(Asset {
				id: asset_id.unwrap(),
				name: asset_name.unwrap(),
				description: asset_description,
				user_id,
				currency_id,
				value_per_unit: None,
				amount: None,
				tag_ids: Vec::new(),
				total_cost_of_ownership: None,
			});
		}

		let positions: Vec<Position> = transaction_position_ids
			.into_iter()
			.enumerate()
			.map(|(i, transaction_position_id)| {
				Position {
					id: transaction_position_id,
					amount: Money::from_amount(transaction_position_amounts[i].unwrap(), minor_in_major as u32, symbol.clone()),
					comment: transaction_position_comments[i].clone(),
					tag_id: transaction_position_tag_ids[i],
				}
			}).collect();

		return Transaction::default()
			.set_id(id)
			.set_user_id(user_id)
			.set_account_id(account_id)
			.set_currency_id(currency_id)
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