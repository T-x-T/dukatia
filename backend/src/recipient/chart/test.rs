use super::*;
use crate::chart::*;

fn get_recipients(n: u32) -> Vec<Recipient> {
	return (0..n).into_iter().map(|x| Recipient {id: Uuid::from_u128(x as u128), name: format!("test_{x}"), user_id: Some(Uuid::from_u128(0)), tag_ids: Vec::new()}).collect();
}

mod calculate_get_per_recipient_over_time {
	use super::*;

	#[test]
	fn no_data_default_chart() {
		let res = calculate_get_per_recipient_over_time(&ChartOptions::default(), Vec::new(), &Vec::new());

		assert_eq!(res, IntermediateChartData::default());
	}

	#[test]
	fn data_default_chart() {
		let transactions: Vec<Transaction> = vec![
			Transaction::default().set_id(Uuid::from_u128(0)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2020-01-01 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(12345, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(1)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2020-01-01 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(22345, 100, "$".to_string())).set_currency_id(1),
			Transaction::default().set_id(Uuid::from_u128(2)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2020-01-02 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(-12345, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(3)).set_recipient_id(Uuid::from_u128(1)).set_timestamp(DateTime::parse_from_str("2020-01-02 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(1000, 100, "$".to_string())).set_currency_id(1),
			Transaction::default().set_id(Uuid::from_u128(4)).set_recipient_id(Uuid::from_u128(1)).set_timestamp(DateTime::parse_from_str("2020-01-03 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(2000, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(5)).set_recipient_id(Uuid::from_u128(2)).set_timestamp(DateTime::parse_from_str("2020-01-04 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(-2100, 100, "$".to_string())).set_currency_id(1),
		];
		let res = calculate_get_per_recipient_over_time(&ChartOptions::default(), transactions, &get_recipients(5));

		assert_eq!(res, IntermediateChartData {
			datasets: [
				(Uuid::from_u128(0), Dataset{label: "test_0".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()), value: 346.9, label: "123.45€ 223.45$".to_string()},
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 1, 2).unwrap()), value: 223.45, label: "0.00€ 223.45$".to_string()},
				]}),
				(Uuid::from_u128(1), Dataset{label: "test_1".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 1, 2).unwrap()), value: 10.0, label: "10.00$".to_string()},
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 1, 3).unwrap()), value: 30.0, label: "20.00€ 10.00$".to_string()},
				]}),
				(Uuid::from_u128(2), Dataset{label: "test_2".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 1, 4).unwrap()), value: -21.0, label: "-21.00$".to_string()},
				]}),
			].iter().cloned().collect()
		});
	}

	#[test]
	fn data_default_chart_daily_period() {
		let mut chart_options = ChartOptions::default();
		chart_options.date_period = Some("daily".to_string());
		
		let transactions: Vec<Transaction> = vec![
			Transaction::default().set_id(Uuid::from_u128(0)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2020-01-01 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(12345, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(1)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2020-01-01 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(22345, 100, "$".to_string())).set_currency_id(1),
			Transaction::default().set_id(Uuid::from_u128(2)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2020-01-02 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(-12345, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(3)).set_recipient_id(Uuid::from_u128(1)).set_timestamp(DateTime::parse_from_str("2020-01-02 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(1000, 100, "$".to_string())).set_currency_id(1),
			Transaction::default().set_id(Uuid::from_u128(4)).set_recipient_id(Uuid::from_u128(1)).set_timestamp(DateTime::parse_from_str("2020-01-03 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(2000, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(5)).set_recipient_id(Uuid::from_u128(2)).set_timestamp(DateTime::parse_from_str("2020-01-04 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(-2100, 100, "$".to_string())).set_currency_id(1),
		];

		let res = calculate_get_per_recipient_over_time(&chart_options, transactions, &get_recipients(5));

		assert_eq!(res, IntermediateChartData {
			datasets: [
				(Uuid::from_u128(0), Dataset{label: "test_0".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()), value: 346.9, label: "123.45€ 223.45$".to_string()},
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 1, 2).unwrap()), value: 223.45, label: "0.00€ 223.45$".to_string()},
				]}),
				(Uuid::from_u128(1), Dataset{label: "test_1".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 1, 2).unwrap()), value: 10.0, label: "10.00$".to_string()},
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 1, 3).unwrap()), value: 30.0, label: "20.00€ 10.00$".to_string()},
				]}),
				(Uuid::from_u128(2), Dataset{label: "test_2".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 1, 4).unwrap()), value: -21.0, label: "-21.00$".to_string()},
				]}),
			].iter().cloned().collect()
		});
	}

	#[test]
	fn data_default_chart_monthly_period() {
		let mut chart_options = ChartOptions::default();
		chart_options.date_period = Some("monthly".to_string());
		
		let transactions: Vec<Transaction> = vec![
			Transaction::default().set_id(Uuid::from_u128(0)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2020-01-01 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(12345, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(1)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2020-01-01 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(22345, 100, "$".to_string())).set_currency_id(1),
			Transaction::default().set_id(Uuid::from_u128(2)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2020-02-02 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(-12345, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(3)).set_recipient_id(Uuid::from_u128(1)).set_timestamp(DateTime::parse_from_str("2020-02-02 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(1000, 100, "$".to_string())).set_currency_id(1),
			Transaction::default().set_id(Uuid::from_u128(4)).set_recipient_id(Uuid::from_u128(1)).set_timestamp(DateTime::parse_from_str("2020-03-03 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(2000, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(5)).set_recipient_id(Uuid::from_u128(2)).set_timestamp(DateTime::parse_from_str("2020-04-04 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(-2100, 100, "$".to_string())).set_currency_id(1),
		];

		let res = calculate_get_per_recipient_over_time(&chart_options, transactions, &get_recipients(5));

		assert_eq!(res, IntermediateChartData {
			datasets: [
				(Uuid::from_u128(0), Dataset{label: "test_0".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()), value: 346.9, label: "123.45€ 223.45$".to_string()},
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 2, 1).unwrap()), value: 223.45, label: "0.00€ 223.45$".to_string()},
				]}),
				(Uuid::from_u128(1), Dataset{label: "test_1".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 2, 1).unwrap()), value: 10.0, label: "10.00$".to_string()},
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 3, 1).unwrap()), value: 30.0, label: "20.00€ 10.00$".to_string()},
				]}),
				(Uuid::from_u128(2), Dataset{label: "test_2".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 4, 1).unwrap()), value: -21.0, label: "-21.00$".to_string()},
				]}),
			].iter().cloned().collect()
		});
	}

	#[test]
	fn data_default_chart_quarterly_period() {
		let mut chart_options = ChartOptions::default();
		chart_options.date_period = Some("quarterly".to_string());
		
		let transactions: Vec<Transaction> = vec![
			Transaction::default().set_id(Uuid::from_u128(0)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2020-01-01 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(12345, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(1)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2020-01-01 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(22345, 100, "$".to_string())).set_currency_id(1),
			Transaction::default().set_id(Uuid::from_u128(2)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2020-04-02 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(-12345, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(3)).set_recipient_id(Uuid::from_u128(1)).set_timestamp(DateTime::parse_from_str("2020-04-02 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(1000, 100, "$".to_string())).set_currency_id(1),
			Transaction::default().set_id(Uuid::from_u128(4)).set_recipient_id(Uuid::from_u128(1)).set_timestamp(DateTime::parse_from_str("2020-08-03 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(2000, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(5)).set_recipient_id(Uuid::from_u128(2)).set_timestamp(DateTime::parse_from_str("2020-12-04 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(-2100, 100, "$".to_string())).set_currency_id(1),
		];

		let res = calculate_get_per_recipient_over_time(&chart_options, transactions, &get_recipients(5));

		assert_eq!(res, IntermediateChartData {
			datasets: [
				(Uuid::from_u128(0), Dataset{label: "test_0".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()), value: 346.9, label: "123.45€ 223.45$".to_string()},
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 4, 1).unwrap()), value: 223.45, label: "0.00€ 223.45$".to_string()},
				]}),
				(Uuid::from_u128(1), Dataset{label: "test_1".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 4, 1).unwrap()), value: 10.0, label: "10.00$".to_string()},
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 7, 1).unwrap()), value: 30.0, label: "20.00€ 10.00$".to_string()},
				]}),
				(Uuid::from_u128(2), Dataset{label: "test_2".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 10, 1).unwrap()), value: -21.0, label: "-21.00$".to_string()},
				]}),
			].iter().cloned().collect()
		});
	}

	#[test]
	fn data_default_chart_yearly_period() {
		let mut chart_options = ChartOptions::default();
		chart_options.date_period = Some("yearly".to_string());
		
		let transactions: Vec<Transaction> = vec![
			Transaction::default().set_id(Uuid::from_u128(0)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2020-01-01 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(12345, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(1)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2020-01-01 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(22345, 100, "$".to_string())).set_currency_id(1),
			Transaction::default().set_id(Uuid::from_u128(2)).set_recipient_id(Uuid::from_u128(0)).set_timestamp(DateTime::parse_from_str("2021-04-02 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(-12345, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(3)).set_recipient_id(Uuid::from_u128(1)).set_timestamp(DateTime::parse_from_str("2021-04-02 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(1000, 100, "$".to_string())).set_currency_id(1),
			Transaction::default().set_id(Uuid::from_u128(4)).set_recipient_id(Uuid::from_u128(1)).set_timestamp(DateTime::parse_from_str("2022-08-03 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(2000, 100, "€".to_string())).set_currency_id(0),
			Transaction::default().set_id(Uuid::from_u128(5)).set_recipient_id(Uuid::from_u128(2)).set_timestamp(DateTime::parse_from_str("2023-12-04 12:34:56 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap().with_timezone(&Utc)).set_total_amount(Money::from_amount(-2100, 100, "$".to_string())).set_currency_id(1),
		];

		let res = calculate_get_per_recipient_over_time(&chart_options, transactions, &get_recipients(5));

		assert_eq!(res, IntermediateChartData {
			datasets: [
				(Uuid::from_u128(0), Dataset{label: "test_0".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()), value: 346.9, label: "123.45€ 223.45$".to_string()},
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()), value: 223.45, label: "0.00€ 223.45$".to_string()},
				]}),
				(Uuid::from_u128(1), Dataset{label: "test_1".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()), value: 10.0, label: "10.00$".to_string()},
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2022, 1, 1).unwrap()), value: 30.0, label: "20.00€ 10.00$".to_string()},
				]}),
				(Uuid::from_u128(2), Dataset{label: "test_2".to_string(), data: vec![
					DataPoint {name: None, timestamp: Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()), value: -21.0, label: "-21.00$".to_string()},
				]}),
			].iter().cloned().collect()
		});
	}
}