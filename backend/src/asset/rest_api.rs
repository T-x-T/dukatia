use std::error::Error;

use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use deadpool_postgres::Pool;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use super::super::webserver::{AppState, is_authorized};
use crate::transaction;

#[get("/api/v1/assets/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::get_all(&data.pool).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[derive(Deserialize, Clone)]
struct AssetPost {
	name: String,
	description: Option<String>,
	currency_id: u32,
	value_per_unit: u32,
	amount: f64,
	tag_ids: Option<Vec<u32>>,
	timestamp: DateTime<Utc>,
	account_id: Option<u32>,
	cost: Option<u32>,
}

#[post("/api/v1/assets")]
async fn post(data: web::Data<AppState>, req: HttpRequest, body: web::Json<AssetPost>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	let asset = super::Asset {
		id: None,
		name: body.name.clone(),
		description: body.description.clone(),
		currency_id: body.currency_id,
		value_per_unit: Some(body.value_per_unit),
		amount: Some(body.amount),
		tag_ids: body.tag_ids.clone(),
		user_id,
		timestamp: Some(body.timestamp),
	};

	let asset_result = super::add(&data.pool, &asset).await;

	if body.account_id.is_some() && asset_result.is_ok() {
		let mut asset = asset;
		asset.id = Some(asset_result.as_ref().unwrap().clone());

		match add_transaction(&data.pool, &body, asset.clone(), user_id, true).await {
			Ok(_) => (),
			Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
		}
	}

	match asset_result {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
	}
}

#[put("/api/v1/assets/{asset_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<AssetPost>, asset_id: web::Path<u32>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}",e))
	};

	let asset = super::Asset {
		id: Some(asset_id.clone()),
		name: body.name.clone(),
		description: body.description.clone(),
		currency_id: body.currency_id,
		value_per_unit: Some(body.value_per_unit),
		amount: Some(body.amount),
		tag_ids: body.tag_ids.clone(),
		user_id,
		timestamp: Some(body.timestamp),
	};

	if body.account_id.is_some() {
		match add_transaction(&data.pool, &body, asset.clone(), user_id, false).await {
			Ok(_) => (),
			Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
		}
	}

	match super::update(&data.pool, &asset).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
	}
}

#[delete("/api/v1/assets/{asset_id}")]
async fn delete_by_id(data: web::Data<AppState>, req: HttpRequest, asset_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}",e))
	};

	return match super::delete_by_id(&data.pool, asset_id.into_inner()).await {
		Ok(_) => HttpResponse::Ok().body(""),
		Err(e) => HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
	};
}

async fn add_transaction(pool: &Pool, body: &web::Json<AssetPost>, asset: super::Asset, user_id: u32, new: bool) -> Result<(), Box<dyn Error>> {
	let last_amount: f64 = if !new {
		super::get_by_id(&pool, asset.id.unwrap()).await.unwrap().amount.unwrap_or_default()
	} else {
		0.0
	};

	let amount_difference = body.amount - last_amount;
	let currency = crate::currency::get_by_id(&pool, body.currency_id).await.unwrap();
	let formatted_value_per_unit = format!("{}{}", body.value_per_unit as f64 / currency.minor_in_mayor as f64, currency.symbol);

	let mut comment: String = if amount_difference < 0.0 {
		format!("Sold {} units at {} each", amount_difference * -1.0, formatted_value_per_unit)
	} else {
		format!("Bought {} units at {} each", amount_difference, formatted_value_per_unit)
	};

	if body.cost.is_some() {
		let formatted_cost = format!("{}{}", body.cost.unwrap() as f64 / currency.minor_in_mayor as f64, currency.symbol);
		comment = format!("{} with additional cost of {}", comment, formatted_cost);
	}

	let transaction = transaction::Transaction {
		id: None,
		currency_id: None,
		account_id: body.account_id.unwrap(),
		recipient_id: 0,
		status: transaction::TransactionStatus::Completed,
		timestamp: body.timestamp,
		amount: ((body.value_per_unit as f64 * amount_difference) as i32 * -1) - body.cost.unwrap_or(0) as i32,
		comment: Some(comment),
		asset: Some(asset),
		user_id,
		tag_ids: None,
	};
	return transaction::add(&pool, &transaction).await;
}