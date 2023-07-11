use actix_web::{get, post, put, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use crate::webserver::{AppState, is_authorized};
use crate::traits::*;

#[get("/api/v1/currencies/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::CurrencyLoader::new(&data.pool).get().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[get("/api/v1/currencies/{currency_id}")]
async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, currency_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::CurrencyLoader::new(&data.pool)
		.set_filter_id(*currency_id)
		.get_first().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => {
			if e.to_string().starts_with("no item of type unknown found") {
				return HttpResponse::NotFound().body(format!("{{\"error\":\"specified item of type currency not found with filter id={currency_id}\"}}"));
			}
			
			return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"));
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
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Currency::default()
		.set_name(body.name.clone())
		.set_minor_in_mayor(body.minor_in_mayor)
		.set_symbol(body.symbol.clone())
		.save(&data.pool).await;

	match result {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[put("/api/v1/currencies/{currency_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<CurrencyPost>, currency_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Currency::default()
		.set_id(*currency_id)
		.set_name(body.name.clone())
		.set_minor_in_mayor(body.minor_in_mayor)
		.set_symbol(body.symbol.clone())
		.save(&data.pool).await;

	match result {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}