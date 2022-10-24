use actix_web::{get, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use super::super::webserver::{AppState, is_authorized};
use super::Period;

#[derive(Deserialize)]
struct Options {
	from_date: Option<chrono::NaiveDate>,
	to_date: Option<chrono::NaiveDate>,
	only_parents: Option<bool>,
	period: Option<Period>,
}

#[get("/api/v1/reports/balance_over_time_per_currency")]
async fn balance_over_time_per_currency(data: web::Data<AppState>, req: HttpRequest, date_control: web::Query<Options>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::balance_over_time_per_currency(&data.pool, date_control.from_date, date_control.to_date).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[get("/api/v1/reports/balance_over_time_per_recipient")]
async fn balance_over_time_per_recipient(data: web::Data<AppState>, req: HttpRequest, date_control: web::Query<Options>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::balance_over_time_per_recipient(&data.pool, date_control.from_date, date_control.to_date).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[get("/api/v1/reports/balance_over_time_per_account")]
async fn balance_over_time_per_account(data: web::Data<AppState>, req: HttpRequest, date_control: web::Query<Options>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::balance_over_time_per_account(&data.pool, date_control.from_date, date_control.to_date).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[get("/api/v1/reports/balance_over_time")]
async fn balance_over_time(data: web::Data<AppState>, req: HttpRequest, options: web::Query<Options>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	if options.period.is_none() {
		return HttpResponse::BadRequest().body("{{\"error\":\"Missing period query option\"}}");
	}

	match super::balance_over_time(&data.pool, options.from_date, options.to_date, options.period.unwrap()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[get("/api/v1/reports/total_per_currency")]
async fn total_per_currency(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::total_per_currency(&data.pool).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[get("/api/v1/reports/spending_per_recipient_in_date_range")]
async fn spending_per_recipient_in_date_range(data: web::Data<AppState>, req: HttpRequest, date_control: web::Query<Options>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	if date_control.from_date.is_none() || date_control.to_date.is_none() {
		return HttpResponse::BadRequest().body("{{\"error\":\"from_date and to_date are required\"}}");
	}

	match super::spending_per_recipient_in_date_range(&data.pool, date_control.from_date.unwrap(), date_control.to_date.unwrap()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[get("/api/v1/reports/spending_per_tag_in_date_range")]
async fn spending_per_tag_in_date_range(data: web::Data<AppState>, req: HttpRequest, options: web::Query<Options>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	if options.from_date.is_none() || options.to_date.is_none() {
		return HttpResponse::BadRequest().body("{{\"error\":\"from_date and to_date are required\"}}");
	}

	let actual_only_parent = options.only_parents.is_some() && options.only_parents.unwrap();

	match super::spending_per_tag_in_date_range(&data.pool, options.from_date.unwrap(), options.to_date.unwrap(), actual_only_parent).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[get("/api/v1/reports/value_per_unit_over_time_for_asset/{asset_id}")]
async fn value_per_unit_over_time_for_asset(data: web::Data<AppState>, req: HttpRequest, asset_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::value_per_unit_over_time_for_asset(&data.pool, asset_id.into_inner()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[get("/api/v1/reports/amount_over_time_for_asset/{asset_id}")]
async fn amount_over_time_for_asset(data: web::Data<AppState>, req: HttpRequest, asset_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::amount_over_time_for_asset(&data.pool, asset_id.into_inner()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}