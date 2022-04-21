use actix_web::{get, web, HttpResponse, HttpRequest, Responder, http::header::ContentType};
use super::super::is_authorized;
use super::super::webserver::AppState;

#[get("/api/v1/currencies/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::get_all(&data.pool).await {
		Ok(res) => return HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}", e)),
	}
}