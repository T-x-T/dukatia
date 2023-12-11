use super::*;
use crate::money::Money;

mod calculate_get_single_asset_total_value_over_time {
	use super::*;

	#[test]
	fn works() {
		let asset = Asset::default().set_id(0);
		let asset_valuation_history: Vec<AssetValuation> = vec![
			AssetValuation { value_per_unit: Money::from_amount(100, 100, "€".to_string()), amount: 1.0, timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), asset_id: 0 },
			AssetValuation { value_per_unit: Money::from_amount(200, 100, "€".to_string()), amount: 2.0, timestamp: DateTime::parse_from_str("2023-10-02 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), asset_id: 0 },
			AssetValuation { value_per_unit: Money::from_amount(123, 100, "€".to_string()), amount: 2.5, timestamp: DateTime::parse_from_str("2023-10-06 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), asset_id: 0 },
			AssetValuation { value_per_unit: Money::from_amount(150, 100, "€".to_string()), amount: 0.0, timestamp: DateTime::parse_from_str("2023-11-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), asset_id: 0 },
		];

		let res = calculate_get_single_asset_total_value_over_time(asset, &asset_valuation_history);

		assert_eq!(res, IntermediateChartData {
			datasets: vec![
				(0, Dataset { 
					label: "".to_string(),
					data: vec![
						DataPoint { 
							name: None,
							timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
							value: 1.0,
							label: "1.00€".to_string()
						},
						DataPoint { 
							name: None,
							timestamp: NaiveDate::from_ymd_opt(2023, 10, 2),
							value: 4.0,
							label: "4.00€".to_string()
						},
						DataPoint { 
							name: None,
							timestamp: NaiveDate::from_ymd_opt(2023, 10, 6),
							value: 3.075,
							label: "3.08€".to_string()
						},
						DataPoint { 
							name: None,
							timestamp: NaiveDate::from_ymd_opt(2023, 11, 1),
							value: 0.0,
							label: "0.00€".to_string()
						},
					]
				})
			].into_iter().collect()
		})
	}
}

mod calculate_get_single_asset_single_value_over_time {
	use super::*;

	#[test]
	fn works() {
		let asset = Asset::default().set_id(0);
		let asset_valuation_history: Vec<AssetValuation> = vec![
			AssetValuation { value_per_unit: Money::from_amount(100, 100, "€".to_string()), amount: 1.0, timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), asset_id: 0 },
			AssetValuation { value_per_unit: Money::from_amount(200, 100, "€".to_string()), amount: 2.0, timestamp: DateTime::parse_from_str("2023-10-02 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), asset_id: 0 },
			AssetValuation { value_per_unit: Money::from_amount(1234, 100, "€".to_string()), amount: 2.5, timestamp: DateTime::parse_from_str("2023-10-06 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), asset_id: 0 },
			AssetValuation { value_per_unit: Money::from_amount(150, 100, "€".to_string()), amount: 0.0, timestamp: DateTime::parse_from_str("2023-11-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), asset_id: 0 },
		];

		let res = calculate_get_single_asset_single_value_over_time(asset, &asset_valuation_history);

		assert_eq!(res, IntermediateChartData {
			datasets: vec![
				(0, Dataset { 
					label: "".to_string(),
					data: vec![
						DataPoint { 
							name: None,
							timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
							value: 1.0,
							label: "1.00€".to_string()
						},
						DataPoint { 
							name: None,
							timestamp: NaiveDate::from_ymd_opt(2023, 10, 2),
							value: 2.0,
							label: "2.00€".to_string()
						},
						DataPoint { 
							name: None,
							timestamp: NaiveDate::from_ymd_opt(2023, 10, 6),
							value: 12.34,
							label: "12.34€".to_string()
						},
						DataPoint { 
							name: None,
							timestamp: NaiveDate::from_ymd_opt(2023, 11, 1),
							value: 1.5,
							label: "1.50€".to_string()
						},
					]
				})
			].into_iter().collect()
		})
	}
}

mod calculate_get_single_asset_amount_over_time {
	use super::*;

	#[test]
	fn works() {
		let asset = Asset::default().set_id(0);
		let asset_valuation_history: Vec<AssetValuation> = vec![
			AssetValuation { value_per_unit: Money::from_amount(100, 100, "€".to_string()), amount: 1.0, timestamp: DateTime::parse_from_str("2023-10-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), asset_id: 0 },
			AssetValuation { value_per_unit: Money::from_amount(200, 100, "€".to_string()), amount: 2.0, timestamp: DateTime::parse_from_str("2023-10-02 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), asset_id: 0 },
			AssetValuation { value_per_unit: Money::from_amount(1234, 100, "€".to_string()), amount: 2.5, timestamp: DateTime::parse_from_str("2023-10-06 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), asset_id: 0 },
			AssetValuation { value_per_unit: Money::from_amount(150, 100, "€".to_string()), amount: 0.0, timestamp: DateTime::parse_from_str("2023-11-01 00:00:00 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().into(), asset_id: 0 },
		];

		let res = calculate_get_single_asset_amount_over_time(asset, &asset_valuation_history);

		assert_eq!(res, IntermediateChartData {
			datasets: vec![
				(0, Dataset { 
					label: "".to_string(),
					data: vec![
						DataPoint { 
							name: None,
							timestamp: NaiveDate::from_ymd_opt(2023, 10, 1),
							value: 1.0,
							label: "1".to_string()
						},
						DataPoint { 
							name: None,
							timestamp: NaiveDate::from_ymd_opt(2023, 10, 2),
							value: 2.0,
							label: "2".to_string()
						},
						DataPoint { 
							name: None,
							timestamp: NaiveDate::from_ymd_opt(2023, 10, 6),
							value: 2.5,
							label: "2.5".to_string()
						},
						DataPoint { 
							name: None,
							timestamp: NaiveDate::from_ymd_opt(2023, 11, 1),
							value: 0.0,
							label: "0".to_string()
						},
					]
				})
			].into_iter().collect()
		})
	}
}