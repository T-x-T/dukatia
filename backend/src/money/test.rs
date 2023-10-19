use super::*;

mod fix_minor_amounts {
	use super::*;

	#[test]
	fn works_with_one_major_too_many() {
		let money = Money { major: 10, minor: 150, minor_in_major: 100, symbol: "€".to_string(), is_negative: Some(false) };
		let money = money.fix_minor_amounts();
		assert_eq!(money.minor, 50);
		assert_eq!(money.major, 11);
	}

	#[test]
	fn works_with_two_major_too_many() {
		let money = Money { major: 10, minor: 260, minor_in_major: 100, symbol: "€".to_string(), is_negative: Some(false) };
		let money = money.fix_minor_amounts();
		assert_eq!(money.minor, 60);
		assert_eq!(money.major, 12);
	}

	#[test]
	fn works_with_zero_major_too_many() {
		let money = Money { major: 10, minor: 40, minor_in_major: 100, symbol: "€".to_string(), is_negative: Some(false) };
		let money = money.fix_minor_amounts();
		assert_eq!(money.minor, 40);
		assert_eq!(money.major, 10);
	}

	#[test]
	fn works_with_minor_equals_minor_in_major() {
		let money = Money { major: 10, minor: 100, minor_in_major: 100, symbol: "€".to_string(), is_negative: Some(false) };
		let money = money.fix_minor_amounts();
		assert_eq!(money.minor, 0);
		assert_eq!(money.major, 11);
	}

	#[test]
	fn works_with_negative_major() {
		let money = Money { major: -10, minor: 150, minor_in_major: 100, symbol: "€".to_string(), is_negative: Some(true)};
		let money = money.fix_minor_amounts();
		assert_eq!(money.minor, 50);
		assert_eq!(money.major, -11);
	}

	#[test]
	fn works_with_zero_major() {
		let money = Money { major: 0, minor: 150, minor_in_major: 100, symbol: "€".to_string(), is_negative: Some(false) };
		let money = money.fix_minor_amounts();
		assert_eq!(money.minor, 50);
		assert_eq!(money.major, 1);
	}
}

mod add {
	use super::*;

	#[test]
	fn works() {
		let a = Money::from_amount(1000, 100, "€".to_string());
		let b = Money::from_amount(12345, 100, "€".to_string());
		let money = a + b;
		assert_eq!(money.to_string(), "133.45€");
	}

	#[test]
	fn works_with_minor_being_too_large() {
		let a = Money { major: 10, minor: 150, minor_in_major: 100, symbol: "€".to_string(), is_negative: Some(false) };
		let b = Money { major: 10, minor: 250, minor_in_major: 100, symbol: "€".to_string(), is_negative: Some(false) };
		let money = a + b;
		assert_eq!(money.to_string(), "24.00€");
	}
}

mod sub {
	use super::*;

	#[test]
	fn works() {
		let a = Money::from_amount(10000, 100, "€".to_string());
		let b = Money::from_amount(123, 100, "€".to_string());
		let money = a - b;
		assert_eq!(money.to_string(), "98.77€");
	}

	#[test]
	fn works_with_rhs_larger() {
		let a = Money::from_amount(1000, 100, "€".to_string());
		let b = Money::from_amount(12345, 100, "€".to_string());
		let money = a - b;
		assert_eq!(money.to_string(), "-113.45€");
	}

	#[test]
	fn works_with_minor_being_too_large() {
		let a = Money { major: 10, minor: 150, minor_in_major: 100, symbol: "€".to_string(), is_negative: Some(false) };
		let b = Money { major: 10, minor: 250, minor_in_major: 100, symbol: "€".to_string(), is_negative: Some(false) };
		let money = a - b;
		assert_eq!(money.to_string(), "-1.00€");
	}
}

mod sum {
	use super::*;

	#[test]
	fn works() {
		let money: Money = vec![
			Money::from_amount(1000, 100, "€".to_string()),
			Money::from_amount(2000, 100, "€".to_string()),
			Money::from_amount(12345, 100, "€".to_string())
		].into_iter().sum();
		assert_eq!(money.to_string(), "153.45€");
	}

	#[test]
	fn uses_first_details() {
		let money: Money = vec![
			Money::from_amount(1000, 100, "€".to_string()),
			Money::from_amount(2000, 200, "$".to_string()),
			Money::from_amount(12345, 300, "*".to_string())
		].into_iter().sum();
		assert_eq!(money.to_string(), "153.45€");
	}
}

mod fmt_display {
	use super::*;

	#[test]
	fn works() {
		let money = Money::from_amount(12345, 100, "€".to_string());
		assert_eq!(money.to_string(), "123.45€");
	}

	#[test]
	fn works_with_negative() {
		let money = Money::from_amount(-12345, 100, "€".to_string());
		assert_eq!(money.to_string(), "-123.45€");
	}

	#[test]
	fn works_with_zero() {
		let money = Money::from_amount(0, 100, "€".to_string());
		assert_eq!(money.to_string(), "0.00€");
	}

	#[test]
	fn works_with_3_digit_minor() {
		let money = Money::from_amount(0, 1000, "!".to_string());
		assert_eq!(money.to_string(), "0.000!");
	}

	#[test]
	fn works_with_negative_cents() {
		let money = Money { major: 0, minor: 50, minor_in_major: 100, symbol: "€".to_string(), is_negative: Some(true)};
		assert_eq!(money.to_string(), "-0.50€");
	}
}

mod to_amount {
	use super::*;

	#[test]
	fn convert_correctly() {
		let money = Money::from_amount(12345, 100, "€".to_string());

		assert_eq!(money.to_amount(), 12345);
	}

	#[test]
	fn convert_correctly_negative() {
		let money = Money::from_amount(-12345, 100, "€".to_string());

		assert_eq!(money.to_amount(), -12345);
	}

	#[test]
	fn convert_correctly_zero() {
		let money = Money::from_amount(0, 100, "€".to_string());

		assert_eq!(money.to_amount(), 0);
	}

	#[test]
	fn convert_correctly_with_two_major_too_many() {
		let money = Money { major: 10, minor: 260, minor_in_major: 100, symbol: "€".to_string(), is_negative: Some(false) };
		assert_eq!(money.to_amount(), 1260);
	}

	#[test]
	fn convert_correctly_with_two_major_too_many_negative() {
		let money = Money { major: -10, minor: 260, minor_in_major: 100, symbol: "€".to_string(), is_negative: Some(true) };
		assert_eq!(money.to_amount(), -1260);
	}

	#[test]
	fn convert_correctly_with_negative_cents() {
		let money = Money::from_amount(-50, 100, "€".to_string());

		assert_eq!(money.to_amount(), -50);
	}
}

mod from_amount {
	use super::*;

	#[test]
	fn convert_correctly() {
		let res = Money::from_amount(12345, 100, "€".to_string());

		assert_eq!(res.major, 123);
		assert_eq!(res.minor, 45);
	}

	#[test]
	fn convert_correctly_negative_amount() {
		let res = Money::from_amount(-12345, 100, "€".to_string());

		assert_eq!(res.major, -123);
		assert_eq!(res.minor, 45);
	}

	#[test]
	fn convert_correctly_zero_amount() {
		let res = Money::from_amount(0, 100, "€".to_string());

		assert_eq!(res.major, 0);
		assert_eq!(res.minor, 0);
	}

	#[test]
	fn convert_correctly_with_minor_in_major_50() {
		let res = Money::from_amount(12345, 50, "€".to_string());

		assert_eq!(res.major, 246);
		assert_eq!(res.minor, 45);
	}

	#[test]
	fn correctly_sets_is_negative() {
		let res = Money::from_amount(-12345, 50, "€".to_string());

		assert_eq!(res.is_negative.unwrap(), true);
	}

	#[test]
	fn correctly_sets_is_negative_on_zero() {
		let res = Money::from_amount(0, 50, "€".to_string());

		assert_eq!(res.is_negative.unwrap(), false);
	}

	#[test]
	fn correctly_sets_is_negative_on_positive() {
		let res = Money::from_amount(1, 50, "€".to_string());

		assert_eq!(res.is_negative.unwrap(), false);
	}

	#[test]
	fn convert_correctly_with_minor_in_major_0() {
		let res = Money::from_amount(12345, 0, "€".to_string());

		assert_eq!(res.major, 12345);
		assert_eq!(res.minor, 0);
	}

	#[test]
	fn convert_correctly_with_minor_in_major_0_and_negative_amount() {
		let res = Money::from_amount(-12345, 0, "€".to_string());

		assert_eq!(res.major, -12345);
		assert_eq!(res.minor, 0);
	}
}