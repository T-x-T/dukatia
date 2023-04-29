use actix_web::{get, post, put, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use super::super::webserver::{AppState, is_authorized};

#[get("/api/v1/currencies/all")]
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

#[get("/api/v1/currencies/{currency_id}")]
async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, currency_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::get_by_id(&data.pool, currency_id.into_inner()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => {
			if e.to_string().starts_with("specified item of type currency not found with filter") {
				return HttpResponse::NotFound().body(format!("{{\"error\":\"{}\"}}", e));
			} else {
				return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e));
			}
		}
	}
}

#[derive(Deserialize)]
struct CurrencyPost {
	name: String,
	minor_in_mayor: u32,
	symbol: String,
}

#[post("/api/v1/currencies")]
async fn post(data: web::Data<AppState>, req: HttpRequest, body: web::Json<CurrencyPost>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	let currency = super::Currency {
		id: None,
		name: body.name.clone(),
		minor_in_mayor: body.minor_in_mayor,
		symbol: body.symbol.clone(),
	};

	match super::add(&data.pool, &currency).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[put("/api/v1/currencies/{currency_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<CurrencyPost>, currency_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	let currency = super::Currency {
		id: Some(currency_id.into_inner()),
		name: body.name.clone(),
		minor_in_mayor: body.minor_in_mayor,
		symbol: body.symbol.clone(),
	};

	match super::update(&data.pool, &currency).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}