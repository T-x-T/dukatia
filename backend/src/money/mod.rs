#[cfg(test)]
mod test;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Money {
	major: i32,
	minor: u32,
	minor_in_major: u32,
	symbol: String,
	is_negative: Option<bool>,
}

impl Money {
	pub fn from_amount(amount: i32, minor_in_major: u32, symbol: String) -> Money {
		let major: i32 = amount / if minor_in_major == 0 { 1 } else { minor_in_major as i32 };
		let minor: u32 = if amount < 0 {
			(-amount % if minor_in_major == 0 { 1 } else { minor_in_major as i32 }) as u32
		} else {
			(amount % if minor_in_major == 0 { 1 } else { minor_in_major as i32 }) as u32
		};

		return Money { major , minor, minor_in_major, symbol, is_negative: Some(amount.is_negative()) };
	}

	pub fn to_amount(&self) -> i32 {
		if self.is_negative.is_some_and(|x| x) || self.major.is_negative() {
			return -((-self.major * self.minor_in_major as i32) + self.minor as i32);
		}
		return (self.major * self.minor_in_major as i32) + self.minor as i32;
	}

	#[allow(dead_code)]
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

	pub fn negate(mut self) -> Self {
		if self.major.is_negative() || self.is_negative.unwrap_or(false) {
			if self.major.is_negative() {
				self.major *= -1;
			}
			self.is_negative = Some(false);
		} else {
			self.major *= -1;
			self.is_negative = Some(true);
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

	fn add(self, rhs: Self) -> Self::Output {
		assert!(self.minor_in_major == rhs.minor_in_major, "Tried to add different currencies {self:?} vs {rhs:?}");

		return Money::from_amount(self.to_amount() + rhs.to_amount(), self.minor_in_major, self.symbol);
	}
}

impl std::ops::Sub for Money {
	type Output = Money;

	fn sub(self, rhs: Self) -> Self::Output {
		assert!(self.minor_in_major == rhs.minor_in_major, "Tried to sub different currencies {self:?} vs {rhs:?}");

		return Money::from_amount(self.to_amount() - rhs.to_amount(), self.minor_in_major, self.symbol);
	}
}

impl std::ops::Mul<i32> for Money {
	type Output = Money;

	fn mul(self, rhs: i32) -> Self::Output {
		return Money::from_amount(self.to_amount() * rhs, self.minor_in_major, self.symbol);
	}
}

impl std::iter::Sum<Money> for i32 {
	fn sum<I: Iterator<Item = Money>>(iter: I) -> i32 {
		return iter.map(|x| x.to_amount()).sum();
	}
}

impl std::iter::Sum<Money> for Money {
	fn sum<I: Iterator<Item = Money>>(iter: I) -> Money {
		let items: Vec<Money> = iter.collect();
		return Money::from_amount(items.iter().map(Money::to_amount).sum(), items.first().unwrap_or(&Money::default()).minor_in_major, items.first().unwrap_or(&Money::default()).symbol.clone());
	}
}