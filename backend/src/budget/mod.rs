pub mod rest_api;
pub mod chart;
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

	#[allow(unused)]
	pub async fn calculate_utilization_of_current_period(self, pool: &Pool) -> Result<Self, Box<dyn Error>> {
		return self.calculate_utilization_of_period_at(pool, Utc::now()).await;
	}

	pub async fn calculate_utilization_of_period_at(mut self, pool: &Pool, timestamp: DateTime<Utc>) -> Result<Self, Box<dyn Error>> {
		let period = self.get_period_at_timestamp(timestamp);
		let mut date_range = period;
		let mut period_count: i32 = 1;

		if date_range.1 < self.active_from || (self.active_to.is_some() && date_range.0 > self.active_to.unwrap()) {
			self.used_amount = Some(Money::from_amount(0, self.amount.get_minor_in_major(), self.amount.get_symbol()));
			self.available_amount = Some(Money::from_amount(0, self.amount.get_minor_in_major(), self.amount.get_symbol()));
			self.utilization = Some(0.0);

			return Ok(self);
		}

		if date_range.0 < self.active_from {
			date_range.0 = self.active_from;
		}
		if self.active_to.is_some() && date_range.1 > self.active_to.unwrap() {
			date_range.1 = self.active_to.unwrap();
		}

		if self.rollover {
			date_range.0 = self.active_from;
			period_count = self.get_period_count(date_range.0, date_range.1);
		}

		let transactions = self.get_transactions(pool, date_range.0, date_range.1).await?;
		
		let mut full_used_amount = Money::from_amount(0, self.amount.get_minor_in_major(), self.amount.get_symbol());
		if !transactions.is_empty() {
			full_used_amount = transactions.iter()
				.map(|x| x.total_amount.clone().unwrap_or(Money::from_amount(0, self.amount.get_minor_in_major(), self.amount.get_symbol())).negate())
				.sum();
		}


		if transactions.is_empty() {
			self.used_amount = Some(Money::from_amount(0, self.amount.get_minor_in_major(), self.amount.get_symbol()));
		} else {
			let sum: Money = transactions.into_iter()
				.filter(|x| x.timestamp > period.0)
				.map(|x| x.total_amount.unwrap_or(Money::from_amount(0, self.amount.get_minor_in_major(), self.amount.get_symbol())).negate())
				.sum();

			self.used_amount = if sum.clone().to_amount() == 0 && sum.get_symbol().is_empty() {
				Some(Money::from_amount(0, self.amount.get_minor_in_major(), self.amount.get_symbol()))
			} else {
				Some(sum)
			};
		}

		self.available_amount = Some(self.clone().amount * period_count - full_used_amount);
		self.utilization = Some(f64::from(self.clone().used_amount.unwrap().to_amount()) / (f64::from(self.clone().amount.to_amount() * period_count)));

		return Ok(self);
	}

	pub async fn get_transactions_of_period_at(&self, pool: &Pool, timestamp: DateTime<Utc>) -> Result<Vec<Transaction>, Box<dyn Error>> {
		let period = self.get_period_at_timestamp(timestamp);
		let mut date_range = period;

		if date_range.0 < self.active_from {
			date_range.0 = self.active_from;
		}
		if self.active_to.is_some() && date_range.1 > self.active_to.unwrap() {
			date_range.1 = self.active_to.unwrap();
		}
		
		return self.get_transactions(pool, date_range.0, date_range.1).await;
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

	pub fn get_period_count(&self, from_timestamp: DateTime<Utc>, to_timestamp: DateTime<Utc>) -> i32 {
		match self.period {
			Period::Daily => {
				let from_day_count = from_timestamp.num_days_from_ce();
				let to_day_count = to_timestamp.num_days_from_ce();
				
				return to_day_count - from_day_count;
			},
			Period::Weekly => {
				let from_day_count = f64::from(from_timestamp.num_days_from_ce()) / 7.0;
				let to_day_count = f64::from(to_timestamp.num_days_from_ce()) / 7.0;

				return (to_day_count - from_day_count).ceil() as i32;
			},
			Period::Monthly => {
				if from_timestamp.year() == to_timestamp.year() {
					return (to_timestamp.month() - from_timestamp.month()) as i32 + 1;
				}

				let years = to_timestamp.year() - from_timestamp.year();
				return ((to_timestamp.month() + 12) - from_timestamp.month()) as i32 + (years * 12) - 11;
			},
			Period::Quarterly => {
				if from_timestamp.year() == to_timestamp.year() {
					return (f64::from(to_timestamp.month() - from_timestamp.month()) / 4.0).ceil() as i32;
				}

				let years = to_timestamp.year() - from_timestamp.year();
				return ((f64::from((to_timestamp.month() + 12) - from_timestamp.month()) - 12.0) / 4.0).ceil() as i32 + (years * 4);
			},
			Period::Yearly => {
				return to_timestamp.year() - from_timestamp.year() + 1;
			},
		}
	}

	pub fn get_past_and_current_periods(&self, max_date: DateTime<Utc>) -> Vec<(DateTime<Utc>, DateTime<Utc>)> {
		let mut output: Vec<(DateTime<Utc>, DateTime<Utc>)> = vec![self.get_period_at_timestamp(self.active_from)];
		
		loop {
			#[allow(clippy::if_same_then_else)]
			if self.active_to.is_some() && (output.last().unwrap().1 >= self.active_to.unwrap()) {
				break;
			} else if output.last().unwrap().1 >= max_date {
				break;
			}

			output.push(self.get_period_at_timestamp(output.last().unwrap().1.checked_add_days(chrono::Days::new(1)).unwrap()));
		}

		return output;
	}

	pub fn get_period_at_timestamp(&self, timestamp: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
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
	pub async fn get_full(self) -> Result<Vec<Budget>, Box<dyn Error>> {
		return self.get_full_at(Utc::now()).await;
	}

	pub async fn get_first_full(self) -> Result<Budget, Box<dyn Error>> {
		return self.get_first_full_at(Utc::now()).await;
	}

	pub async fn get_full_at(self, timestamp: DateTime<Utc>) -> Result<Vec<Budget>, Box<dyn Error>> {
		let res = self.clone().get().await?;

		let mut budgets: Vec<Budget> = Vec::new();

		for x in res {
			budgets.push(x.calculate_utilization_of_period_at(self.pool, timestamp).await?);
		}

		return Ok(budgets);
	}

	pub async fn get_first_full_at(self, timestamp: DateTime<Utc>) -> Result<Budget, Box<dyn Error>> {
		let res = self.clone().get_first().await?;

		return res.calculate_utilization_of_period_at(self.pool, timestamp).await;
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