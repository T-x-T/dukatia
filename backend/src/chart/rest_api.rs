use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::webserver::{AppState, is_authorized};


//Docs: /dev/rest_api/charts#get-by-id
#[get("/api/v1/charts/{chart_id}")]
pub async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, chart_id: web::Path<Uuid>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match super::get_by_id(&data.pool, chart_id.into_inner(), user_id).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

//Docs: /dev/rest_api/charts#get-all-charts-in-dashboard
#[get("/api/v1/dashboards/{dashboard_id}/charts")]
pub async fn get_all_charts_in_dashboard(data: web::Data<AppState>, req: HttpRequest, dashboard_id: web::Path<Uuid>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match super::get_all_charts_in_dashboard(&data.pool, dashboard_id.into_inner(), user_id).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}


#[derive(Debug, Clone, Deserialize)]
struct ChartOptionsQuery {
	from_date: Option<DateTime<Utc>>,
	to_date: Option<DateTime<Utc>>,
	date_period: Option<String>,
	asset_id: Option<Uuid>,
	budget_id: Option<Uuid>,
	max_items: Option<u32>,
	date_range: Option<u32>,
	only_positive: Option<bool>,
	only_negative: Option<bool>,
	filter_collection: Option<String>,
	start_at_zero: Option<bool>,
}

//Docs: /dev/rest_api/charts#get-data-of-chart-by-id
#[get("/api/v1/charts/{chart_id}/data")]
pub async fn get_chart_data_by_id(data: web::Data<AppState>, req: HttpRequest, chart_id: web::Path<Uuid>, options: web::Query<ChartOptionsQuery>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let mut chart_options = match super::get_by_id(&data.pool, *chart_id, user_id).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"failed getting chart data: {e}\"}}")),
	};
	
	if options.from_date.is_some() {
		chart_options.filter_from = options.from_date;
	}
	if options.to_date.is_some() {
		chart_options.filter_to = options.to_date;
	}
	if options.date_period.is_some() {
		chart_options.date_period.clone_from(&Some(options.date_period.as_ref().unwrap().as_str().into()));
	}
	if options.asset_id.is_some() {
		chart_options.asset_id = options.asset_id;
	}
	if options.budget_id.is_some() {
		chart_options.budget_id = options.budget_id;
	}
	if options.max_items.is_some() {
		chart_options.max_items = options.max_items;
	}
	if options.date_range.is_some() {
		chart_options.date_range = options.date_range.map(std::convert::Into::into);
	}
	if options.only_positive.is_some() {
		chart_options.only_positive = options.only_positive;
	}
	if options.only_negative.is_some() {
		chart_options.only_negative = options.only_negative;
	}
	if options.start_at_zero.is_some() {
		chart_options.start_at_zero = options.start_at_zero;
	}

	match super::get_chart_data(&data.pool, chart_options).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

//Docs: /dev/rest_api/charts#get-data-of-chart-by-filter-collection
#[get("/api/v1/charts/by_collection/{filter_collection}")]
pub async fn get_chart_data_by_filter_collection(data: web::Data<AppState>, req: HttpRequest, path: web::Path<String>, options: web::Query<ChartOptionsQuery>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let chart_options = super::ChartOptions {
		id: Uuid::nil(),
		user_id,
		chart_type: super::ChartType::Line,
		title: options.filter_collection.clone().unwrap_or_default(),
		filter_from: options.from_date,
		filter_to: options.to_date,
		filter_collection: Some(path.as_str().into()),
		date_period: options.date_period.as_ref().map(|x| x.as_str().into()),
		asset_id: options.asset_id,
		budget_id: options.budget_id,
		max_items: options.max_items,
		date_range: options.date_range.map(std::convert::Into::into),
		only_positive: options.only_positive,
		only_negative: options.only_negative,
		top_left_x: None,
		top_left_y: None,
		bottom_right_x: None,
		bottom_right_y: None,
		dashboard_id: None,
		start_at_zero: options.start_at_zero,
	};

	match super::get_chart_data(&data.pool, chart_options).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[derive(Deserialize)]
struct ChartPost {
	chart_type: String,
	title: String,
	filter_from: Option<DateTime<Utc>>,
	filter_to: Option<DateTime<Utc>>,
	filter_collection: Option<String>,
	date_period: Option<String>,
	max_items: Option<u32>,
	date_range: Option<u32>,
	only_positive: Option<bool>,
	only_negative: Option<bool>,
	top_left_x: Option<u32>,
	top_left_y: Option<u32>,
	bottom_right_x: Option<u32>,
	bottom_right_y: Option<u32>,
	dashboard_id: Option<Uuid>,
	start_at_zero: Option<bool>,
}

//Docs: /dev/rest_api/charts#create-chart
#[post("/api/v1/charts")]
pub async fn post(data: web::Data<AppState>, req: HttpRequest, body: web::Json<ChartPost>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};
	let body = body.into_inner();
	let chart = super::ChartOptions {
		id: Uuid::new_v4(),
		user_id,
		chart_type: body.chart_type.as_str().into(),
		title: body.title,
		filter_from: body.filter_from,
		filter_to: body.filter_to,
		filter_collection: body.filter_collection.map(|x| x.as_str().into()),
		date_period: body.date_period.map(|x| x.as_str().into()),
		asset_id: None,
		budget_id: None,
		max_items: body.max_items,
		date_range: body.date_range.map(std::convert::Into::into),
		only_positive: body.only_positive,
		only_negative: body.only_negative,
		top_left_x: body.top_left_x,
		top_left_y: body.top_left_y,
		bottom_right_x: body.bottom_right_x,
		bottom_right_y: body.bottom_right_y,
		dashboard_id: body.dashboard_id,
		start_at_zero: body.start_at_zero,
	};

	match super::add(&data.pool, &chart).await {
		Ok(id) => return HttpResponse::Ok().body(format!("{{\"id\":\"{id}\"}}")),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

//Docs: /dev/rest_api/charts#modify-chart
#[put("/api/v1/charts/{chart_id}")]
pub async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<ChartPost>, chart_id: web::Path<Uuid>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}")),
	};
	
	let body = body.into_inner();
	let chart = super::ChartOptions {
		id: *chart_id,
		user_id,
		chart_type: body.chart_type.as_str().into(),
		title: body.title,
		filter_from: body.filter_from,
		filter_to: body.filter_to,
		filter_collection: body.filter_collection.map(|x| x.as_str().into()),
		date_period: body.date_period.map(|x| x.as_str().into()),
		asset_id: None,
		budget_id: None,
		max_items: body.max_items,
		date_range: body.date_range.map(std::convert::Into::into),
		only_positive: body.only_positive,
		only_negative: body.only_negative,
		top_left_x: body.top_left_x,
		top_left_y: body.top_left_y,
		bottom_right_x: body.bottom_right_x,
		bottom_right_y: body.bottom_right_y,
		dashboard_id: body.dashboard_id,
		start_at_zero: body.start_at_zero,
	};

	match super::update(&data.pool, &chart).await {
		Ok(()) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

//Docs: /dev/rest_api/charts#delete-chart
#[delete("/api/v1/charts/{chart_id}")]
pub async fn delete(data: web::Data<AppState>, req: HttpRequest, chart_id: web::Path<Uuid>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match super::delete(&data.pool, chart_id.into_inner()).await {
		Ok(()) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}