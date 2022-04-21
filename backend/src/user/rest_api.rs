use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use super::super::webserver::AppState;
use super::super::user::{User, login};

#[derive(Deserialize)]
struct LoginData {
	name: String,
	secret: String,
}

#[post("/api/v1/login")]
async fn post_login(data: web::Data<AppState>, body: web::Json<LoginData>) -> impl Responder {
	let user = User {
		id: None,
		name: body.name.clone(),
		secret: body.secret.to_string(),
		superuser: false,
	};

	match login(&data.config, &data.pool, user).await {
		Ok(access_token) => return HttpResponse::Ok().body(format!("{{\"accessToken\":\"{}\"}}", access_token)),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	};
}