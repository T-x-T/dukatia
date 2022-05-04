use actix_web::{get, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use super::super::webserver::{AppState, is_authorized};

#[derive(Deserialize)]
struct DateControl {
	from_date: Option<chrono::NaiveDate>,
	to_date: Option<chrono::NaiveDate>,
}

#[get("/api/v1/reports/balance_over_time_per_currency")]
async fn balance_over_time_per_currency(data: web::Data<AppState>, req: HttpRequest, date_control: web::Query<DateControl>) -> impl Responder {
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
async fn balance_over_time_per_recipient(data: web::Data<AppState>, req: HttpRequest, date_control: web::Query<DateControl>) -> impl Responder {
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
async fn balance_over_time_per_account(data: web::Data<AppState>, req: HttpRequest, date_control: web::Query<DateControl>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::balance_over_time_per_account(&data.pool, date_control.from_date, date_control.to_date).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}