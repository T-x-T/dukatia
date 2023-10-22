pub mod rest_api;
mod db;

#[cfg(test)]
mod test;

use serde::{Serialize, Deserialize};
use serde_repr::Serialize_repr;
use std::error::Error;
use deadpool_postgres::Pool;
use chrono::prelude::*;
use crate::money::Money;
use crate::transaction::{Transaction, TransactionLoader};
use crate::traits::*;

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize, Default)]
#[repr(u8)]
pub enum Period {
	Daily = 0,
	Weekly = 1,
	#[default]
	Monthly = 2,
	Quarterly = 3,
	Yearly = 4,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct Budget {
	pub id: Option<u32>,
  pub name: String,
	pub user_id: u32,
	pub amount: Money,
	pub rollover: bool,
	pub period: Period,
	pub filter_tag_ids: Vec<u32>,
	pub currency_id: u32,
	pub active_from: DateTime<Utc>,
	pub active_to: Option<DateTime<Utc>>,
	pub used_amount: Option<Money>,
	pub available_amount: Option<Money>,
	pub utilization: Option<f64>
}

impl Save for Budget {
	async fn save(self, pool: &Pool) -> Result<u32, Box<dyn Error>> {
		match self.id {
			Some(id) => {
				db::BudgetDbWriter::new(pool, self).replace().await?;
				return Ok(id)
			},
			None => return db::BudgetDbWriter::new(pool, self).insert().await,
		}
	}
}

impl Delete for Budget {
	async fn delete(self, pool: &Pool) -> Result<(), Box<dyn Error>> {
		match self.id {
			Some(_) => return db::BudgetDbWriter::new(pool, self).delete().await,
			None => return Err(Box::new(crate::CustomError::MissingProperty { property: "id".to_string(), item_type: "Budget".to_string() }))
		}
	}
}

impl Budget {
	pub fn set_id(mut self, id: u32) -> Self {
		self.id = Some(id);
		return self;
	}

	pub fn set_name(mut self, name: String) -> Self {
		self.name = name;
		return self;
	}

	pub fn set_user_id(mut self, user_id: u32) -> Self {
		self.user_id = user_id;
		return self;
	}

	pub fn set_amount(mut self, amount: Money) -> Self {
		self.amount = amount;
		return self;
	}

	pub fn set_rollover(mut self, rollover: bool) -> Self {
		self.rollover = rollover;
		return self;
	}

	pub fn set_period(mut self, period: Period) -> Self {
		self.period = period;
		return self;
	}

	pub fn set_filter_tag_ids(mut self, filter_tag_ids: Vec<u32>) -> Self {
		self.filter_tag_ids = filter_tag_ids;
		return self;
	}

	pub fn set_currency_id(mut self, currency_id: u32) -> Self {
		self.currency_id = currency_id;
		return self;
	}

	pub fn set_active_from(mut self, active_from: DateTime<Utc>) -> Self {
		self.active_from = active_from;
		return self;
	}

	pub fn set_active_to_opt(mut self, active_to: Option<DateTime<Utc>>) -> Self {
		self.active_to = active_to;
		return self;
	}

	pub async fn calculate_utilization(mut self, pool: &Pool) -> Result<Self, Box<dyn Error>> {
		let mut period = self.get_period_at_timestamp(Utc::now());
		
		if period.0 < self.active_from {
			period.0 = self.active_from;
		}
		if self.active_to.is_some() && period.1 > self.active_to.unwrap() {
			period.1 = self.active_to.unwrap();
		}

		let transactions = self.get_transactions(pool, period.0, period.1).await?;

		if transactions.is_empty() {
			self.used_amount = Some(Money::from_amount(0, self.amount.get_minor_in_major(), self.amount.get_symbol()));
		} else {
			self.used_amount = Some(transactions.into_iter().map(|x| x.total_amount.unwrap_or(Money::from_amount(0, self.amount.get_minor_in_major(), self.amount.get_symbol())).negate()).sum());
		}

		self.available_amount = Some(self.clone().amount - self.clone().used_amount.unwrap());
		self.utilization = Some(f64::from(self.clone().used_amount.unwrap().to_amount()) / f64::from(self.clone().amount.to_amount()));

		return Ok(self);
	}

	#[allow(dead_code)]
	pub async fn get_all_transactions(&self, pool: &Pool) -> Result<Vec<Transaction>, Box<dyn Error>> {
		return self.get_transactions(pool, DateTime::<Utc>::MIN_UTC, DateTime::<Utc>::MAX_UTC).await;
	}

	pub async fn get_transactions(&self, pool: &Pool, from_timestamp: DateTime<Utc>, to_timestamp: DateTime<Utc>) -> Result<Vec<Transaction>, Box<dyn Error>> {
		let mut transactions: Vec<Transaction> = Vec::new();

		let mut retrieved_tag_ids: Vec<u32> = Vec::new();

		for tag_id in &self.filter_tag_ids {
			transactions.append(
				&mut TransactionLoader::new(pool)
					.set_filter_tag_id(*tag_id, NumberFilterModes::Exact)
					.set_filter_time_range(from_timestamp, to_timestamp, TimeRangeFilterModes::Between)
					.set_filter_currency_id(self.currency_id, NumberFilterModes::Exact)
					.get()
					.await?
					.into_iter()
					.filter(|transaction| !transaction.tag_ids.clone().unwrap_or_default().iter().any(|tag_id| retrieved_tag_ids.contains(tag_id)))
					.collect()
			);

			retrieved_tag_ids.push(*tag_id);
		}

		return Ok(transactions);
	}

	fn get_period_at_timestamp(&self, timestamp: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
		match self.period {
			Period::Daily => {
				let start = timestamp.date_naive().and_time(NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap()).and_utc();	
				let end = timestamp.date_naive().and_time(NaiveTime::parse_from_str("23:59:59", "%H:%M:%S").unwrap()).and_utc();

				return (start, end);
			},
			Period::Weekly => {
				let start = timestamp.date_naive().week(Weekday::Mon).first_day().and_time(NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap()).and_utc();	
				let end = timestamp.date_naive().week(Weekday::Mon).last_day().and_time(NaiveTime::parse_from_str("23:59:59", "%H:%M:%S").unwrap()).and_utc();	

				return (start, end);
			},
			Period::Monthly => {
				let start = NaiveDate::from_ymd_opt(timestamp.year(), timestamp.month(), 1).unwrap().and_time(chrono::NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap()).and_utc();	
				let end = if timestamp.month() < 12 {
					NaiveDate::from_ymd_opt(timestamp.year(), timestamp.month() + 1, 1).unwrap().pred_opt().unwrap()
				} else {
					NaiveDate::from_ymd_opt(timestamp.year() + 1, 1, 1).unwrap().pred_opt().unwrap()
				}.and_time(chrono::NaiveTime::parse_from_str("23:59:59", "%H:%M:%S").unwrap()).and_utc();

				return (start, end);
			},
			Period::Quarterly => {
				let start_month = match timestamp.month() {
					1..=3 => 1,
					4..=6 => 4,
					7..=9 => 7,
					_ => 10,
				};

				let start = NaiveDate::from_ymd_opt(timestamp.year(), start_month, 1).unwrap().and_time(chrono::NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap()).and_utc();	
				let end = if start_month + 3 < 12 {
					NaiveDate::from_ymd_opt(timestamp.year(), start_month + 3, 1).unwrap().pred_opt().unwrap()
				} else {
					NaiveDate::from_ymd_opt(timestamp.year() + 1, 1, 1).unwrap().pred_opt().unwrap()
				}.and_time(chrono::NaiveTime::parse_from_str("23:59:59", "%H:%M:%S").unwrap()).and_utc();

				return (start, end);
			},
			Period::Yearly => {
				let start = NaiveDate::from_ymd_opt(timestamp.year(), 1, 1).unwrap().and_time(chrono::NaiveTime::parse_from_str("00:00:00", "%H:%M:%S").unwrap()).and_utc();	
				let end = NaiveDate::from_ymd_opt(timestamp.year(), 12, 31).unwrap().and_time(chrono::NaiveTime::parse_from_str("23:59:59", "%H:%M:%S").unwrap()).and_utc();	

				return (start, end);
			},
		}
	}
}

#[derive(Debug, Clone)]
pub struct BudgetLoader<'a> {
	pool: &'a Pool,
	query_parameters: QueryParameters,
}

impl<'a> BudgetLoader<'a> {
	async fn get_full(self) -> Result<Vec<Budget>, Box<dyn Error>> {
		let res = self.clone().get().await?;

		let mut budgets: Vec<Budget> = Vec::new();

		for x in res {
			budgets.push(x.calculate_utilization(self.pool).await?);
		}

		return Ok(budgets);
	}

	async fn get_first_full(self) -> Result<Budget, Box<dyn Error>> {
		let res = self.clone().get_first().await?;

		return res.calculate_utilization(self.pool).await;
	}
}

impl<'a> Loader<'a, Budget> for BudgetLoader<'a> {
	fn new(pool: &'a Pool) -> Self {
		Self {
			pool,
			query_parameters: QueryParameters::default(),
		}
	}

	async fn get(self) -> Result<Vec<Budget>, Box<dyn Error>> {
		return db::BudgetDbReader::new(self.pool)
			.set_query_parameters(self.query_parameters)
			.execute()
			.await;
	}

	fn get_query_parameters(&self) -> &QueryParameters {
		return &self.query_parameters;
	}

	fn set_query_parameters(mut self, query_parameters: QueryParameters) -> Self {
		self.query_parameters = query_parameters;
		return self;
	}
}