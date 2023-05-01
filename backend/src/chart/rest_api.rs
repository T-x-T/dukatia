use actix_web::{get, web, HttpResponse, HttpRequest, Responder};
use crate::webserver::{AppState, is_authorized};



#[get("/api/v1/charts/{chart_id}")]
async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, chart_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::get_by_id(&data.pool, chart_id.into_inner()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[get("/api/v1/dashboards/{dashboard_id}/charts")]
async fn get_all_charts_in_dashboard(data: web::Data<AppState>, req: HttpRequest, dashboard_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::get_all_charts_in_dashboard(&data.pool, dashboard_id.into_inner()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[get("/api/v1/charts/{chart_id}/data")]
async fn get_chart_data_by_id(data: web::Data<AppState>, req: HttpRequest, chart_id: web::Path<u32>, options: web::Query<super::ChartOptions>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::get_chart_contents_by_id(&data.pool, chart_id.into_inner(), options.into_inner()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}
