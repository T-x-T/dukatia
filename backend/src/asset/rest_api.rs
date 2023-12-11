use std::error::Error;

use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use deadpool_postgres::Pool;
use serde::Deserialize;
use chrono::{DateTime, Utc};
use crate::webserver::{AppState, is_authorized};
use crate::transaction::{Transaction, Position};
use crate::money::Money;
use crate::traits::*;

#[derive(Debug, Deserialize)]
struct RequestParameters {
	skip_results: Option<u32>,
	max_results: Option<u32>,
	sort_property: Option<String>,
	sort_direction: Option<String>,
	filter_id: Option<u32>,
	filter_mode_id: Option<String>,
	filter_name: Option<String>,
	filter_mode_name: Option<String>,
	filter_description: Option<String>,
	filter_mode_description: Option<String>,
	filter_amount: Option<f64>,
	filter_mode_amount: Option<String>,
	filter_value_per_unit: Option<u32>,
	filter_mode_value_per_unit: Option<String>,
	filter_tag_id: Option<u32>,
	filter_mode_tag_id: Option<String>,
	timestamp: Option<DateTime<Utc>>,
}

//TODO: test filters and sorting for properties other than id
#[get("/api/v1/assets/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest, request_parameters: web::Query<RequestParameters>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let sort_property: Option<FilterAndSortProperties> = match &request_parameters.sort_property {
		Some(x) => {
			match x.as_str() {
				"id" => Some(FilterAndSortProperties::Id),
				"name" => Some(FilterAndSortProperties::Name),
				"description" => Some(FilterAndSortProperties::Description),
				"amount" => Some(FilterAndSortProperties::FloatAmount),
				"value per unit" => Some(FilterAndSortProperties::ValuePerUnit),
				_ => return HttpResponse::BadRequest().body(format!("{{\"error\":\"sort_property is invalid: {x}\"}}")),
			}
		},
		None => None,
	};

	let sort_direction: Option<SortDirection> = match &request_parameters.sort_direction {
		Some(x) => {
			match x.as_str() {
				"asc" | "ASC" => Some(SortDirection::Asc),
				"desc" | "DESC" => Some(SortDirection::Desc),
				_ => return HttpResponse::BadRequest().body(format!("{{\"error\":\"sort_direction is invalid: {x}\"}}")),
			}
		},
		None => None,
	};

	let filters = Filters {
		id: request_parameters.filter_id.map(|x| {
			(x, request_parameters.filter_mode_id.clone().unwrap_or_default().into())
		}),
		name: request_parameters.filter_name.clone().map(|x| {
			(x, request_parameters.filter_mode_name.clone().unwrap_or_default().into())
		}),
		description: request_parameters.filter_description.clone().map(|x| {
			(x, request_parameters.filter_mode_description.clone().unwrap_or_default().into())
		}),
		float_amount: request_parameters.filter_amount.map(|x| {
			(x, request_parameters.filter_mode_amount.clone().unwrap_or_default().into())
		}),
		value_per_unit: request_parameters.filter_value_per_unit.map(|x| {
			(x, request_parameters.filter_mode_value_per_unit.clone().unwrap_or_default().into())
		}),
		tag_id: request_parameters.filter_tag_id.map(|x| {
			(x, request_parameters.filter_mode_tag_id.clone().unwrap_or_default().into())
		}),
		..Default::default()
	};

	let result = super::AssetLoader::new(&data.pool)
	.set_query_parameters(
		QueryParameters::default()
			.set_max_results_opt(request_parameters.max_results)
			.set_skip_results_opt(request_parameters.skip_results)
			.set_sort_property_opt(sort_property)
			.set_sort_direction_opt(sort_direction)
			.set_filters(filters)
	)
	.get_at(request_parameters.timestamp.unwrap_or(Utc::now())).await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}


#[get("/api/v1/assets/{asset_id}")]
async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, asset_id: web::Path<u32>, request_parameters: web::Query<RequestParameters>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::AssetLoader::new(&data.pool)
		.set_filter_id(*asset_id, NumberFilterModes::Exact)
		.get_first_at(request_parameters.timestamp.unwrap_or(Utc::now())).await;

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
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
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
		Ok(id) => return HttpResponse::Ok().body(format!("{{\"id\":{id}}}")),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[put("/api/v1/assets/{asset_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<AssetPost>, asset_id: web::Path<u32>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
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
	let _user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Asset::default()
		.set_id(*asset_id)
		.delete(&data.pool).await;

	return match result {
		Ok(()) => HttpResponse::Ok().body(""),
		Err(e) => HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	};
}






#[get("/api/v1/assets/{asset_id}/valuation_history")]
async fn get_valuation_history_by_asset_id(data: web::Data<AppState>, req: HttpRequest, asset_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::AssetValuationLoader::new(&data.pool).set_filter_asset_id(*asset_id, NumberFilterModes::Exact).get().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}


#[derive(Deserialize, Clone, Debug)]
struct AssetValuationPost {
	value_per_unit: Money,
	amount: Option<f64>,
	amount_change: Option<f64>,
	timestamp: DateTime<Utc>,
	cost: Option<Money>,
	total_value: Option<Money>,
	account_id: Option<u32>,
}

#[post("/api/v1/assets/{asset_id}/valuation_history")]
async fn replace_valuation_history_of_asset(data: web::Data<AppState>, req: HttpRequest, asset_id: web::Path<u32>, body: web::Json<Vec<AssetValuationPost>>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
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
		Ok(()) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[post("/api/v1/assets/{asset_id}/valuations")]
async fn post_valuation(data: web::Data<AppState>, req: HttpRequest, body: web::Json<AssetValuationPost>, asset_id: web::Path<u32>) -> impl Responder {
	let asset_id = asset_id.into_inner();
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	if body.amount.is_none() && body.amount_change.is_none() {
		return HttpResponse::BadRequest().body("{{\"error\":\"field amount or amount_change needs to be set\"}}".to_string());
	}

	let mut asset_valuation = body;

	if asset_valuation.amount_change.is_some() {
		let valuation_history = super::AssetValuationLoader::new(&data.pool).set_filter_asset_id(asset_id, NumberFilterModes::Exact).get().await.expect("couldnt get amount history");
		let mut last_asset_valuation_amount: f64 = 0.0;
		for x in valuation_history {
			if x.timestamp.signed_duration_since(asset_valuation.timestamp).num_seconds() < 0 {
				last_asset_valuation_amount = x.amount;
			}
		}
		asset_valuation.amount = Some(last_asset_valuation_amount + asset_valuation.amount_change.unwrap());
	}

	return match add_valuation(&data.pool, &asset_valuation, asset_id, user_id).await {
		Ok(()) => HttpResponse::Ok().body(""),
		Err(e) => HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	};
}

async fn add_valuation(pool: &Pool, body: &web::Json<AssetValuationPost>, asset_id: u32, user_id: u32) -> Result<(), Box<dyn Error>> {
	let asset = super::AssetLoader::new(pool)
		.set_filter_id(asset_id, NumberFilterModes::Exact)
		.get_first().await?;

	super::AssetValuation {
		value_per_unit: body.value_per_unit.clone(),
		amount: body.amount.unwrap(),
		timestamp: body.timestamp,
		asset_id,
	}.save(pool).await?;

	if body.account_id.is_none() {
		return Ok(());
	}
	
	let last_amount: f64 = asset.amount.unwrap_or(0.0);
	let amount_difference = body.amount.unwrap() - last_amount;
	let formatted_value_per_unit = body.value_per_unit.to_string();

	let mut comment: String = if amount_difference < 0.0 {
		format!("Sold {} units at {} each", amount_difference * -1.0, formatted_value_per_unit)
	} else {
		format!("Bought {amount_difference} units at {formatted_value_per_unit} each")
	};

	if body.cost.is_some() {
		let formatted_cost = body.cost.clone().unwrap().to_string();
		comment = format!("{comment} with additional cost of {formatted_cost}");
	}

	let amount: Money = if body.total_value.is_some() {
		body.total_value.clone().unwrap()
	} else {
		Money::from_amount(-((f64::from(body.value_per_unit.to_amount()) * amount_difference) as i32) - body.cost.clone().unwrap_or_default().to_amount(), body.value_per_unit.get_minor_in_major(), body.value_per_unit.get_symbol())
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