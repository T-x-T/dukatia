use chrono::Datelike;
use std::collections::BTreeMap;
use serde::Serialize;
use super::Transaction;

#[derive(Debug, Serialize, Clone)]
pub struct Timeseries {
	pub data: BTreeMap<chrono::NaiveDate, i32>
}

impl Timeseries {
	pub fn build(transactions: Vec<&Transaction>) -> Self {
		let mut data: BTreeMap<chrono::NaiveDate, i32> = BTreeMap::new();
	
		for transaction in transactions {
			if data.contains_key(&transaction.timestamp.date().naive_local()) {
				data.insert(transaction.timestamp.date().naive_local(), data.get(&transaction.timestamp.date().naive_local()).unwrap() + transaction.amount);
			} else {
				data.insert(transaction.timestamp.date().naive_local(), transaction.amount);
			}
		}
	
		return Self{data};
	}
	
	pub fn create_rolling_sum(mut self) -> Self {
		let mut prev: i32 = 0;
		for i in 0..self.data.len() {
			let cur_key = self.data.iter().nth(i).unwrap().0.clone();
			let cur_val = self.data.iter().nth(i).unwrap().1.clone();
			self.data.insert(cur_key, prev + cur_val);
			prev = self.data.iter().nth(i).unwrap().1.clone();
		}
		return self;
	}

	pub fn create_sum_aggregate_monthly(mut self) -> Self {
		self.data.insert(chrono::NaiveDate::from_yo(9999, 1), 0); //For the for..in loop to properly register the last value
		let mut output: Timeseries = Timeseries { data: BTreeMap::new() };

		if self.data.len() == 0 {
			return output;
		}

		let first_raw_date = self.data.iter().nth(0).unwrap().0;
		let first_date = chrono::NaiveDate::from_ymd(first_raw_date.year(), first_raw_date.month(), 1);
		let current_date = chrono::NaiveDate::from_ymd(chrono::Local::today().year(), chrono::Local::today().month(), 1);
		let mut all_dates: Vec<chrono::NaiveDate> = vec![first_date];

		while all_dates.last().unwrap() != &current_date {
			let cur = all_dates.last().unwrap().clone();
			if cur.month() == 12 {
				all_dates.push(chrono::NaiveDate::from_ymd(cur.year() + 1, 1, 1));
			} else {
				all_dates.push(chrono::NaiveDate::from_ymd(cur.year(), cur.month() + 1, 1));
			}
		}

		all_dates.iter().for_each(|date| {
			output.data.insert(*date, 0);
		});

		let mut current_month_amount: i32 = 0;
		let mut current_month: chrono::NaiveDate = chrono::NaiveDate::from_yo(1, 1);
		
		for i in 0..self.data.len() {
			let year = self.data.iter().nth(i).unwrap().0.clone().year();
			let month = self.data.iter().nth(i).unwrap().0.clone().month();
			let month_date = chrono::NaiveDate::from_ymd(year, month, 1);
			
			if month_date > current_month {
				if i > 0 { 
					output.data.insert(current_month, current_month_amount);
				}
				current_month = month_date;
				current_month_amount = 0;
			}
			current_month_amount = current_month_amount + self.data.iter().nth(i).unwrap().1;
		}

		return output;
	}

	pub fn create_sum_aggregate_quarterly(mut self) -> Self {
		self.data.insert(chrono::NaiveDate::from_yo(9999, 1), 0); //For the for..in loop to properly register the last value
		let mut output: Timeseries = Timeseries { data: BTreeMap::new() };

		if self.data.len() == 0 {
			return output;
		}

		let first_raw_date = self.data.iter().nth(0).unwrap().0;
		let first_date = chrono::NaiveDate::from_ymd(first_raw_date.year(), get_quarter_of_month(first_raw_date.month()), 1);
		let current_date = chrono::NaiveDate::from_ymd(chrono::Local::today().year(), get_quarter_of_month(chrono::Local::today().month()), 1);
		let mut all_dates: Vec<chrono::NaiveDate> = vec![first_date];

		while all_dates.last().unwrap() != &current_date {
			let cur = all_dates.last().unwrap().clone();
			if cur.month() == 10 {
				all_dates.push(chrono::NaiveDate::from_ymd(cur.year() + 1, 1, 1));
			} else {
				all_dates.push(chrono::NaiveDate::from_ymd(cur.year(), cur.month() + 3, 1));
			}
		}

		all_dates.iter().for_each(|date| {
			output.data.insert(*date, 0);
		});

		let mut current_quarter_amount: i32 = 0;
		let mut current_quarter: chrono::NaiveDate = chrono::NaiveDate::from_yo(1, 1);
		
		for i in 0..self.data.len() {
			let year = self.data.iter().nth(i).unwrap().0.clone().year();
			let month = get_quarter_of_month(self.data.iter().nth(i).unwrap().0.clone().month());
			let quarter_date = chrono::NaiveDate::from_ymd(year, month, 1);
			
			if quarter_date > current_quarter {
				if i > 0 { 
					output.data.insert(current_quarter, current_quarter_amount);
				}
				current_quarter = quarter_date;
				current_quarter_amount = 0;
			}
			current_quarter_amount = current_quarter_amount + self.data.iter().nth(i).unwrap().1;
		}

		return output;
	}

	pub fn create_sum_aggregate_yearly(mut self) -> Self {
		self.data.insert(chrono::NaiveDate::from_yo(9999, 1), 0); //For the for..in loop to properly register the last value
		let mut output: Timeseries = Timeseries { data: BTreeMap::new() };

		if self.data.len() == 0 {
			return output;
		}

		let first_raw_date = self.data.iter().nth(0).unwrap().0;
		let first_date = chrono::NaiveDate::from_ymd(first_raw_date.year(), 1, 1);
		let current_date = chrono::NaiveDate::from_ymd(chrono::Local::today().year(), 1, 1);
		let mut all_dates: Vec<chrono::NaiveDate> = vec![first_date];

		while all_dates.last().unwrap() != &current_date {
			let cur = all_dates.last().unwrap().clone();
			all_dates.push(chrono::NaiveDate::from_ymd(cur.year() + 1, 1, 1));
		}

		all_dates.iter().for_each(|date| {
			output.data.insert(*date, 0);
		});

		let mut current_year_amount: i32 = 0;
		let mut current_year: chrono::NaiveDate = chrono::NaiveDate::from_yo(1, 1);
		
		for i in 0..self.data.len() {
			let year = self.data.iter().nth(i).unwrap().0.clone().year();
			let year_date = chrono::NaiveDate::from_ymd(year, 1, 1);
			
			if year_date > current_year {
				if i > 0 { 
					output.data.insert(current_year, current_year_amount);
				}
				current_year = year_date;
				current_year_amount = 0;
			}
			current_year_amount = current_year_amount + self.data.iter().nth(i).unwrap().1;
		}

		return output;
	}
}

fn get_quarter_of_month(input: u32) -> u32 {
	if vec![1, 2, 3].contains(&input) {
		return 1;
	} else if vec![4, 5, 6].contains(&input) {
		return 4;
	} else	if vec![7, 8, 9].contains(&input) {
		return 7;
	}else {
		return 10;
	}
}
