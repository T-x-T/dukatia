use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use chrono::{DateTime, Utc};
use super::super::webserver::{AppState, is_authorized};

#[get("/api/v1/assets/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => {println!("{:?}", e); return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))}
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
	timestamp: DateTime<Utc>
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

	match super::add(&data.pool, &asset).await {
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
		id: Some(asset_id.into_inner()),
		name: body.name.clone(),
		description: body.description.clone(),
		currency_id: body.currency_id,
		value_per_unit: Some(body.value_per_unit),
		amount: Some(body.amount),
		tag_ids: body.tag_ids.clone(),
		user_id,
		timestamp: Some(body.timestamp),
	};

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