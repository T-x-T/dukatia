use actix_web::{get, post, put, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use uuid::Uuid;
use crate::webserver::{AppState, is_authorized};
use crate::traits::*;

#[derive(Debug, Deserialize)]
struct RequestParameters {
	skip_results: Option<u32>,
	max_results: Option<u32>,
	filter_id: Option<Uuid>,
	filter_mode_id: Option<String>,
	filter_name: Option<String>,
	filter_mode_name: Option<String>,
	filter_symbol: Option<String>,
	filter_mode_symbol: Option<String>,
	filter_minor_in_major: Option<u32>,
	filter_mode_minor_in_major: Option<String>,
}

//TODO: test filters for properties other than id
//Docs: /dev/rest_api/currencies#get-all
#[get("/api/v1/currencies/all")]
pub async fn get_all(data: web::Data<AppState>, req: HttpRequest, request_parameters: web::Query<RequestParameters>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let filters = Filters {
		id: request_parameters.filter_id.map(|x| {
			(x, request_parameters.filter_mode_id.clone().unwrap_or_default().into())
		}),
		name: request_parameters.filter_name.clone().map(|x| {
			(x, request_parameters.filter_mode_name.clone().unwrap_or_default().into())
		}),
		symbol: request_parameters.filter_symbol.clone().map(|x| {
			(x, request_parameters.filter_mode_symbol.clone().unwrap_or_default().into())
		}),
		minor_in_major: request_parameters.filter_minor_in_major.map(|x| {
			(x, request_parameters.filter_mode_minor_in_major.clone().unwrap_or_default().into())
		}),
		..Default::default()
	};

	let result = super::CurrencyLoader::new(&data.pool)
	.set_query_parameters(
		QueryParameters::default()
			.set_sort_property_opt(Some(FilterAndSortProperties::Name))
			.set_sort_direction_opt(Some(SortDirection::Asc))
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

//Docs: /dev/rest_api/currencies#get-by-id
#[get("/api/v1/currencies/{currency_id}")]
pub async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, currency_id: web::Path<Uuid>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
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

//Docs: /dev/rest_api/currencies#create-currency
#[post("/api/v1/currencies")]
pub async fn post(data: web::Data<AppState>, req: HttpRequest, body: web::Json<CurrencyPost>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match crate::user::UserLoader::new(&data.pool).set_filter_id(user_id, NumberFilterModes::Exact).get_first().await {
    Ok(user) => {
			if !user.superuser {
				return HttpResponse::BadRequest().body("{\"error\":\"youre not allowed to create new currencies\"}");
			}

			let result = super::Currency::default()
				.set_name(body.name.clone())
				.set_minor_in_major(body.minor_in_major)
				.set_symbol(body.symbol.clone())
				.create(&data.pool).await;
		
			match result {
				Ok(id) => return HttpResponse::Ok().body(format!("{{\"id\":\"{id}\"}}")),
				Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
			}
		},
    Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}	
}

//Docs: /dev/rest_api/currencies#modify-currency
#[put("/api/v1/currencies/{currency_id}")]
pub async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<CurrencyPost>, currency_id: web::Path<Uuid>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match crate::user::UserLoader::new(&data.pool).set_filter_id(user_id, NumberFilterModes::Exact).get_first().await {
    Ok(user) => {
			if !user.superuser {
				return HttpResponse::BadRequest().body("{\"error\":\"youre not allowed to update existing currencies\"}");
			}

			let result = super::Currency::default()
				.set_id(*currency_id)
				.set_name(body.name.clone())
				.set_minor_in_major(body.minor_in_major)
				.set_symbol(body.symbol.clone())
				.update(&data.pool).await;
	
			match result {
				Ok(()) => return HttpResponse::Ok().body(""),
				Err(e) => {
					if e.to_string().starts_with("no item of type unknown found") {
						return HttpResponse::NotFound().body(format!("{{\"error\":\"specified item of type currency not found with filter id={currency_id}\"}}"));
					}
					
					return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"));
				}
			}
		},
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}