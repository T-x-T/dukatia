use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use super::Period;
use crate::money::Money;
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
	filter_amount: Option<u32>,
	filter_mode_amount: Option<String>,
	filter_rollover: Option<bool>,
	filter_mode_rollover: Option<String>,
	filter_filter_tag_id: Option<Uuid>,
	filter_mode_filter_tag_id: Option<String>,
	filter_lower_active_from: Option<DateTime<Utc>>,
	filter_upper_active_from: Option<DateTime<Utc>>,
	filter_mode_active_from: Option<String>,
	filter_lower_active_to: Option<DateTime<Utc>>,
	filter_upper_active_to: Option<DateTime<Utc>>,
	filter_mode_active_to: Option<String>,
	filter_currency_id: Option<u32>,
	filter_mode_currency_id: Option<String>,
	at_timestamp: Option<DateTime<Utc>>,
}

//TODO: test filters for properties other than id
#[get("/api/v1/budgets/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest, request_parameters: web::Query<RequestParameters>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let filters = Filters {
		id_uuid: request_parameters.filter_id.map(|x| {
			(x, request_parameters.filter_mode_id.clone().unwrap_or_default().into())
		}),
		name: request_parameters.filter_name.clone().map(|x| {
			(x, request_parameters.filter_mode_name.clone().unwrap_or_default().into())
		}),
 		int_amount: request_parameters.filter_amount.map(|x| {
			(x as i32, request_parameters.filter_mode_amount.clone().unwrap_or_default().into())
		}),
		rollover: request_parameters.filter_rollover.map(|x| {
			(x, request_parameters.filter_mode_rollover.clone().unwrap_or_default().into())
		}),
		tag_id: request_parameters.filter_filter_tag_id.map(|x| {
			(x, request_parameters.filter_mode_filter_tag_id.clone().unwrap_or_default().into())
		}),
		active_from: request_parameters.filter_lower_active_from.and_then(|x| {
			request_parameters.filter_upper_active_from.map(|y| {
				(x, y, request_parameters.filter_mode_active_from.clone().unwrap_or_default().into())
			})
		}),
		active_to: request_parameters.filter_lower_active_to.and_then(|x| {
			request_parameters.filter_upper_active_to.map(|y| {
				(x, y, request_parameters.filter_mode_active_to.clone().unwrap_or_default().into())
			})
		}),
		currency_id: request_parameters.filter_currency_id.map(|x| {
			(x, request_parameters.filter_mode_currency_id.clone().unwrap_or_default().into())
		}),
		user_id: Some((user_id, NumberFilterModes::ExactOrAlsoNull)),
		..Default::default()
	};

	let result = super::BudgetLoader::new(&data.pool)
	.set_query_parameters(
		QueryParameters::default()
			.set_max_results_opt(request_parameters.max_results)
			.set_skip_results_opt(request_parameters.skip_results)
			.set_filters(filters)
	)
	.get_full_at(request_parameters.at_timestamp.unwrap_or(Utc::now())).await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[get("/api/v1/budgets/{budget_id}")]
async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, budget_id: web::Path<Uuid>, request_parameters: web::Query<RequestParameters>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::BudgetLoader::new(&data.pool)
		.set_filter_id_uuid(*budget_id, NumberFilterModes::Exact)
		.set_filter_user_id(user_id, NumberFilterModes::Exact)
		.get_first_full_at(request_parameters.at_timestamp.unwrap_or(Utc::now())).await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => {
			if e.to_string().starts_with("no item of type unknown found") {
				return HttpResponse::NotFound().body(format!("{{\"error\":\"specified item of type budget not found with filter id={budget_id}\"}}"));
			}
			
			return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"));
		}
	}
}

#[get("/api/v1/budgets/{budget_id}/transactions")]
async fn get_transactions(data: web::Data<AppState>, req: HttpRequest, budget_id: web::Path<Uuid>, request_parameters: web::Query<RequestParameters>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let budget = super::BudgetLoader::new(&data.pool)
		.set_filter_id_uuid(*budget_id, NumberFilterModes::Exact)
		.set_filter_user_id(user_id, NumberFilterModes::Exact)
		.get_first()
		.await;

	match budget {
		Ok(budget) => {
			let transactions = budget.get_transactions_of_period_at(&data.pool, request_parameters.at_timestamp.unwrap_or(Utc::now())).await;

			match transactions {
				Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
				Err(e) => {
					if e.to_string().starts_with("no item of type unknown found") {
						return HttpResponse::NotFound().body(format!("{{\"error\":\"specified item of type budget not found with filter id={budget_id}\"}}"));
					}
					
					return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"));
				}
			}
		},
		Err(e) => {
			if e.to_string().starts_with("no item of type unknown found") {
				return HttpResponse::NotFound().body(format!("{{\"error\":\"specified item of type budget not found with filter id={budget_id}\"}}"));
			}
			
			return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"));
		}
	}
}

#[derive(Debug, Deserialize)]
struct BudgetPost {
	name: String,
	rollover: bool,
	period: u8,
	amount: Money,
	filter_tag_ids: Vec<Uuid>,
	currency_id: u32,
	active_from: DateTime<Utc>,
	active_to: Option<DateTime<Utc>>,
}

#[post("/api/v1/budgets")]
async fn post(data: web::Data<AppState>, req: HttpRequest, body: web::Json<BudgetPost>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Budget::default()
		.set_name(body.name.clone())
		.set_user_id(user_id)
		.set_rollover(body.rollover)
		.set_period(match body.period {
			0 => Period::Daily,
			1 => Period::Weekly,
			2 => Period::Monthly,
			3 => Period::Quarterly,
			4 => Period::Yearly,
			_ => panic!("unknown period"),
		})
		.set_amount(body.amount.clone())
		.set_filter_tag_ids(body.filter_tag_ids.clone())
		.set_currency_id(body.currency_id)
		.set_active_from(body.active_from)
		.set_active_to_opt(body.active_to)
		.create(&data.pool).await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(format!("{{\"id\":\"{res}\"}}")),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[put("/api/v1/budgets/{budget_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<BudgetPost>, budget_id: web::Path<Uuid>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Budget::default()
		.set_id(*budget_id)
		.set_name(body.name.clone())
		.set_user_id(user_id)
		.set_rollover(body.rollover)
		.set_period(match body.period {
			0 => Period::Daily,
			1 => Period::Weekly,
			2 => Period::Monthly,
			3 => Period::Quarterly,
			4 => Period::Yearly,
			_ => panic!("unknown period"),
		})
		.set_amount(body.amount.clone())
		.set_filter_tag_ids(body.filter_tag_ids.clone())
		.set_currency_id(body.currency_id)
		.set_active_from(body.active_from)
		.set_active_to_opt(body.active_to)
		.update(&data.pool).await;

	match result {
		Ok(()) => return HttpResponse::Ok().body(""),
		Err(e) => {
			if e.to_string().starts_with("you can only access items you own") {
				return HttpResponse::NotFound().body("");
			}

			if e.to_string().starts_with("no item of type unknown found") {
				return HttpResponse::NotFound().body(format!("{{\"error\":\"specified item of type budget not found with filter id={budget_id}\"}}"));
			}

			return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"))
		},
	}
}

#[delete("/api/v1/budgets/{budget_id}")]
async fn delete(data: web::Data<AppState>, req: HttpRequest, budget_id: web::Path<Uuid>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Budget::default()
		.set_id(*budget_id)
		.set_user_id(user_id)
		.delete(&data.pool).await;

	match result {
		Ok(()) => return HttpResponse::Ok().body(""),
		Err(e) => {
			if e.to_string().starts_with("you can only access items you own") {
				return HttpResponse::NotFound().body("");
			}

			if e.to_string().starts_with("no item of type unknown found") {
				return HttpResponse::NotFound().body(format!("{{\"error\":\"specified item of type budget not found with filter id={budget_id}\"}}"));
			}

			return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"))
		},
	}
}