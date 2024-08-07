use actix_web::{get, web, HttpResponse, HttpRequest, Responder};
use crate::webserver::{AppState, is_authorized};

//Docs: /dev/rest_api/users#get-dashboards
#[get("/api/v1/users/me/dashboards")]
pub async fn get_all_of_user(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match super::get_all_of_user(&data.pool, user_id).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}