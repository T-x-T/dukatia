use super::*;
use chrono::{DateTime, Utc, NaiveDateTime};

mod get_date_for_period {
	use super::*;

	#[tokio::test(flavor = "multi_thread")]
	async fn daily_period() {
		let date_period = "daily";
		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(), Utc);

		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d").unwrap(), Utc);

		assert_eq!(res, expected_res);
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn monthly_period() {
		let date_period = "monthly";

		for i in 1..12 {
			let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str(format!("2015-{i}-05 23:56:04").as_str(), "%Y-%m-%d %H:%M:%S").unwrap(), Utc);
			
			let res = get_date_for_period(date_period, &timestamp);
			let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str(format!("2015-{i}-01").as_str(), "%Y-%m-%d").unwrap(), Utc);
	
			assert_eq!(res, expected_res);
		}
	}

	#[tokio::test(flavor = "multi_thread")]
	async fn quarterly_period() {
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

	#[tokio::test(flavor = "multi_thread")]
	async fn yearly_period() {
		let date_period = "yearly";
		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(), Utc);

		let res = get_date_for_period(date_period, &timestamp);
		let expected_res: Date<Utc> = Date::from_utc(NaiveDate::parse_from_str("2015-01-01", "%Y-%m-%d").unwrap(), Utc);

		assert_eq!(res, expected_res);
	}
}