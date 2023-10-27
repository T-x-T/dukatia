use chrono::DateTime;
use super::*;

mod get_period_at_timestamp {
	use super::*;

	#[test]
	fn daily_period() {
		let budget = Budget::default().set_period(Period::Daily);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-10-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-10-19 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-10-19 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn weekly_period() {
		let budget = Budget::default().set_period(Period::Weekly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-10-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-10-16 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-10-22 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn monthly_period() {
		let budget = Budget::default().set_period(Period::Monthly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-10-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn monthly_period_dec() {
		let budget = Budget::default().set_period(Period::Monthly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-12-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-12-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn monthly_period_jan() {
		let budget = Budget::default().set_period(Period::Monthly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-01-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-01-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-01-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn monthly_period_feb() {
		let budget = Budget::default().set_period(Period::Monthly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-02-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-02-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-02-28 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn monthly_period_feb_leap_year() {
		let budget = Budget::default().set_period(Period::Monthly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2020-02-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2020-02-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2020-02-29 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn quarterly_period_1() {
		let budget = Budget::default().set_period(Period::Quarterly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-03-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-01-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-03-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn quarterly_period_2() {
		let budget = Budget::default().set_period(Period::Quarterly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-04-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-04-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-06-30 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn quarterly_period_3() {
		let budget = Budget::default().set_period(Period::Quarterly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-09-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-07-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-09-30 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn quarterly_period_4() {
		let budget = Budget::default().set_period(Period::Quarterly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-12-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}

	#[test]
	fn yearly_period() {
		let budget = Budget::default().set_period(Period::Yearly);
		let res = budget.get_period_at_timestamp(DateTime::parse_from_str("2023-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());
		
		assert_eq!(res.0, DateTime::parse_from_str("2023-01-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
		assert_eq!(res.1, DateTime::parse_from_str("2023-12-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap());
	}
}

mod get_period_count {
	use super::*;

	#[test]
	fn daily() {
		let budget = Budget::default().set_period(Period::Daily);
		let res = budget.get_period_count(DateTime::parse_from_str("2021-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());

		assert_eq!(res, 730);
	}

	#[test]
	fn weekly() {
		let budget = Budget::default().set_period(Period::Weekly);
		let res = budget.get_period_count(DateTime::parse_from_str("2021-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());

		assert_eq!(res, 105);
	}

	#[test]
	fn monthly_same_year() {
		let budget = Budget::default().set_period(Period::Monthly);
		let res = budget.get_period_count(DateTime::parse_from_str("2023-06-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());

		assert_eq!(res, 7);
	}

	#[test]
	fn monthly_different_year() {
		let budget = Budget::default().set_period(Period::Monthly);
		let res = budget.get_period_count(DateTime::parse_from_str("2021-06-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());

		assert_eq!(res, 31);
	}

	#[test]
	fn quarterly_same_year() {
		let budget = Budget::default().set_period(Period::Quarterly);
		let res = budget.get_period_count(DateTime::parse_from_str("2023-06-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());

		assert_eq!(res, 2);
	}

	#[test]
	fn quarterly_different_year() {
		let budget = Budget::default().set_period(Period::Quarterly);
		let res = budget.get_period_count(DateTime::parse_from_str("2021-06-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());

		assert_eq!(res, 10);
	}

	#[test]
	fn yearly_different_year() {
		let budget = Budget::default().set_period(Period::Yearly);
		let res = budget.get_period_count(DateTime::parse_from_str("2021-06-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-12-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());

		assert_eq!(res, 3);
	}
}

mod get_past_and_current_periods {
	use super::*;

	#[test]
	fn works_with_active_to_in_past() {
		let res = Budget::default()
			.set_period(Period::Monthly)
			.set_active_from(DateTime::parse_from_str("2021-06-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into())
			.set_active_to_opt(Some(DateTime::parse_from_str("2022-06-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()))
			.get_past_and_current_periods(DateTime::parse_from_str("4022-06-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());

		assert_eq!(res.first().unwrap().clone(), (Into::<DateTime<Utc>>::into(DateTime::parse_from_str("2021-06-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()), Into::<DateTime<Utc>>::into(DateTime::parse_from_str("2021-06-30 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap())));
		assert_eq!(res.last().unwrap().clone(), (Into::<DateTime<Utc>>::into(DateTime::parse_from_str("2022-06-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()), Into::<DateTime<Utc>>::into(DateTime::parse_from_str("2022-06-30 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap())));
		assert_eq!(res.len(), 13);
	}

	#[test]
	fn works_with_active_to_in_future() {
		let res = Budget::default()
			.set_period(Period::Monthly)
			.set_active_from(DateTime::parse_from_str("2021-06-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into())
			.set_active_to_opt(Some(DateTime::parse_from_str("3022-06-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()))
			.get_past_and_current_periods(DateTime::parse_from_str("2023-10-27 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());

		assert_eq!(res.first().unwrap().clone(), (Into::<DateTime<Utc>>::into(DateTime::parse_from_str("2021-06-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()), Into::<DateTime<Utc>>::into(DateTime::parse_from_str("2021-06-30 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap())));
		assert_eq!(res.last().unwrap().clone(), (Into::<DateTime<Utc>>::into(DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()), Into::<DateTime<Utc>>::into(DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap())));
		assert_eq!(res.len(), 29);
	}

	#[test]
	fn works_with_active_to_none() {
		let res = Budget::default()
			.set_period(Period::Monthly)
			.set_active_from(DateTime::parse_from_str("2021-06-19 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into())
			.get_past_and_current_periods(DateTime::parse_from_str("2023-10-27 13:56:04 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into());

		assert_eq!(res.first().unwrap().clone(), (Into::<DateTime<Utc>>::into(DateTime::parse_from_str("2021-06-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()), Into::<DateTime<Utc>>::into(DateTime::parse_from_str("2021-06-30 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap())));
		assert_eq!(res.last().unwrap().clone(), (Into::<DateTime<Utc>>::into(DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap()), Into::<DateTime<Utc>>::into(DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap())));
		assert_eq!(res.len(), 29);
	}
}