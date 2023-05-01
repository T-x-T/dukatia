use std::error::Error;
use deadpool_postgres::Pool;

use super::{Chart, ChartData};

use crate::CustomError;
use crate::currency;
use crate::transaction;

pub async fn get_chart_data(pool: &Pool, chart: Chart) -> Result<ChartData, Box<dyn Error>> {
	if chart.text_template.is_none() {
		return Err(Box::new(CustomError::MissingProperty { property: String::from("text_template"), item_type: String::from("chart") }));
	}

	let mut output = String::new();

	let input_parts = chart.text_template.as_ref().unwrap().split('%');

	let mut counter = 0;
	for part in input_parts {
		if counter % 2 == 0 {
			output.push_str(part);
			counter += 1;
		} else {
			output.push_str(compute_function(pool, part).await?.as_str());
			counter += 1;
		}
	}

	return Ok(ChartData { text: Some(output), pie: None });
}

async fn compute_function(pool: &Pool, function: &str) -> Result<String, Box<dyn Error>> {
	let function_name = function.split("{").next();
	let function_body = function.split("{").skip(1).next();
	
	if function_name.is_none() {
		return Err(Box::new(CustomError::InvalidItem { reason: format!("function {function} doesnt contain a function") }));
	}
	if function_body.is_none() {
		return Err(Box::new(CustomError::InvalidItem { reason: format!("function {function} doesnt contain a body") }));
	}
	
	let function_name = function_name.unwrap();
	
	let mut body_chars = function_body.unwrap().chars();
	body_chars.next_back();
	let function_body = body_chars.as_str();
	
	match function_name {
		"foreach_currency" => return compute_function_foreach_currency(pool, function_body).await,
		_ => return Err(Box::new(CustomError::InvalidItem { reason: format!("function name {:?} is not recognized", function_name) })),
	}
}

async fn compute_function_foreach_currency(pool: &Pool, body: &str) -> Result<String, Box<dyn Error>> {
	let currencies = currency::get_all(pool).await?;
	let mut output = String::new();

	for currency in currencies.into_iter() {
		let mut in_token_name = false;
		let mut token_name = String::new();

		for char in body.chars() {
			if in_token_name {
				if char == '\\' || char == ' ' || char == ':' || char == '*' || char == '$' {
					
					output.push_str(compute_token_currency(&pool, token_name.as_str(), &currency).await?.as_str());
					token_name = String::new();
					in_token_name = char == '$';
					
					if char == '*' {
						output.push('\n');
						continue;
					}
					if !in_token_name {
						output.push(char);
					}
				} else {
					token_name.push(char);
				}
			} else {
				if char == '$' {
					in_token_name = true;
				} else {
					output.push(char);
				}
			}
		}
	}

	return Ok(output);
}

async fn compute_token_currency(pool: &Pool, token_name: &str, currency: &currency::Currency) -> Result<String, Box<dyn Error>> {
	return Ok(match token_name {
		"name" => currency.name.clone(),
		"symbol" => currency.symbol.clone(),
		"current_balance" => (current_balance_of_currency(
				&pool, currency.id.unwrap()
			).await? as f64 / currency.minor_in_mayor as f64).to_string(),
		_ => return Err(Box::new(CustomError::InvalidItem { reason: format!("token name {:?} is not recognized in function foreach_currency", token_name) })),
	});
}

async fn current_balance_of_currency(pool: &Pool, currency_id: u32) -> Result<i32, Box<dyn Error>> {
	let mut output: i32 = 0;
	let transactions = transaction::get_all(&pool).await?;

	transactions.iter().for_each(|transaction| {
		if transaction.currency_id.unwrap() == currency_id {
			output += transaction.amount;
		}
	});

	return Ok(output);
}