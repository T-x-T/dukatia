use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Money {
	major: i32,
	minor: u32,
	minor_in_major: u32,
	symbol: String,
	is_negative: Option<bool>,
}

impl Money {
	pub fn from_amount(amount: i32, minor_in_major: u32, symbol: String) -> Money {
		let major: i32 = amount / minor_in_major as i32;
		let minor: u32 = if amount < 0 {
			(-amount % minor_in_major as i32) as u32
		} else {
			(amount % minor_in_major as i32) as u32
		};

		return Money { major , minor, minor_in_major, symbol, is_negative: Some(amount.is_negative()) };
	}

	pub fn to_amount(&self) -> i32 {
		if self.is_negative.is_some_and(|x| x) || self.major.is_negative() {
			return -((-self.major * self.minor_in_major as i32) + self.minor as i32);
		}
		return (self.major * self.minor_in_major as i32) + self.minor as i32;
	}

	fn fix_minor_amounts(mut self) -> Self {
		while self.minor >= self.minor_in_major {
			self.minor -= self.minor_in_major;
			if self.is_negative.is_some_and(|x| x) || self.major.is_negative() {
				self.major -= 1;
			} else {
				self.major += 1;
			}
		}
		return self;
	}

	pub fn get_minor_in_major(&self) -> u32 {
		return self.minor_in_major;
	}

	pub fn get_symbol(&self) -> String {
		return self.symbol.clone();
	}
}

impl std::fmt::Display for Money {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let minor_digits = self.minor_in_major.to_string().len() - 1;
		return write!(f, "{}{}.{:0minor_digits$}{}", if self.is_negative.is_some_and(|x| x) && self.major == 0 { "-" } else {""}, self.major, self.minor, self.symbol);
	}
}

impl std::ops::Add for Money {
	type Output = Money;

	fn add(mut self, rhs: Self) -> Self::Output {
		assert!(self.minor_in_major == rhs.minor_in_major, "Tried to add different currencies {self:?} vs {rhs:?}");

		self.minor += rhs.minor;
		self.major += rhs.major;
		
		self = self.fix_minor_amounts();

		return self;
	}
}

impl std::iter::Sum<Money> for i32 {
	fn sum<I: Iterator<Item = Money>>(iter: I) -> i32 {
		return iter.map(|x| x.to_amount()).sum();
	}
}


#[cfg(test)]
mod test {
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
	}
}