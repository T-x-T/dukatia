use actix_web::{get, post, put, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use super::super::webserver::{AppState, is_authorized};

#[get("/api/v1/accounts/all")]
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

//TODO: add testing
#[get("/api/v1/accounts/all/deep")]
async fn get_all_deep(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::get_all_deep(&data.pool).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[get("/api/v1/accounts/{account_id}")]
async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, account_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::get_by_id(&data.pool, account_id.into_inner()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => {
			if e.to_string().starts_with("specified item of type account not found with filter") {
				return HttpResponse::NotFound().body(format!("{{\"error\":\"{}\"}}", e));
			} else {
				return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e));
			}
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
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	let account = super::Account {
		id: None,
		name: body.name.clone(),
		user_id: user_id,
		tag_ids: body.tag_ids.clone(),
		default_currency_id: body.default_currency_id,
	};

	match super::add(&data.pool, &account).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[put("/api/v1/accounts/{account_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<AccountPost>, account_id: web::Path<u32>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}",e))
	};

	let account = super::Account {
		id: Some(account_id.into_inner()),
		name: body.name.clone(),
		user_id: user_id,
		tag_ids: body.tag_ids.clone(),
		default_currency_id: body.default_currency_id,
	};

	match super::update(&data.pool, &account).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
	}
}