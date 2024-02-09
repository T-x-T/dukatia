pub mod rest_api;

use deadpool_postgres::Pool;
use std::error::Error;
use crate::CustomError;
use crate::traits::*;
use crate::money::Money;

use crate::account::Account;
use crate::tag::Tag;
use crate::recipient::Recipient;
use crate::asset::Asset;
use crate::budget::{Budget, Period};
use crate::transaction::{Transaction, Position, TransactionStatus};

pub async fn insert_demo_data(pool: &Pool, user_id: u32) -> Result<(), Box<dyn Error>> {
	if user_has_data(pool, user_id).await? {
		return Err(Box::new(CustomError::InvalidActionForItem { action: "create demo data for user with preexisting data".to_string(), item_type: "user".to_string() }));
	}

	for account in get_accounts(user_id) {
		account.save(pool).await?;
	}

	for tag in get_tags(user_id) {
		tag.save(pool).await?;
	}

	for recipient in get_recipients(user_id) {
		recipient.save(pool).await?;
	}

	for asset in get_assets(user_id) {
		asset.save(pool).await?;
	}

	for budget in get_budgets(user_id) {
		budget.save(pool).await?;
	}

	for transaction in get_transactions(user_id) {
		transaction.save(pool).await?;
	}

	return Ok(());
}

async fn user_has_data(pool: &Pool, user_id: u32) -> Result<bool, Box<dyn Error>> {
	if crate::account::AccountLoader::new(pool)
		.set_filter_user_id(user_id, NumberFilterModes::Exact)
		.get_first().await.is_ok() {
			return Ok(true);
		}

	if crate::asset::AssetLoader::new(pool)
		.set_filter_user_id(user_id, NumberFilterModes::Exact)
		.get_first().await.is_ok() {
			return Ok(true);
		}

	if crate::budget::BudgetLoader::new(pool)
		.set_filter_user_id(user_id, NumberFilterModes::Exact)
		.get_first().await.is_ok() {
			return Ok(true);
		}

	if crate::recipient::RecipientLoader::new(pool)
		.set_filter_user_id(user_id, NumberFilterModes::Exact)
		.get_first().await.is_ok() {
			return Ok(true);
		}

	if crate::tag::TagLoader::new(pool)
		.set_filter_user_id(user_id, NumberFilterModes::Exact)
		.get_first().await.is_ok() {
			return Ok(true);
		}

	if crate::transaction::TransactionLoader::new(pool)
		.set_filter_user_id(user_id, NumberFilterModes::Exact)
		.get_first().await.is_ok() {
			return Ok(true);
		}

	return Ok(false);
}

fn get_accounts(user_id: u32) -> Vec<Account> {
	return vec![
		Account {name: "Main street bank".to_string(), default_currency_id: 0, user_id, ..Default::default()},
		Account {name: "US of Banks".to_string(), default_currency_id: 1, user_id, ..Default::default()},
		Account {name: "PayBuddy".to_string(), default_currency_id: 0, user_id, ..Default::default()},
	];
}

fn get_tags(user_id: u32) -> Vec<Tag> {
	return vec![
		Tag {name: "Housing".to_string(), user_id, ..Default::default()},
		Tag {name: "Transportation".to_string(), user_id, ..Default::default()},
		Tag {name: "Food".to_string(), user_id, ..Default::default()},
		Tag {name: "Insurance".to_string(), user_id, ..Default::default()},
		Tag {name: "Entertainment".to_string(), user_id, ..Default::default()},
		Tag {name: "Clothing".to_string(), user_id, ..Default::default()},
	];
}

fn get_recipients(user_id: u32) -> Vec<Recipient> {
	return vec![
		Recipient {name: "Landlord inc".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "Car dealerzz".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "Tire shop".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "The Car wash company".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "Transport for Paxterya".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "Entire Foods".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "brutto".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "ldil".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "real supermarket corp".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "insurance inc".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "smarter than your doc insurance ltd".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "Banana Computers inc".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "meanflix".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "bloxbuster".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "ttlstore.com".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "Optimum".to_string(), user_id: Some(user_id), ..Default::default()},
		Recipient {name: "M+H".to_string(), user_id: Some(user_id), ..Default::default()},
	];
}

fn get_assets(user_id: u32) -> Vec<Asset> {
	return vec![
		Asset {name: "Banana Computers inc".to_string(), user_id, ..Default::default()},
		Asset {name: "All universe ETF".to_string(), user_id, ..Default::default()},
		Asset {name: "gamestart".to_string(), user_id, ..Default::default()},
	];
}

fn get_budgets(user_id: u32) -> Vec<Budget> {
	return vec![
		Budget {name: "Waste".to_string(), user_id, amount: Money::from_amount(100_000, 100, "€".to_string()), period: Period::Monthly, currency_id: 0, ..Default::default()},
		Budget {name: "Electronics".to_string(), user_id, amount: Money::from_amount(200_000, 100, "€".to_string()), period: Period::Yearly, currency_id: 0, ..Default::default()},
		Budget {name: "Food".to_string(), user_id, amount: Money::from_amount(15_000, 100, "€".to_string()), period: Period::Weekly, currency_id: 0, ..Default::default()},
	];
}

fn get_transactions(user_id: u32) -> Vec<Transaction> {
	return vec![
		Transaction {user_id, currency_id: Some(0), status: TransactionStatus::Completed, timestamp: chrono::Utc::now(), positions: vec![Position {amount: Money::from_amount(150_000, 100, "€".to_string()), ..Default::default()}], ..Default::default()},
	];
}