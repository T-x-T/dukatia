use super::*;

mod get_date_for_period {
	use super::*;

	#[test]
	fn daily_period() {
		let date_period = "daily";
		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();

		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();

		assert_eq!(res, expected_res);
	}

	#[test]
	fn monthly_period() {
		let date_period = "monthly";

		for i in 1..12 {
			let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, i, 5).unwrap();
			
			let res = get_date_for_period(date_period, timestamp);
			let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, i, 1).unwrap();
	
			assert_eq!(res, expected_res);
		}
	}

	#[test]
	fn quarterly_period() {
		let date_period = "quarterly";

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 1, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 1, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 2, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 1, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 3, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 1, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 4, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 4, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 5, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 4, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 6, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 4, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 7, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 7, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 8, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 7, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 9, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 7, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 10, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 10, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 11, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 10, 1).unwrap();
		assert_eq!(res, expected_res);

		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 12, 5).unwrap();
		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 10, 1).unwrap();
		assert_eq!(res, expected_res);
	}

	#[test]
	fn yearly_period() {
		let date_period = "yearly";
		let timestamp: NaiveDate = NaiveDate::from_ymd_opt(2015, 4, 5).unwrap();

		let res = get_date_for_period(date_period, timestamp);
		let expected_res: NaiveDate = NaiveDate::from_ymd_opt(2015, 1, 1).unwrap();

		assert_eq!(res, expected_res);
	}
}