use actix_web::{post, web, HttpResponse, HttpRequest, Responder, put};
use serde::Deserialize;
use super::super::webserver::{AppState, is_authorized};
use super::super::user::LoginCredentials;


#[post("/api/v1/login")]
async fn post_login(data: web::Data<AppState>, body: web::Json<LoginCredentials>) -> impl Responder {
	match super::login(&data.config, &data.pool, body.into_inner()).await {
		Ok(access_token) => return HttpResponse::Ok().body(format!("{{\"accessToken\":\"{access_token}\"}}")),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	};
}

#[post("/api/v1/logout")]
async fn post_logout(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match super::logout(&data.pool, user_id, req.cookie("accessToken").unwrap().value().to_string()).await {
		Ok(()) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	};
}


#[derive(Deserialize)]
struct PutSecretBody {
	old_secret: String,
	new_secret: String,
}

#[put("/api/v1/users/me/secret")]
async fn put_secret(data: web::Data<AppState>, body: web::Json<PutSecretBody>, req: HttpRequest) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match super::update_secret(&data.config, &data.pool, body.old_secret.clone(), body.new_secret.clone(), user_id).await {
		Ok(()) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}