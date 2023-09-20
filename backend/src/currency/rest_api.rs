use actix_web::{get, post, put, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use crate::webserver::{AppState, is_authorized};
use crate::traits::*;

#[derive(Debug, Deserialize)]
struct RequestParameters {
	skip_results: Option<u32>,
	max_results: Option<u32>,
	filter_id: Option<u32>,
	filter_mode_id: Option<String>,
	filter_name: Option<String>,
	filter_mode_name: Option<String>,
	filter_symbol: Option<String>,
	filter_mode_symbol: Option<String>,
	filter_minor_in_major: Option<u32>,
	filter_mode_minor_in_major: Option<String>,
}

//TODO: test filters for properties other than id
#[get("/api/v1/currencies/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest, request_parameters: web::Query<RequestParameters>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let filters = Filters {
		id: request_parameters.filter_id.map(|x| {
			(x, request_parameters.filter_mode_id.clone().unwrap_or(String::new()).into())
		}),
		name: request_parameters.filter_name.clone().map(|x| {
			(x, request_parameters.filter_mode_name.clone().unwrap_or(String::new()).into())
		}),
		symbol: request_parameters.filter_symbol.clone().map(|x| {
			(x, request_parameters.filter_mode_symbol.clone().unwrap_or(String::new()).into())
		}),
		minor_in_major: request_parameters.filter_minor_in_major.map(|x| {
			(x, request_parameters.filter_mode_minor_in_major.clone().unwrap_or(String::new()).into())
		}),
		..Default::default()
	};

	let result = super::CurrencyLoader::new(&data.pool)
	.set_query_parameters(
		QueryParameters::default()
			.set_max_results_opt(request_parameters.max_results)
			.set_skip_results_opt(request_parameters.skip_results)
			.set_filters(filters)
	)
	.get().await;

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
		.set_filter_id(*currency_id, NumberFilterModes::Exact)
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
	minor_in_major: u32,
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
		.set_minor_in_major(body.minor_in_major)
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
		.set_minor_in_major(body.minor_in_major)
		.set_symbol(body.symbol.clone())
		.save(&data.pool).await;

	match result {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}