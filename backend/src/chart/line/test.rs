use super::*;
use chrono::{DateTime, Utc, NaiveDateTime};

mod get_date_for_period {
	use super::*;

	#[test]
	fn daily_period() {
		let date_period = "daily";
		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(), Utc);

		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d").unwrap(), Utc);

		assert_eq!(res, expected_res);
	}

	#[test]
	fn monthly_period() {
		let date_period = "monthly";

		for i in 1..12 {
			let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str(format!("2015-{i}-05 23:56:04").as_str(), "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
			
			let res = get_date_for_period(date_period, &timestamp);
			let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str(format!("2015-{i}-01").as_str(), "%Y-%m-%d").unwrap(), Utc);
	
			assert_eq!(res, expected_res);
		}
	}

	#[test]
	fn quarterly_period() {
		let date_period = "quarterly";

		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str(format!("2015-01-05 23:56:04").as_str(), "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str(format!("2015-01-01").as_str(), "%Y-%m-%d").unwrap(), Utc);
		assert_eq!(res, expected_res);

		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str(format!("2015-02-05 23:56:04").as_str(), "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str(format!("2015-01-01").as_str(), "%Y-%m-%d").unwrap(), Utc);
		assert_eq!(res, expected_res);

		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str(format!("2015-03-05 23:56:04").as_str(), "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str(format!("2015-01-01").as_str(), "%Y-%m-%d").unwrap(), Utc);
		assert_eq!(res, expected_res);

		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str(format!("2015-04-05 23:56:04").as_str(), "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str(format!("2015-04-01").as_str(), "%Y-%m-%d").unwrap(), Utc);
		assert_eq!(res, expected_res);

		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str(format!("2015-05-05 23:56:04").as_str(), "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str(format!("2015-04-01").as_str(), "%Y-%m-%d").unwrap(), Utc);
		assert_eq!(res, expected_res);

		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str(format!("2015-06-05 23:56:04").as_str(), "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str(format!("2015-04-01").as_str(), "%Y-%m-%d").unwrap(), Utc);
		assert_eq!(res, expected_res);

		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str(format!("2015-07-05 23:56:04").as_str(), "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str(format!("2015-07-01").as_str(), "%Y-%m-%d").unwrap(), Utc);
		assert_eq!(res, expected_res);

		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str(format!("2015-08-05 23:56:04").as_str(), "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str(format!("2015-07-01").as_str(), "%Y-%m-%d").unwrap(), Utc);
		assert_eq!(res, expected_res);

		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str(format!("2015-09-05 23:56:04").as_str(), "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str(format!("2015-07-01").as_str(), "%Y-%m-%d").unwrap(), Utc);
		assert_eq!(res, expected_res);

		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str(format!("2015-10-05 23:56:04").as_str(), "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str(format!("2015-10-01").as_str(), "%Y-%m-%d").unwrap(), Utc);
		assert_eq!(res, expected_res);

		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str(format!("2015-11-05 23:56:04").as_str(), "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str(format!("2015-10-01").as_str(), "%Y-%m-%d").unwrap(), Utc);
		assert_eq!(res, expected_res);

		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str(format!("2015-12-05 23:56:04").as_str(), "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str(format!("2015-10-01").as_str(), "%Y-%m-%d").unwrap(), Utc);
		assert_eq!(res, expected_res);
	}

	#[test]
	fn yearly_period() {
		let date_period = "yearly";
		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(), Utc);

		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str("2015-01-01", "%Y-%m-%d").unwrap(), Utc);

		assert_eq!(res, expected_res);
	}
}

mod actually_compute_single_budget_utilization_history_part {
	use super::*;

	#[test]
	fn default_budgets() {
		let res = actually_compute_single_budget_utilization_history_part(&budget::Budget::default(), (DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()));

		assert_eq!(res.0, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: 0.0, label: "0.0".to_string() });
		assert_eq!(res.1, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: 0.0, label: "0.0".to_string() });
	}

	#[test]
	fn less_than_one_percent_utilization() {
		let budget = budget::Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: budget::Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: DateTime::parse_from_str("2022-10-12 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			active_to: None,
			used_amount: Some(Money::from_amount(50, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(9_950, 100, "€".to_string())),
			utilization: Some(0.005),
		};

		let res = actually_compute_single_budget_utilization_history_part(&budget, (DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()));

		assert_eq!(res.0, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: 0.50, label: "0.50€".to_string() });
		assert_eq!(res.1, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: 99.50, label: "99.50€".to_string() });
	}

	#[test]
	fn fifty_percent_utilization() {
		let budget = budget::Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: budget::Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: DateTime::parse_from_str("2022-10-12 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			active_to: None,
			used_amount: Some(Money::from_amount(5_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(5_000, 100, "€".to_string())),
			utilization: Some(0.5),
		};

		let res = actually_compute_single_budget_utilization_history_part(&budget, (DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()));

		assert_eq!(res.0, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: 50.0, label: "50.00€".to_string() });
		assert_eq!(res.1, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: 50.0, label: "50.00€".to_string() });
	}

	#[test]
	fn almost_hundred_percent_utilization() {
		let budget = budget::Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: budget::Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: DateTime::parse_from_str("2022-10-12 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			active_to: None,
			used_amount: Some(Money::from_amount(9_999, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(1, 100, "€".to_string())),
			utilization: Some(0.999),
		};

		let res = actually_compute_single_budget_utilization_history_part(&budget, (DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()));

		assert_eq!(res.0, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: 99.99, label: "99.99€".to_string() });
		assert_eq!(res.1, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: 0.01, label: "0.01€".to_string() });
	}

	#[test]
	fn hundred_percent_utilization() {
		let budget = budget::Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: budget::Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: DateTime::parse_from_str("2022-10-12 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			active_to: None,
			used_amount: Some(Money::from_amount(10_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(0, 100, "€".to_string())),
			utilization: Some(1.0),
		};

		let res = actually_compute_single_budget_utilization_history_part(&budget, (DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()));

		assert_eq!(res.0, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: 100.0, label: "100.00€".to_string() });
		assert_eq!(res.1, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: 0.0, label: "0.00€".to_string() });
	}

	#[test]
	fn over_hundred_percent_utilization() {
		let budget = budget::Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: budget::Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: DateTime::parse_from_str("2022-10-12 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			active_to: None,
			used_amount: Some(Money::from_amount(15_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(0, 100, "€".to_string())),
			utilization: Some(1.5),
		};

		let res = actually_compute_single_budget_utilization_history_part(&budget, (DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()));

		assert_eq!(res.0, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: 150.0, label: "150.00€".to_string() });
		assert_eq!(res.1, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: 0.0, label: "0.00€".to_string() });
	}

	#[test]
	fn negative_used_amount() {
		let budget = budget::Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: budget::Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: DateTime::parse_from_str("2022-10-12 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			active_to: None,
			used_amount: Some(Money::from_amount(-5_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(15_000, 100, "€".to_string())),
			utilization: Some(0.0),
		};

		let res = actually_compute_single_budget_utilization_history_part(&budget, (DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()));

		assert_eq!(res.0, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: -50.0, label: "-50.00€".to_string() });
		assert_eq!(res.1, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: 150.0, label: "150.00€".to_string() });
	}

	#[test]
	fn negative_available_amount() {
		let budget = budget::Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: budget::Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: DateTime::parse_from_str("2022-10-12 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			active_to: None,
			used_amount: Some(Money::from_amount(15_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(-5_000, 100, "€".to_string())),
			utilization: Some(0.0),
		};

		let res = actually_compute_single_budget_utilization_history_part(&budget, (DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()));

		assert_eq!(res.0, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: 150.0, label: "150.00€".to_string() });
		assert_eq!(res.1, Point{ timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), value: -50.0, label: "-50.00€".to_string() });
	}
}