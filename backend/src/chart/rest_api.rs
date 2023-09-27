use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use chrono::{DateTime, Utc};
use crate::webserver::{AppState, is_authorized};



#[get("/api/v1/charts/{chart_id}")]
async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, chart_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match super::get_by_id(&data.pool, chart_id.into_inner()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[get("/api/v1/dashboards/{dashboard_id}/charts")]
async fn get_all_charts_in_dashboard(data: web::Data<AppState>, req: HttpRequest, dashboard_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match super::get_all_charts_in_dashboard(&data.pool, dashboard_id.into_inner()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}


#[derive(Debug, Clone, Deserialize)]
pub struct ChartOptions {
	pub from_date: Option<DateTime<Utc>>,
	pub to_date: Option<DateTime<Utc>>,
	pub only_parents: Option<bool>,
	pub date_period: Option<String>,
	pub asset_id: Option<u32>,
	pub max_items: Option<u32>,
	pub date_range: Option<u32>,
}

#[get("/api/v1/charts/{chart_id}/data")]
async fn get_chart_data_by_id(data: web::Data<AppState>, req: HttpRequest, chart_id: web::Path<u32>, options: web::Query<ChartOptions>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match super::get_chart_contents_by_id(&data.pool, chart_id.into_inner(), options.into_inner()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[derive(Deserialize)]
struct ChartPost {
	chart_type: String,
	title: String,
	text_template: Option<String>,
	filter_from: Option<DateTime<Utc>>,
	filter_to: Option<DateTime<Utc>>,
	filter_collection: Option<String>,
	date_period: Option<String>,
	max_items: Option<u32>,
	date_range: Option<u32>,
	top_left_x: Option<u32>,
	top_left_y: Option<u32>,
	bottom_right_x: Option<u32>,
	bottom_right_y: Option<u32>,
}

#[post("/api/v1/charts")]
async fn post(data: web::Data<AppState>, req: HttpRequest, body: web::Json<ChartPost>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};
	let body = body.into_inner();
	let chart = super::Chart {
		id: None,
		user_id: Some(user_id),
		chart_type: body.chart_type,
		title: body.title,
		text_template: body.text_template,
		filter_from: body.filter_from,
		filter_to: body.filter_to,
		filter_collection: body.filter_collection,
		date_period: body.date_period,
		asset_id: None,
		max_items: body.max_items,
		date_range: body.date_range,
		top_left_x: body.top_left_x,
		top_left_y: body.top_left_y,
		bottom_right_x: body.bottom_right_x,
		bottom_right_y: body.bottom_right_y,
	};

	match super::add(&data.pool, &chart).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[put("/api/v1/charts/{chart_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<ChartPost>, chart_id: web::Path<u32>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}")),
	};
	let body = body.into_inner();
	let chart = super::Chart {
		id: Some(chart_id.into_inner()),
		user_id: Some(user_id),
		chart_type: body.chart_type,
		title: body.title,
		text_template: body.text_template,
		filter_from: body.filter_from,
		filter_to: body.filter_to,
		filter_collection: body.filter_collection,
		date_period: body.date_period,
		asset_id: None,
		max_items: body.max_items,
		date_range: body.date_range,
		top_left_x: body.top_left_x,
		top_left_y: body.top_left_y,
		bottom_right_x: body.bottom_right_x,
		bottom_right_y: body.bottom_right_y,
	};

	match super::update(&data.pool, &chart).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[delete("/api/v1/charts/{chart_id}")]
async fn delete(data: web::Data<AppState>, req: HttpRequest, chart_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match super::delete(&data.pool, chart_id.into_inner()).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}