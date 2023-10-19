use chrono::DateTime;
use super::*;

mod get_period_at_timestamp {
	use super::*;

	#[test]
	fn daily_period() {
		let budget = Budget::default()
			.set_period(Period::Daily);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-10-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-10-19 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-10-19 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn weekly_period() {
		let budget = Budget::default()
			.set_period(Period::Weekly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-10-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-10-16 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-10-22 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn monthly_period() {
		let budget = Budget::default()
			.set_period(Period::Monthly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-10-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn monthly_period_dec() {
		let budget = Budget::default()
			.set_period(Period::Monthly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-12-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-12-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn monthly_period_jan() {
		let budget = Budget::default()
			.set_period(Period::Monthly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-01-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-01-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-01-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn monthly_period_feb() {
		let budget = Budget::default()
			.set_period(Period::Monthly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-02-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-02-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-02-28 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn monthly_period_feb_leap_year() {
		let budget = Budget::default()
			.set_period(Period::Monthly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2020-02-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2020-02-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2020-02-29 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn quarterly_period_1() {
		let budget = Budget::default()
			.set_period(Period::Quarterly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-03-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-01-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-03-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn quarterly_period_2() {
		let budget = Budget::default()
			.set_period(Period::Quarterly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-04-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-04-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-06-30 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn quarterly_period_3() {
		let budget = Budget::default()
			.set_period(Period::Quarterly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-09-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-07-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-09-30 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn quarterly_period_4() {
		let budget = Budget::default()
			.set_period(Period::Quarterly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-12-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn yearly_period() {
		let budget = Budget::default()
			.set_period(Period::Yearly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-01-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-12-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}
}