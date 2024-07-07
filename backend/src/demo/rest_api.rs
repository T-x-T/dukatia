use actix_web::{post, web, HttpResponse, HttpRequest, Responder};
use crate::webserver::{AppState, is_authorized};

//Docs: undocumented on purpose -> move to some scope like users or something and then document
#[post("/api/v1/insert_demo_data")]
pub async fn insert_demo_data(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match super::insert_demo_data(&data.pool, user_id).await {
		Ok(()) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}