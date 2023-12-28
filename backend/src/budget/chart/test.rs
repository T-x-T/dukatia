use super::*;
use crate::budget::Period;

mod calculate_get_all_budget_utilization_overview {
	use super::*;

	#[test]
	fn two_budgets() {
		let budget1 = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(50, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(9_950, 100, "€".to_string())),
			utilization: Some(0.005),
		};

		let budget2 = Budget {
			id: Some(1),
			name: "test2".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "$".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 1,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(5_000, 100, "$".to_string())),
			available_amount: Some(Money::from_amount(5_000, 100, "$".to_string())),
			utilization: Some(0.5),
		};

		let res = calculate_get_all_budget_utilization_overview(vec![budget1, budget2]);

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: 0.50,
								label: "0.50€".to_string(),
							},
							DataPoint { 
								name: Some("test2".to_string()),
								timestamp: None,
								value: 50.00,
								label: "50.00$".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: 99.50,
								label: "99.50€".to_string(),
							},
							DataPoint { 
								name: Some("test2".to_string()),
								timestamp: None,
								value: 50.00,
								label: "50.00$".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}
}

mod calculate_get_single_budget_current_period_utilization {
	use super::*;

	#[test]
	fn default_budgets() {
		let res = calculate_get_single_budget_current_period_utilization(Budget::default());

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: Some("".to_string()),
								timestamp: None,
								value: 0.0,
								label: "0.0".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: Some("".to_string()),
								timestamp: None,
								value: 0.0,
								label: "0.0".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn less_than_one_percent_utilization() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(50, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(9_950, 100, "€".to_string())),
			utilization: Some(0.005),
		};

		let res = calculate_get_single_budget_current_period_utilization(budget);

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: 0.50,
								label: "0.50€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: 99.50,
								label: "99.50€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn fifty_percent_utilization() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(5_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(5_000, 100, "€".to_string())),
			utilization: Some(0.5),
		};

		let res = calculate_get_single_budget_current_period_utilization(budget);

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: 50.00,
								label: "50.00€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: 50.00,
								label: "50.00€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn almost_hundred_percent_utilization() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(9_999, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(1, 100, "€".to_string())),
			utilization: Some(0.999),
		};

		let res = calculate_get_single_budget_current_period_utilization(budget);

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: 99.99,
								label: "99.99€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: 0.01,
								label: "0.01€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn hundred_percent_utilization() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(10_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(0, 100, "€".to_string())),
			utilization: Some(1.0),
		};

		let res = calculate_get_single_budget_current_period_utilization(budget);

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: 100.00,
								label: "100.00€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: 00.00,
								label: "0.00€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn over_hundred_percent_utilization() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(15_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(0, 100, "€".to_string())),
			utilization: Some(1.5),
		};

		let res = calculate_get_single_budget_current_period_utilization(budget);

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: 150.00,
								label: "150.00€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: 0.00,
								label: "0.00€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn negative_used_amount() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(-5_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(15_000, 100, "€".to_string())),
			utilization: Some(0.0),
		};

		let res = calculate_get_single_budget_current_period_utilization(budget);

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: -50.00,
								label: "-50.00€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: 150.00,
								label: "150.00€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn negative_available_amount() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(15_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(-5_000, 100, "€".to_string())),
			utilization: Some(0.0),
		};

		let res = calculate_get_single_budget_current_period_utilization(budget);

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: 150.00,
								label: "150.00€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: Some("test".to_string()),
								timestamp: None,
								value: -50.00,
								label: "-50.00€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}
}

mod calculate_get_single_budget_utilization_history {
	use super::*;

	#[test]
	fn default_budgets() {
		let res = calculate_get_single_budget_utilization_history(&Budget::default(), (
			DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()
		));

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 0.0,
								label: "0.0".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 0.0,
								label: "0.0".to_string(),
							},
						] 
					}),
					(2, Dataset { 
						label: "total".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 0.0,
								label: "0.0".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn less_than_one_percent_utilization() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(50, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(9_950, 100, "€".to_string())),
			utilization: Some(0.005),
		};

		let res = calculate_get_single_budget_utilization_history(&budget, (
			DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()
		));

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 0.50,
								label: "0.50€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 99.50,
								label: "99.50€".to_string(),
							},
						] 
					}),
					(2, Dataset { 
						label: "total".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 100.00,
								label: "100.00€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn fifty_percent_utilization() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(5_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(5_000, 100, "€".to_string())),
			utilization: Some(0.5),
		};

		let res = calculate_get_single_budget_utilization_history(&budget, (
			DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()
		));

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 50.00,
								label: "50.00€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 50.00,
								label: "50.00€".to_string(),
							},
						] 
					}),
					(2, Dataset { 
						label: "total".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 100.00,
								label: "100.00€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn almost_hundred_percent_utilization() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(9_999, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(1, 100, "€".to_string())),
			utilization: Some(0.999),
		};

		let res = calculate_get_single_budget_utilization_history(&budget, (
			DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()
		));

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 99.99,
								label: "99.99€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 0.01,
								label: "0.01€".to_string(),
							},
						] 
					}),
					(2, Dataset { 
						label: "total".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 100.00,
								label: "100.00€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn hundred_percent_utilization() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(10_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(0, 100, "€".to_string())),
			utilization: Some(1.0),
		};

		let res = calculate_get_single_budget_utilization_history(&budget, (
			DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()
		));

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 100.00,
								label: "100.00€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 00.00,
								label: "0.00€".to_string(),
							},
						] 
					}),
					(2, Dataset { 
						label: "total".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 100.00,
								label: "100.00€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn over_hundred_percent_utilization() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(15_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(0, 100, "€".to_string())),
			utilization: Some(1.5),
		};

		let res = calculate_get_single_budget_utilization_history(&budget, (
			DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()
		));

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 150.00,
								label: "150.00€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 0.00,
								label: "0.00€".to_string(),
							},
						] 
					}),
					(2, Dataset { 
						label: "total".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 100.00,
								label: "100.00€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn negative_used_amount() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(-5_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(15_000, 100, "€".to_string())),
			utilization: Some(0.0),
		};

		let res = calculate_get_single_budget_utilization_history(&budget, (
			DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()
		));

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: -50.00,
								label: "-50.00€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 150.00,
								label: "150.00€".to_string(),
							},
						] 
					}),
					(2, Dataset { 
						label: "total".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 100.00,
								label: "100.00€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn negative_available_amount() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: false,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: chrono::Utc::now(),
			active_to: None,
			used_amount: Some(Money::from_amount(15_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(-5_000, 100, "€".to_string())),
			utilization: Some(0.0),
		};

		let res = calculate_get_single_budget_utilization_history(&budget, (
			DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			DateTime::parse_from_str("2023-10-31 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()
		));

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 150.00,
								label: "150.00€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: -50.00,
								label: "-50.00€".to_string(),
							},
						] 
					}),
					(2, Dataset { 
						label: "total".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
								value: 100.00,
								label: "100.00€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}

	#[test]
	fn rollover_enabled() {
		let budget = Budget {
			id: Some(0),
			name: "test".to_string(),
			user_id: 0,
			amount: Money::from_amount(10_000, 100, "€".to_string()),
			rollover: true,
			period: Period::Monthly,
			filter_tag_ids: vec![0],
			currency_id: 0,
			active_from: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			active_to: Some(DateTime::parse_from_str("2024-09-15 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()),
			used_amount: Some(Money::from_amount(5_000, 100, "€".to_string())),
			available_amount: Some(Money::from_amount(5_000, 100, "€".to_string())),
			utilization: Some(0.5),
		};

		let res = calculate_get_single_budget_utilization_history(&budget, (
			DateTime::parse_from_str("2024-09-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(),
			DateTime::parse_from_str("2024-09-30 23:59:59 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into()
		));

		assert_eq!(res, 
			IntermediateChartData {
				datasets: vec![
					(0, Dataset { 
						label: "used".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2024, 9, 1),
								value: 50.00,
								label: "50.00€".to_string(),
							},
						] 
					}),
					(1, Dataset { 
						label: "available".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2024, 9, 1),
								value: 50.00,
								label: "50.00€".to_string(),
							},
						] 
					}),
					(2, Dataset { 
						label: "total".to_string(),
						data: vec![
							DataPoint { 
								name: None,
								timestamp: NaiveDate::from_ymd_opt(2024, 9, 1),
								value: 1200.00,
								label: "1200.00€".to_string(),
							},
						] 
					}),
				].into_iter().collect()
			}
		);
	}
}