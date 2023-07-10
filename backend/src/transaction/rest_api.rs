use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use chrono::prelude::*;
use super::super::webserver::{AppState, is_authorized};
use super::Asset;

#[get("/api/v1/transactions/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::TransactionLoader::new(&data.pool).get().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[get("/api/v1/transactions/all/deep")]
async fn get_all_deep(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::TransactionLoader::new(&data.pool).all_deep().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[get("/api/v1/transactions/{transaction_id}")]
async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, transaction_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::TransactionLoader::new(&data.pool)
		.set_filter_id(*transaction_id)
		.get_first().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => {
			if e.to_string().starts_with("no item of type Transaction found") {
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

	//TODO: replace this once Asset implements Default and uses impl
	let mut asset: Option<Asset> = None;
	if body.asset_id.is_some() {
		asset = Some(Asset {
			id: body.asset_id,
			name: String::new(),
			currency_id: 0,
			user_id,
			amount: None,
			value_per_unit: None,
			description: None,
			tag_ids: None,
			total_cost_of_ownership: None,
		});
	}

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
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[put("/api/v1/transactions/{transaction_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<TransactionPost>, transaction_id: web::Path<u32>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let mut asset: Option<Asset> = None;
	if body.asset_id.is_some() {
		asset = Some(Asset {
			id: body.asset_id,
			name: String::new(),
			currency_id: 0,
			user_id,
			amount: None,
			value_per_unit: None,
			description: None,
			tag_ids: None,
			total_cost_of_ownership: None,
		});
	}

	let result = super::Transaction::default()
		.set_id(transaction_id.into_inner())
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
		.set_id(transaction_id.into_inner())
		.delete(&data.pool).await;

	return match result {
		Ok(_) => HttpResponse::Ok().body(""),
		Err(e) => HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	};
}