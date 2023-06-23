use std::error::Error;

use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use deadpool_postgres::Pool;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use super::super::webserver::{AppState, is_authorized};
use crate::transaction;

#[derive(Deserialize, Clone, Debug)]
struct AssetPost {
	name: String,
	description: Option<String>,
	currency_id: u32,
	tag_ids: Option<Vec<u32>>,
}

#[derive(Deserialize, Clone, Debug)]
struct AssetValuationPost {
	value_per_unit: u32,
	amount: Option<f64>,
	amount_change: Option<f64>,
	timestamp: DateTime<Utc>,
	cost: Option<u32>,
	total_value: Option<i32>,
	account_id: Option<u32>,
}

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

#[get("/api/v1/assets/all/deep")]
async fn get_all_deep(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::get_all_deep(&data.pool).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[get("/api/v1/assets/{asset_id}")]
async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, asset_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::get_by_id(&data.pool, asset_id.into_inner()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => {
			if e.to_string().starts_with("specified item of type asset not found with filter") {
				return HttpResponse::NotFound().body(format!("{{\"error\":\"{}\"}}", e));
			} else {
				return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e));
			}
		}
	}
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
		value_per_unit: None,
		amount: None,
		tag_ids: body.tag_ids.clone(),
		user_id,
		total_cost_of_ownership: None,
	};

	match super::add(&data.pool, &asset).await {
		Ok(id) => return HttpResponse::Ok().body(format!("{{\"id\":\"{}\"}}", id)),
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
		value_per_unit: None,
		amount: None,
		tag_ids: body.tag_ids.clone(),
		user_id,
		total_cost_of_ownership: None,
	};

	match super::update(&data.pool, &asset).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
	}
}

#[get("/api/v1/assets/{asset_id}/valuation_history")]
async fn get_valuation_history_by_asset_id(data: web::Data<AppState>, req: HttpRequest, asset_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}",e))
	};

	match super::get_valuation_history_by_asset_id(&data.pool, asset_id.into_inner()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
	}
}

#[post("/api/v1/assets/{asset_id}/valuation_history")]
async fn replace_valuation_history_of_asset(data: web::Data<AppState>, req: HttpRequest, asset_id: web::Path<u32>, body: web::Json<Vec<AssetValuationPost>>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}",e))
	};

	let mut asset_valuations: Vec<super::AssetValuation> = Vec::new();
	for x in body.clone().into_iter() {
		if x.amount.is_none() {
			return HttpResponse::BadRequest().body(format!("{{\"error\":\"field amount needs to be set\"}}"));
		}
		asset_valuations.push(super::AssetValuation {
			timestamp: x.timestamp,
			amount: x.amount.unwrap(),
			value_per_unit: x.value_per_unit,
		});
	}	

	match super::replace_valuation_history_of_asset(&data.pool, asset_id.into_inner(), asset_valuations).await {
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

#[post("/api/v1/assets/{asset_id}/valuations")]
async fn post_valuation(data: web::Data<AppState>, req: HttpRequest, body: web::Json<AssetValuationPost>, asset_id: web::Path<u32>) -> impl Responder {
	let asset_id = asset_id.into_inner();
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}",e))
	};

	if body.amount.is_none() && body.amount_change.is_none() {
		return HttpResponse::BadRequest().body(format!("{{\"error\":\"field amount or amount_change needs to be set\"}}"));
	}

	let mut asset_valuation = body;

	if asset_valuation.amount_change.is_some() {
		let valuation_history = super::get_valuation_history_by_asset_id(&data.pool, asset_id).await.expect("couldnt get amount history");
		let mut last_asset_valuation_amount: f64 = 0.0;
		for x in valuation_history {
			if x.timestamp.signed_duration_since(asset_valuation.timestamp).num_seconds() < 0 {
				last_asset_valuation_amount = x.amount;
			}
		}
		asset_valuation.amount = Some(last_asset_valuation_amount + asset_valuation.amount_change.unwrap());
	}

	return match add_valuation(&data.pool, &asset_valuation, asset_id, user_id).await {
		Ok(_) => HttpResponse::Ok().body(""),
		Err(e) => HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
	};
}

async fn add_valuation(pool: &Pool, body: &web::Json<AssetValuationPost>, asset_id: u32, user_id: u32) -> Result<(), Box<dyn Error>> {
	let asset = super::get_by_id(&pool, asset_id).await?;

	let asset_valuation = super::AssetValuation {
		value_per_unit: body.value_per_unit,
		amount: body.amount.unwrap(),
		timestamp: body.timestamp,
	};
	super::add_valuation(&pool, asset_id, &asset_valuation).await?;

	if body.account_id.is_none() {
		return Ok(());
	}
	
	let last_amount: f64 = asset.amount.unwrap_or(0.0);
	let amount_difference = body.amount.unwrap() - last_amount;
	let currency = crate::currency::get_by_id(&pool, asset.currency_id).await.unwrap();
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

	let amount = if body.total_value.is_some() {
		body.total_value.unwrap()
	} else {
		((body.value_per_unit as f64 * amount_difference) as i32 * -1) - body.cost.unwrap_or(0) as i32
	};

	let transaction = transaction::Transaction {
		id: None,
		currency_id: None,
		account_id: body.account_id.unwrap(),
		recipient_id: 0,
		status: transaction::TransactionStatus::Completed,
		timestamp: body.timestamp,
		total_amount: None,
		comment: Some(comment),
		asset: Some(asset),
		user_id,
		tag_ids: None,
		positions: vec![transaction::Position {
			id: None,
			amount: amount,
			comment: None,
			tag_id: None,
		}],
	};
	transaction::add(&pool, &transaction).await?;

	return Ok(());
}