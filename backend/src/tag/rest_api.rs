use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use super::super::webserver::{AppState, is_authorized};

#[get("/api/v1/tags/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}",e))
	};

	match super::get_all(&data.pool).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
	}
}

#[get("/api/v1/tags/all/deep")]
async fn get_all_deep(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}",e))
	};

	match super::get_all_deep(&data.pool).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
	}
}

#[get("/api/v1/tags/{tag_id}")]
async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, tag_id: web::Path<u32>) -> impl Responder {
	let _user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}",e))
	};

	match super::get_by_id(&data.pool, tag_id.into_inner()).await {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => {
			if e.to_string().starts_with("specified item of type tag not found with filter") {
				return HttpResponse::NotFound().body(format!("{{\"error\":\"{}\"}}", e));
			} else {
				return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}", e));
			}
		}
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
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}",e))
	};

	let tag = super::Tag {
		id: None,
		name: body.name.to_string(),
		parent_id: body.parent_id,
		user_id: user_id 
	};

	match super::add(&data.pool, &tag).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
	}
}

#[put("/api/v1/tags/{tag_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<TagPost>, tag_id: web::Path<u32>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}",e))
	};

	let tag = super::Tag {
		id: Some(tag_id.into_inner()),
		name: body.name.to_string(),
		parent_id: body.parent_id,
		user_id: user_id 
	};

	match super::update(&data.pool, &tag).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
	}
}

#[delete("/api/v1/tags/{tag_id}")]
async fn delete(data: web::Data<AppState>, req: HttpRequest, tag_id: web::Path<u32>) -> impl Responder {
	let _ = match is_authorized(&data.pool, &req).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{}\"}}",e))
	};

	match super::delete(&data.pool, tag_id.into_inner()).await {
		Ok(_) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{}\"}}",e)),
	}
}