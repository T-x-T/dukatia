use std::error::Error;

use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use deadpool_postgres::Pool;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use crate::webserver::{AppState, is_authorized};
use crate::transaction::{Transaction, Position};
use crate::currency::CurrencyLoader;
use crate::traits::*;

#[get("/api/v1/assets/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::AssetLoader::new(&data.pool).get().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[get("/api/v1/assets/all/deep")]
async fn get_all_deep(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::DeepAssetLoader::new(&data.pool).get().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[get("/api/v1/assets/{asset_id}")]
async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, asset_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::AssetLoader::new(&data.pool)
		.set_filter_id(*asset_id)
		.get_first().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => {
			if e.to_string().starts_with("no item of type unknown found") {
				return HttpResponse::NotFound().body(format!("{{\"error\":\"specified item of type asset not found with filter id={asset_id}\"}}"));
			}
			
			return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"));
		}
	}
}

#[derive(Deserialize, Clone, Debug)]
struct AssetPost {
	name: String,
	description: Option<String>,
	currency_id: u32,
	tag_ids: Option<Vec<u32>>,
}

#[post("/api/v1/assets")]
async fn post(data: web::Data<AppState>, req: HttpRequest, body: web::Json<AssetPost>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Asset::default()
		.set_name(body.name.clone())
		.set_description_opt(body.description.clone())
		.set_currency_id(body.currency_id)
		.set_tag_ids_opt(body.tag_ids.clone())
		.set_user_id(user_id)
		.save(&data.pool).await;

	match result {
		Ok(id) => return HttpResponse::Ok().body(format!("{{\"id\":\"{id}\"}}")),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[put("/api/v1/assets/{asset_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<AssetPost>, asset_id: web::Path<u32>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Asset::default()
		.set_id(*asset_id)
		.set_name(body.name.clone())
		.set_description_opt(body.description.clone())
		.set_currency_id(body.currency_id)
		.set_tag_ids_opt(body.tag_ids.clone())
		.set_user_id(user_id)
		.save(&data.pool).await;

	match result {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[delete("/api/v1/assets/{asset_id}")]
async fn delete_by_id(data: web::Data<AppState>, req: HttpRequest, asset_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Asset::default()
		.set_id(*asset_id)
		.delete(&data.pool).await;

	return match result {
		Ok(_) => HttpResponse::Ok().body(""),
		Err(e) => HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	};
}






#[get("/api/v1/assets/{asset_id}/valuation_history")]
async fn get_valuation_history_by_asset_id(data: web::Data<AppState>, req: HttpRequest, asset_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::AssetValuationLoader::new(&data.pool).set_filter_asset_id(*asset_id).get().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
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

#[post("/api/v1/assets/{asset_id}/valuation_history")]
async fn replace_valuation_history_of_asset(data: web::Data<AppState>, req: HttpRequest, asset_id: web::Path<u32>, body: web::Json<Vec<AssetValuationPost>>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let mut asset_valuations: Vec<super::AssetValuation> = Vec::new();
	for x in body.clone() {
		if x.amount.is_none() {
			return HttpResponse::BadRequest().body("{{\"error\":\"field amount needs to be set\"}}".to_string());
		}
		asset_valuations.push(super::AssetValuation {
			timestamp: x.timestamp,
			amount: x.amount.unwrap(),
			value_per_unit: x.value_per_unit,
			asset_id: *asset_id,
		});
	}	

	let result = super::Asset::default()
		.set_id(*asset_id)
		.replace_valuation_history(&data.pool, asset_valuations).await;

	match result {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[post("/api/v1/assets/{asset_id}/valuations")]
async fn post_valuation(data: web::Data<AppState>, req: HttpRequest, body: web::Json<AssetValuationPost>, asset_id: web::Path<u32>) -> impl Responder {
	let asset_id = asset_id.into_inner();
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	if body.amount.is_none() && body.amount_change.is_none() {
		return HttpResponse::BadRequest().body("{{\"error\":\"field amount or amount_change needs to be set\"}}".to_string());
	}

	let mut asset_valuation = body;

	if asset_valuation.amount_change.is_some() {
		let valuation_history = super::AssetValuationLoader::new(&data.pool).set_filter_asset_id(asset_id).get().await.expect("couldnt get amount history");
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
		Err(e) => HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	};
}

async fn add_valuation(pool: &Pool, body: &web::Json<AssetValuationPost>, asset_id: u32, user_id: u32) -> Result<(), Box<dyn Error>> {
	let asset = super::AssetLoader::new(pool)
		.set_filter_id(asset_id)
		.get_first().await?;

	super::AssetValuation {
		value_per_unit: body.value_per_unit,
		amount: body.amount.unwrap(),
		timestamp: body.timestamp,
		asset_id,
	}.save(pool).await?;

	if body.account_id.is_none() {
		return Ok(());
	}
	
	let last_amount: f64 = asset.amount.unwrap_or(0.0);
	let amount_difference = body.amount.unwrap() - last_amount;
	let currency = CurrencyLoader::new(pool).set_filter_id(asset.currency_id).get_first().await?;
	let formatted_value_per_unit = format!("{}{}", f64::from(body.value_per_unit) / f64::from(currency.minor_in_mayor), currency.symbol);

	let mut comment: String = if amount_difference < 0.0 {
		format!("Sold {} units at {} each", amount_difference * -1.0, formatted_value_per_unit)
	} else {
		format!("Bought {amount_difference} units at {formatted_value_per_unit} each")
	};

	if body.cost.is_some() {
		let formatted_cost = format!("{}{}", f64::from(body.cost.unwrap()) / f64::from(currency.minor_in_mayor), currency.symbol);
		comment = format!("{comment} with additional cost of {formatted_cost}");
	}

	let amount = if body.total_value.is_some() {
		body.total_value.unwrap()
	} else {
		-((f64::from(body.value_per_unit) * amount_difference) as i32) - body.cost.unwrap_or(0) as i32
	};

	Transaction::default()
		.set_account_id(body.account_id.unwrap())
		.set_timestamp(body.timestamp)
		.set_comment(comment)
		.set_user_id(user_id)
		.set_asset(asset)	
		.set_positions(vec![Position {
			amount,
			..Default::default()
		}])
		.save(pool).await?;

	return Ok(());
}