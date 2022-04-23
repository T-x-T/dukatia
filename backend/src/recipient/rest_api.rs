use actix_web::{get, post, put, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use super::super::webserver::{AppState, is_authorized};

#[get("/api/v1/recipients/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	match super::get_all(&data.pool).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[derive(Deserialize)]
struct RecipientPost {
	name: String,
	tag_ids: Option<Vec<u32>>,
}

#[post("/api/v1/recipients")]
async fn post(data: web::Data<AppState>, req: HttpRequest, body: web::Json<RecipientPost>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}", e))
	};

	let recipient = super::Recipient {
		id: None,
		name: body.name.clone(),
		user_id: Some(user_id),
		tag_ids: body.tag_ids.clone(),
	};

	match super::add(&data.pool, &recipient).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e)),
	}
}

#[put("/api/v1/recipients/{recipient_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<RecipientPost>, recipient_id: web::Path<u32>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}",e))
	};

	let recipient = super::Recipient {
		id: Some(recipient_id.into_inner()),
		name: body.name.to_string(),
		user_id: Some(user_id),
		tag_ids: body.tag_ids.clone()
	};

	match super::update(&data.pool, &recipient).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
	}
}

