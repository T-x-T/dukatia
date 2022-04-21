use actix_web::{get, post, put, web, HttpResponse, HttpRequest, Responder, http::header::ContentType};
use serde::Deserialize;
use super::super::is_authorized;
use super::super::webserver::AppState;

#[get("/api/v1/tags/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}",e))
	};

	match super::get_all(&data.pool).await {
		Ok(res) => return HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}",e)),
	}
}

#[derive(Deserialize)]
struct TagPost {
	name: String,
	parent_id: Option<u32>
}

#[post("/api/v1/tags")]
async fn post(data: web::Data<AppState>, req: HttpRequest, body: web::Json<TagPost>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}",e))
	};

	let tag = super::Tag {
		id: None,
		name: body.name.to_string(),
		parent_id: body.parent_id,
		user_id: user_id 
	};

	match super::add(&data.pool, &tag).await {
		Ok(_) => return HttpResponse::Ok().content_type(ContentType::json()).body(""),
		Err(e) => return HttpResponse::BadRequest().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}",e)),
	}
}

#[put("/api/v1/tags/{tag_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<TagPost>, tag_id: web::Path<u32>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}",e))
	};

	let tag = super::Tag {
		id: Some(tag_id.into_inner()),
		name: body.name.to_string(),
		parent_id: body.parent_id,
		user_id: user_id 
	};

	match super::update(&data.pool, &tag).await {
		Ok(_) => return HttpResponse::Ok().content_type(ContentType::json()).body(""),
		Err(e) => return HttpResponse::BadRequest().content_type(ContentType::json()).body(format!("{{\"error\":\"{}\"}}",e)),
	}
}

