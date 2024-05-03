pub mod rest_api;

use deadpool_postgres::Pool;
use chrono::prelude::*;
use rand::Rng;
use uuid::Uuid;
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
use crate::currency;

pub async fn insert_demo_data(pool: &Pool, user_id: Uuid) -> Result<(), Box<dyn Error>> {
	if user_has_data(pool, user_id).await? {
		return Err(Box::new(CustomError::InvalidActionForItem { action: "create demo data for user with preexisting data".to_string(), item_type: "user".to_string() }));
	}

	let currencies = currency::CurrencyLoader::new(pool).get().await?;

	let accounts = get_accounts(user_id, &currencies);
	let tags = get_tags(user_id);
	let recipients = get_recipients(user_id);
	let assets = get_assets(user_id, &currencies);
	let budgets = get_budgets(user_id, &currencies, &tags);
	let transactions = get_transactions(user_id, &currencies, &recipients, &accounts, &tags, &assets);

	for account in accounts.clone() {
		account.create(pool).await?;
	}
	
	for tag in tags.clone() {
		tag.create(pool).await?;
	}

	for recipient in recipients.clone() {
		recipient.create(pool).await?;
	}

	for asset in assets.clone() {
		asset.create(pool).await?;
	}

	for budget in budgets.clone() {
		budget.create(pool).await?;
	}

	transactions.clone().create(pool).await?;

	return Ok(());
}

async fn user_has_data(pool: &Pool, user_id: Uuid) -> Result<bool, Box<dyn Error>> {
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

fn get_accounts(user_id: Uuid, currencies: &[currency::Currency]) -> Vec<Account> {
	let first_currency = currencies.iter().filter(|c| c.symbol == "€").last().unwrap_or(currencies.first().unwrap());
	let second_currency = currencies.iter().filter(|c| c.symbol == "$").last().unwrap_or(currencies.get(1).unwrap());

	return vec![
		Account {name: "Main street bank".to_string(), default_currency_id: first_currency.id, user_id, ..Default::default()},
		Account {name: "US of Banks".to_string(), default_currency_id: second_currency.id, user_id, ..Default::default()},
		Account {name: "PayBuddy".to_string(), default_currency_id: first_currency.id, user_id, ..Default::default()},
	];
}

fn get_tags(user_id: Uuid) -> Vec<Tag> {
	return vec![
		Tag {name: "Housing".to_string(), user_id, ..Default::default()},
		Tag {name: "Transportation".to_string(), user_id, ..Default::default()},
		Tag {name: "Food".to_string(), user_id, ..Default::default()},
		Tag {name: "Insurance".to_string(), user_id, ..Default::default()},
		Tag {name: "Entertainment".to_string(), user_id, ..Default::default()},
		Tag {name: "Clothing".to_string(), user_id, ..Default::default()},
		Tag {name: "Payday".to_string(), user_id, ..Default::default()},
	];
}

fn get_recipients(user_id: Uuid) -> Vec<Recipient> {
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
		Recipient {name: "Big corp ltd".to_string(), user_id: Some(user_id), ..Default::default()},
	];
}

fn get_assets(user_id: Uuid, currencies: &[currency::Currency]) -> Vec<Asset> {
	let first_currency = currencies.iter().filter(|c| c.symbol == "€").last().unwrap_or(currencies.first().unwrap());

	return vec![
		Asset {name: "Banana Computers inc".to_string(), user_id, currency_id: first_currency.id, ..Default::default()},
		Asset {name: "All universe ETF".to_string(), user_id, currency_id: first_currency.id, ..Default::default()},
		Asset {name: "gamestart".to_string(), user_id, currency_id: first_currency.id, ..Default::default()},
	];
}

fn get_budgets(user_id: Uuid, currencies: &[currency::Currency], tags: &[Tag]) -> Vec<Budget> {
	let first_currency = currencies.iter().filter(|c| c.symbol == "€").last().unwrap_or(currencies.first().unwrap());
	let mut rng = rand::thread_rng();
	let filtered_tags: Vec<&Tag> = tags.iter().filter(|x| x.name != "Payday").collect();

	return vec![
		Budget {name: "Waste".to_string(), user_id, amount: Money::from_amount(100_000, 100, "€".to_string()), period: Period::Monthly, currency_id: first_currency.id, filter_tag_ids: vec![(*filtered_tags.get(rng.gen_range(0..filtered_tags.len())).unwrap()).clone().id], ..Default::default()},
		Budget {name: "Electronics".to_string(), user_id, amount: Money::from_amount(200_000, 100, "€".to_string()), period: Period::Yearly, currency_id: first_currency.id, filter_tag_ids: vec![(*filtered_tags.get(rng.gen_range(0..filtered_tags.len())).unwrap()).clone().id], ..Default::default()},
		Budget {name: "Food".to_string(), user_id, amount: Money::from_amount(15_000, 100, "€".to_string()), period: Period::Weekly, currency_id: first_currency.id, filter_tag_ids: vec![(*filtered_tags.get(rng.gen_range(0..filtered_tags.len())).unwrap()).clone().id], ..Default::default()},
	];
}

fn get_transactions(user_id: Uuid, currencies: &[currency::Currency], recipients: &[Recipient], accounts: &[Account], tags: &[Tag], assets: &[Asset]) -> Vec<Transaction> {
	let mut rng = rand::thread_rng();
	
	return (0..1000).map(|i| {
		let recipient = recipients.get(rng.gen_range(0..recipients.len())).unwrap();
		let account = accounts.get(rng.gen_range(0..accounts.len())).unwrap();
		let currency: currency::Currency = currencies.iter().filter(|x| x.id == account.default_currency_id).cloned().collect::<Vec<currency::Currency>>().first().unwrap().clone();
		let tag: Tag = if recipient.name == *"Big corp ltd" {
			tags.iter().filter(|x| x.name == "Payday").cloned().collect::<Vec<Tag>>().first().unwrap().clone()
		} else {
			let filtered_tags: Vec<&Tag> = tags.iter().filter(|x| x.name != "Payday").collect();
			(*filtered_tags.get(rng.gen_range(0..filtered_tags.len())).unwrap()).clone()
		};

		let amount: i32 = if recipient.name == *"Big corp ltd" {
			rng.gen_range(2000*currency.minor_in_major as i32..5000*currency.minor_in_major as i32)
		} else if i % 25 == 0 {
			rng.gen_range(-2000*currency.minor_in_major as i32..1000*currency.minor_in_major as i32)
		} else {
			rng.gen_range(-200*currency.minor_in_major as i32..10*currency.minor_in_major as i32)
		};

		let asset: Option<Asset> = if rng.gen_range(0..100) == 50 {
			Some(assets.get(rng.gen_range(0..assets.len())).unwrap().clone())
		} else {
			None
		};

		return Transaction {
			user_id,
			currency_id: Some(currency.id),
			recipient_id: recipient.id,
			account_id: account.id,
			tag_ids: vec![tag.id],
			asset,
			status: TransactionStatus::Completed,
			timestamp: Utc::now() - chrono::Duration::minutes(rng.gen_range(0..60*24*365)),
			positions: vec![Position {amount: Money::from_amount(amount, currency.minor_in_major, currency.symbol.clone()), ..Default::default()}],
			..Default::default()};
	}).collect();
}