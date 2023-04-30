mod db;
pub mod rest_api;

use crate::CustomError;
use crate::currency;

use serde::Serialize;
use std::error::Error;
use deadpool_postgres::Pool;

#[derive(Debug, Clone, Serialize)]
pub struct Chart {
	pub id: Option<u32>,
	pub user_id: Option<u32>,
	pub grid_size: String,
	pub chart_type: String,
	pub title: String,
	pub text_template: Option<String>
}

#[derive(Debug, Clone, Serialize)]
pub struct ChartData {
	pub text: Option<String>,
}

pub async fn get_by_id(pool: &Pool, id: u32) -> Result<Chart, Box<dyn Error>> {
	return db::get_by_id(pool, id).await;
}

async fn get_chart_contents_by_id(pool: &Pool, chart_id: u32) -> Result<ChartData, Box<dyn Error>> {
	let chart = get_by_id(pool, chart_id).await.unwrap();

	if chart.chart_type == "text" {
		return get_text_chart_content(pool, chart).await;
	} else if chart.chart_type == "pie" {

	} else if chart.chart_type == "line" {
			
	}

	return Err(Box::new(CustomError::InvalidItem { reason: String::from("chart_type is not equal to text, pie or line") }));
}

async fn get_text_chart_content(pool: &Pool, chart: Chart) -> Result<ChartData, Box<dyn Error>> {
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

	return Ok(ChartData { text: Some(output) });
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
	
	println!("name: {} body: {}", function_name, function_body);
	
	//return Ok(function);
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
				if char == '\\' || char == ' ' || char == '*' || char == '$' {
					in_token_name = char == '$';
					
					match token_name.as_str() {
						"name" => output.push_str(currency.name.as_str()),
						"symbol" => output.push_str(currency.symbol.as_str()),
						_ => return Err(Box::new(CustomError::InvalidItem { reason: format!("token name {:?} is not recognized in function foreach_currency", token_name) })),
					}
					token_name = String::new();
					
					if !in_token_name {
						if char == '*' {
							output.push('\n');
						} else {
							output.push(char);
						}
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
