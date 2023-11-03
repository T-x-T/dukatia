use super::*;

mod actually_compute_single_budget_current_period {
	use super::*;

	#[test]
	fn default_budgets() {
		let res = actually_compute_single_budget_period(budget::Budget::default());

		assert_eq!(res, vec![("used".to_string(), ("0.0".to_string(), 0.0)), ("available".to_string(), ("0.0".to_string(), 0.0))]);
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
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(50, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(9_950, 100, "€".to_string())),
			utilization: Some(0.005),
		};

		let res = actually_compute_single_budget_period(budget);

		assert_eq!(res, vec![("used".to_string(), ("0.50€".to_string(), 50.0)), ("available".to_string(), ("99.50€".to_string(), 9950.0))]);
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
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(5_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(5_000, 100, "€".to_string())),
			utilization: Some(0.5),
		};

		let res = actually_compute_single_budget_period(budget);

		assert_eq!(res, vec![("used".to_string(), ("50.00€".to_string(), 5_000.0)), ("available".to_string(), ("50.00€".to_string(), 5_000.0))]);
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
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(9_999, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(1, 100, "€".to_string())),
			utilization: Some(0.999),
		};

		let res = actually_compute_single_budget_period(budget);

		assert_eq!(res, vec![("used".to_string(), ("99.99€".to_string(), 9_999.0)), ("available".to_string(), ("0.01€".to_string(), 1.0))]);
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
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(10_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(0, 100, "€".to_string())),
			utilization: Some(1.0),
		};

		let res = actually_compute_single_budget_period(budget);

		assert_eq!(res, vec![("used".to_string(), ("100.00€".to_string(), 10_000.0)), ("available".to_string(), ("0.00€".to_string(), 0.0))]);
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
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(15_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(0, 100, "€".to_string())),
			utilization: Some(1.5),
		};

		let res = actually_compute_single_budget_period(budget);

		assert_eq!(res, vec![("used".to_string(), ("150.00€".to_string(), 15_000.0)), ("available".to_string(), ("0.00€".to_string(), 0.0))]);
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
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(-5_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(15_000, 100, "€".to_string())),
			utilization: Some(0.0),
		};

		let res = actually_compute_single_budget_period(budget);

		assert_eq!(res, vec![("used".to_string(), ("-50.00€".to_string(), -5_000.0)), ("available".to_string(), ("150.00€".to_string(), 15_000.0))]);
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
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(15_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(-5_000, 100, "€".to_string())),
			utilization: Some(0.0),
		};

		let res = actually_compute_single_budget_period(budget);

		assert_eq!(res, vec![("used".to_string(), ("150.00€".to_string(), 15_000.0)), ("available".to_string(), ("0.00€".to_string(), 0.0))]);
	}
}