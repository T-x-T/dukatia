use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use super::super::webserver::{AppState, is_authorized};

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
	tag_ids: Option<Vec<u32>>
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
	};

	match super::add(&data.pool, &asset).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
	}
}