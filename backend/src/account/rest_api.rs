use actix_web::{get, post, put, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use super::super::webserver::{AppState, is_authorized};
use crate::traits::*;

#[derive(Debug, Deserialize)]
struct RequestParameters {
	skip_results: Option<u32>,
	max_results: Option<u32>,
	filter_id: Option<u32>,
	filter_mode_id: Option<String>,
	filter_name: Option<String>,
	filter_mode_name: Option<String>,
	filter_currency_id: Option<u32>,
	filter_mode_currency_id: Option<String>,
	filter_tag_id: Option<u32>,
	filter_mode_tag_id: Option<String>,
	filter_balance: Option<i64>,
	filter_mode_balance: Option<String>,
}

//TODO: test max results, skip results, filters and sorting
#[get("/api/v1/accounts/all")]
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
		default_currency_id: request_parameters.filter_currency_id.map(|x| {
			(x, request_parameters.filter_mode_currency_id.clone().unwrap_or(String::new()).into())
		}),
		tag_id: request_parameters.filter_tag_id.map(|x| {
			(x, request_parameters.filter_mode_tag_id.clone().unwrap_or(String::new()).into())
		}),
		balance: request_parameters.filter_balance.map(|x| {
			(x, request_parameters.filter_mode_balance.clone().unwrap_or(String::new()).into())
		}),
		..Default::default()
	};

	let result = super::AccountLoader::new(&data.pool)
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

#[get("/api/v1/accounts/{account_id}")]
async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, account_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::AccountLoader::new(&data.pool).set_filter_id(*account_id, NumberFilterModes::Exact).get_first().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => {
			if e.to_string().starts_with("no item of type unknown found") {
				return HttpResponse::NotFound().body(format!("{{\"error\":\"specified item of type account not found with filter id={account_id}\"}}"));
			}
			
			return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"));
		}
	}
}

#[derive(Deserialize)]
struct AccountPost {
	name: String,
	default_currency_id: u32,
	tag_ids: Option<Vec<u32>>,
}

#[post("/api/v1/accounts")]
async fn post(data: web::Data<AppState>, req: HttpRequest, body: web::Json<AccountPost>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Account::default()
		.set_user_id(user_id)
		.set_name(body.name.clone())
		.set_default_currency_id(body.default_currency_id)
		.set_tag_ids_opt(body.tag_ids.clone())
		.save(&data.pool).await;

	match result {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[put("/api/v1/accounts/{account_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<AccountPost>, account_id: web::Path<u32>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Account::default()
		.set_id(*account_id)
		.set_user_id(user_id)
		.set_name(body.name.clone())
		.set_default_currency_id(body.default_currency_id)
		.set_tag_ids_opt(body.tag_ids.clone())
		.save(&data.pool).await;

	match result {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}