use deadpool_postgres::Pool;
use chrono::{DateTime, Utc};
use std::error::Error;
use super::super::CustomError;
use super::{Budget, Period};
use crate::money::Money;
use crate::traits::*;

#[derive(Debug)]
pub struct BudgetDbReader<'a> {
	query_parameters: QueryParameters,
	pool: &'a Pool
}

impl<'a> DbReader<'a, Budget> for BudgetDbReader<'a> {
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

	async fn execute(self) -> Result<Vec<Budget>, Box<dyn Error>> {
		let query = "SELECT * FROM public.budget_data";
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
pub struct BudgetDbWriter<'a> {
	pool: &'a Pool,
	budget: Budget,
}

impl<'a> DbWriter<'a, Budget> for BudgetDbWriter<'a> {
	fn new(pool: &'a Pool, item: Budget) -> Self {
		Self {
			pool,
			budget: item,
		}
	}

	async fn insert(self) -> Result<u32, Box<dyn Error>> {
		let client = self.pool.get().await.unwrap();
		let id: i32 = client
			.query(
				"INSERT INTO public.budgets (id, name, user_id, amount, rollover, period, currency_id, active_from, active_to) VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8) RETURNING id;",
				&[&self.budget.name, &(self.budget.user_id as i32), &(self.budget.amount.to_amount()), &self.budget.rollover, &(self.budget.period as i32), &(self.budget.currency_id as i32), &self.budget.active_from, &self.budget.active_to]
			)
			.await?
			[0].get(0);
	
		for tag_id in &self.budget.filter_tag_ids {
			client.query(
				"INSERT INTO public.budget_filter_tags (budget_id, tag_id) VALUES ($1, $2);",
				&[&id, &(*tag_id as i32)]
			).await?;
		}
	
		return Ok(id as u32);
	}

	async fn replace(self) -> Result<(), Box<dyn Error>> {
		if self.budget.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty{property: String::from("id"), item_type: String::from("budget")}));
		}
	
		let old = super::BudgetLoader::new(self.pool)
			.set_filter_id(self.budget.id.unwrap(), NumberFilterModes::Exact)
			.get_first().await?;

		if old.user_id != self.budget.user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
		}
	
		let client = self.pool.get().await?;
		
		client.query(
				"UPDATE public.budgets SET name=$1, amount=$2, rollover=$3, period=$4, currency_id=$5, active_from=$6, active_to=$7 WHERE id=$8;",
				&[&self.budget.name, &(self.budget.amount.to_amount()), &self.budget.rollover, &(self.budget.period as i32), &(self.budget.currency_id as i32), &self.budget.active_from, &self.budget.active_to, &(self.budget.id.unwrap() as i32)]
			)
			.await?;
		
		client.query(
				"DELETE FROM public.budget_filter_tags WHERE budget_id=$1",
				&[&(self.budget.id.unwrap() as i32)]
			).await?;
	
			for tag_id in &self.budget.filter_tag_ids {
				client.query(
					"INSERT INTO public.budget_filter_tags (budget_id, tag_id) VALUES ($1, $2);",
					&[&(self.budget.id.unwrap() as i32), &(*tag_id as i32)]
				).await?;
			}
	
		return Ok(());
	}
}


impl<'a> DbDeleter<'a, Budget> for BudgetDbWriter<'a> {
	async fn delete(self) -> Result<(), Box<dyn Error>> {
		if self.budget.id.is_none() {
			return Err(Box::new(CustomError::MissingProperty{property: String::from("id"), item_type: String::from("budget")}));
		}

		let old = super::BudgetLoader::new(self.pool)
			.set_filter_id(self.budget.id.unwrap(), NumberFilterModes::Exact)
			.get_first().await?;

		if old.user_id != self.budget.user_id {
			return Err(Box::new(CustomError::UserIsntOwner));
		}

		self.pool.get()
			.await?
			.query("DELETE FROM public.budgets WHERE id=$1;", &[&(self.budget.id.unwrap() as i32)]).await?;
	
		return Ok(());
	}
}

#[allow(clippy::unwrap_or_default)]
impl From<tokio_postgres::Row> for Budget {
	fn from(value: tokio_postgres::Row) -> Self {
		let id: i32 = value.get(0);
		let name: String = value.get(1);
		let user_id: i32 = value.get(2);
		let amount: i32 = value.get(3);
		let rollover: bool = value.get(4);
		let period: i32 = value.get(5);
		let filter_tag_ids = value
			.try_get(6)
			.unwrap_or(Vec::new())
			.into_iter()
			.map(|x: i32| x as u32)
			.collect();
		let active_from: DateTime<Utc> = value.get(7);
		let active_to: Option<DateTime<Utc>> = value.get(8);
		let minor_in_major: i32 = value.get(9);
		let symbol: String = value.get(10);
		let currency_id: i32 = value.get(11);
		
		return Budget {
			id: Some(id as u32),
			name,
			user_id: user_id as u32,
			amount: Money::from_amount(amount, minor_in_major as u32, symbol),
			rollover,
			period: match period {
				0 => Period::Daily,
				1 => Period::Weekly,
				2 => Period::Monthly,
				3 => Period::Quarterly,
				4 => Period::Yearly,
				_ => panic!("unknown period found in budgets table"),
			},
			filter_tag_ids,
			active_from,
			active_to,
			currency_id: currency_id as u32,
			used_amount: None,
			available_amount: None,
			utilization: None,
		};
	}
}