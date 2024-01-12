use actix_web::{get, post, web, HttpResponse, HttpRequest, Responder, put};
use serde::Deserialize;
use super::super::webserver::{AppState, is_authorized};
use super::*;


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

	match User::default().set_id(user_id).logout(&data.pool, req.cookie("accessToken").unwrap().value().to_string()).await {
		Ok(()) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	};
}

#[get("/api/v1/users/me")]
async fn get_me(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match UserLoader::new(&data.pool)
		.set_filter_id(user_id, NumberFilterModes::Exact)
		.get_first()
		.await {
			Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
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

	let user = match UserLoader::new(&data.pool)
		.set_filter_id(user_id, NumberFilterModes::Exact)
		.get_first()
		.await {
			Ok(x) => x,
			Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
		};

	match user.update_secret(&data.pool, &data.config.pepper, body.old_secret.clone(), body.new_secret.clone()).await {
		Ok(()) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}