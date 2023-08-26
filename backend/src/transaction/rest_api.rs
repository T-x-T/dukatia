use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use chrono::{DateTime, Utc};
use crate::webserver::{AppState, is_authorized};
use crate::asset::Asset;
use crate::traits::*;

#[derive(Debug, Deserialize)]
struct RequestParameters {
	skip_results: Option<u32>,
	max_results: Option<u32>,
	sort_property: Option<String>,
	sort_direction: Option<String>,
	filter_id: Option<u32>,
	filter_mode_id: Option<String>,
	filter_total_amount: Option<i32>,
	filter_mode_total_amount: Option<String>,
	filter_asset_id: Option<u32>,
	filter_mode_asset_id: Option<String>,
	filter_user_id: Option<u32>,
	filter_mode_user_id: Option<String>,
	filter_currency_id: Option<u32>,
	filter_mode_currency_id: Option<String>,
	filter_account_id: Option<u32>,
	filter_mode_account_id: Option<String>,
	filter_recipient_id: Option<u32>,
	filter_mode_recipient_id: Option<String>,
	filter_tag_id: Option<u32>,
	filter_mode_tag_id: Option<String>,
	filter_comment: Option<String>,
	filter_mode_comment: Option<String>,
	filter_time_range_lower: Option<DateTime<Utc>>,
	filter_time_range_upper: Option<DateTime<Utc>>,
	filter_mode_time_range: Option<String>,
}

//TODO: test filters and sorting for properties other than id
#[get("/api/v1/transactions/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest, request_parameters: web::Query<RequestParameters>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let sort_property: Option<FilterAndSortProperties> = match &request_parameters.sort_property {
		Some(x) => {
			match x.as_str() {
				"account" => Some(FilterAndSortProperties::AccountId),
				"comment" => Some(FilterAndSortProperties::Comment),
				"currency_id" => Some(FilterAndSortProperties::CurrencyId),
				"id" => Some(FilterAndSortProperties::Id),
				"recipient" => Some(FilterAndSortProperties::RecipientId),
				"status" => Some(FilterAndSortProperties::Status),
				"timestamp" => Some(FilterAndSortProperties::Timestamp),
				"user_id" => Some(FilterAndSortProperties::UserId),
				"amount" => Some(FilterAndSortProperties::TotalAmount),
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
			(x, request_parameters.filter_mode_id.clone().unwrap_or(String::new()).into())
		}),
		total_amount: request_parameters.filter_total_amount.map(|x| {
			(x, request_parameters.filter_mode_total_amount.clone().unwrap_or(String::new()).into())
		}),
		asset_id: request_parameters.filter_asset_id.map(|x| {
			(x, request_parameters.filter_mode_asset_id.clone().unwrap_or(String::new()).into())
		}),
		user_id: request_parameters.filter_user_id.map(|x| {
			(x, request_parameters.filter_mode_user_id.clone().unwrap_or(String::new()).into())
		}),
		currency_id: request_parameters.filter_currency_id.map(|x| {
			(x, request_parameters.filter_mode_currency_id.clone().unwrap_or(String::new()).into())
		}),
		account_id: request_parameters.filter_account_id.map(|x| {
			(x, request_parameters.filter_mode_account_id.clone().unwrap_or(String::new()).into())
		}),
		recipient_id: request_parameters.filter_recipient_id.map(|x| {
			(x, request_parameters.filter_mode_recipient_id.clone().unwrap_or(String::new()).into())
		}),
		tag_id: request_parameters.filter_tag_id.map(|x| {
			(x, request_parameters.filter_mode_tag_id.clone().unwrap_or(String::new()).into())
		}),
		comment: request_parameters.filter_comment.clone().map(|x| {
			(x, request_parameters.filter_mode_comment.clone().unwrap_or(String::new()).into())
		}),
		time_range: request_parameters.filter_time_range_lower.and_then(|x| {
			request_parameters.filter_time_range_upper.map(|y| {
				(x, y, request_parameters.filter_mode_time_range.clone().unwrap_or(String::new()).into())
			})
		}),
		..Default::default()
	};

	let result = super::TransactionLoader::new(&data.pool)
		.set_query_parameters(
			QueryParameters::default()
				.set_max_results_opt(request_parameters.max_results)
				.set_skip_results_opt(request_parameters.skip_results)
				.set_sort_property_opt(sort_property)
				.set_sort_direction_opt(sort_direction)
				.set_filters(filters)
		)
		.get().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

//TODO: test filters and sorting for properties other than id
#[get("/api/v1/transactions/all/deep")]
async fn get_all_deep(data: web::Data<AppState>, req: HttpRequest, request_parameters: web::Query<RequestParameters>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let sort_property: Option<FilterAndSortProperties> = match &request_parameters.sort_property {
		Some(x) => {
			match x.as_str() {
				"account" => Some(FilterAndSortProperties::AccountId),
				"comment" => Some(FilterAndSortProperties::Comment),
				"currency_id" => Some(FilterAndSortProperties::CurrencyId),
				"id" => Some(FilterAndSortProperties::Id),
				"recipient" => Some(FilterAndSortProperties::RecipientId),
				"status" => Some(FilterAndSortProperties::Status),
				"timestamp" => Some(FilterAndSortProperties::Timestamp),
				"user_id" => Some(FilterAndSortProperties::UserId),
				"amount" => Some(FilterAndSortProperties::TotalAmount),
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
			(x, request_parameters.filter_mode_id.clone().unwrap_or(String::new()).into())
		}),
		total_amount: request_parameters.filter_total_amount.map(|x| {
			(x, request_parameters.filter_mode_total_amount.clone().unwrap_or(String::new()).into())
		}),
		asset_id: request_parameters.filter_asset_id.map(|x| {
			(x, request_parameters.filter_mode_asset_id.clone().unwrap_or(String::new()).into())
		}),
		user_id: request_parameters.filter_user_id.map(|x| {
			(x, request_parameters.filter_mode_user_id.clone().unwrap_or(String::new()).into())
		}),
		currency_id: request_parameters.filter_currency_id.map(|x| {
			(x, request_parameters.filter_mode_currency_id.clone().unwrap_or(String::new()).into())
		}),
		account_id: request_parameters.filter_account_id.map(|x| {
			(x, request_parameters.filter_mode_account_id.clone().unwrap_or(String::new()).into())
		}),
		recipient_id: request_parameters.filter_recipient_id.map(|x| {
			(x, request_parameters.filter_mode_recipient_id.clone().unwrap_or(String::new()).into())
		}),
		tag_id: request_parameters.filter_tag_id.map(|x| {
			(x, request_parameters.filter_mode_tag_id.clone().unwrap_or(String::new()).into())
		}),
		comment: request_parameters.filter_comment.clone().map(|x| {
			(x, request_parameters.filter_mode_comment.clone().unwrap_or(String::new()).into())
		}),
		time_range: request_parameters.filter_time_range_lower.and_then(|x| {
			request_parameters.filter_time_range_upper.map(|y| {
				(x, y, request_parameters.filter_mode_time_range.clone().unwrap_or(String::new()).into())
			})
		}),
		..Default::default()
	};

	let result = super::DeepTransactionLoader::new(&data.pool)
	.set_query_parameters(
		QueryParameters::default()
			.set_max_results_opt(request_parameters.max_results)
			.set_skip_results_opt(request_parameters.skip_results)
			.set_sort_property_opt(sort_property)
			.set_sort_direction_opt(sort_direction)
			.set_filters(filters)
	)
	.get().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

//TODO: test max results, skip results and filters
#[get("/api/v1/transactions/summary")]
async fn summary(data: web::Data<AppState>, req: HttpRequest, request_parameters: web::Query<RequestParameters>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let filters = Filters { 
		id: request_parameters.filter_id.map(|x| {
			(x, request_parameters.filter_mode_id.clone().unwrap_or(String::new()).into())
		}),
		total_amount: request_parameters.filter_total_amount.map(|x| {
			(x, request_parameters.filter_mode_total_amount.clone().unwrap_or(String::new()).into())
		}),
		asset_id: request_parameters.filter_asset_id.map(|x| {
			(x, request_parameters.filter_mode_asset_id.clone().unwrap_or(String::new()).into())
		}),
		user_id: request_parameters.filter_user_id.map(|x| {
			(x, request_parameters.filter_mode_user_id.clone().unwrap_or(String::new()).into())
		}),
		currency_id: request_parameters.filter_currency_id.map(|x| {
			(x, request_parameters.filter_mode_currency_id.clone().unwrap_or(String::new()).into())
		}),
		account_id: request_parameters.filter_account_id.map(|x| {
			(x, request_parameters.filter_mode_account_id.clone().unwrap_or(String::new()).into())
		}),
		recipient_id: request_parameters.filter_recipient_id.map(|x| {
			(x, request_parameters.filter_mode_recipient_id.clone().unwrap_or(String::new()).into())
		}),
		tag_id: request_parameters.filter_tag_id.map(|x| {
			(x, request_parameters.filter_mode_tag_id.clone().unwrap_or(String::new()).into())
		}),
		comment: request_parameters.filter_comment.clone().map(|x| {
			(x, request_parameters.filter_mode_comment.clone().unwrap_or(String::new()).into())
		}),
		time_range: request_parameters.filter_time_range_lower.and_then(|x| {
			request_parameters.filter_time_range_upper.map(|y| {
				(x, y, request_parameters.filter_mode_time_range.clone().unwrap_or(String::new()).into())
			})
		}),
		..Default::default()
	};

	let result = super::TransactionLoader::new(&data.pool)
		.set_query_parameters(
			QueryParameters::default()
				.set_filters(filters)
		)
		.summarize()
		.await;

	return match result {
		Ok(res) => HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	};
}

#[get("/api/v1/transactions/{transaction_id}")]
async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, transaction_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::TransactionLoader::new(&data.pool)
		.set_filter_id(*transaction_id, NumberFilterModes::Exact)
		.get_first().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => {
			if e.to_string().starts_with("no item of type unknown found") {
				return HttpResponse::NotFound().body(format!("{{\"error\":\"Couldnt find transaction with id {transaction_id}\"}}"));
			}
			
			return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"));
		}
	}
}

#[derive(Deserialize)]
struct TransactionPost {
	account_id: u32,
	recipient_id: u32,
	status: u8,
	timestamp: DateTime<Utc>,
	comment: Option<String>,
	tag_ids: Option<Vec<u32>>,
	asset_id: Option<u32>,
	positions: Vec<super::Position>,
}

#[post("/api/v1/transactions")]
async fn post(data: web::Data<AppState>, req: HttpRequest, body: web::Json<TransactionPost>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let asset: Option<Asset> = body.asset_id.map(|id| Asset::default().set_id(id).set_user_id(user_id));

	let result = super::Transaction::default()
		.set_user_id(user_id)
		.set_account_id(body.account_id)
		.set_recipient_id(body.recipient_id)
		.set_status(match body.status {
			0 => super::TransactionStatus::Withheld,
			1 => super::TransactionStatus::Completed,
			_ => return HttpResponse::BadRequest().body("{{\"error\":\"Invalid status\"}}"),
		})
		.set_timestamp(body.timestamp)
		.set_comment_opt(body.comment.clone())
		.set_tag_ids_opt(body.tag_ids.clone())
		.set_asset_opt(asset)	
		.set_positions(body.positions.clone())
		.save(&data.pool).await;

	match result {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => {
			if e.to_string().starts_with("no item of type unknown found") {
				return HttpResponse::BadRequest().body(format!("{{\"error\":\"specified item of type account not found with filter id={}\"}}", body.account_id));
			}

			return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"))
		},
	}
}

#[put("/api/v1/transactions/{transaction_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<TransactionPost>, transaction_id: web::Path<u32>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let asset: Option<Asset> = body.asset_id.map(|id| Asset::default().set_id(id).set_user_id(user_id));

	let result = super::Transaction::default()
		.set_id(*transaction_id)
		.set_user_id(user_id)
		.set_account_id(body.account_id)
		.set_recipient_id(body.recipient_id)
		.set_status(match body.status {
			0 => super::TransactionStatus::Withheld,
			1 => super::TransactionStatus::Completed,
			_ => return HttpResponse::BadRequest().body("{{\"error\":\"Invalid status\"}}"),
		})
		.set_timestamp(body.timestamp)
		.set_comment_opt(body.comment.clone())
		.set_tag_ids_opt(body.tag_ids.clone())
		.set_asset_opt(asset)	
		.set_positions(body.positions.clone())
		.save(&data.pool).await;

	match result {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[delete("/api/v1/transactions/{transaction_id}")]
async fn delete(data: web::Data<AppState>, req: HttpRequest, transaction_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Transaction::default()
		.set_id(*transaction_id)
		.delete(&data.pool).await;

	return match result {
		Ok(_) => HttpResponse::Ok().body(""),
		Err(e) => HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	};
}