use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use uuid::Uuid;
use super::super::webserver::{AppState, is_authorized};
use crate::traits::*;

#[derive(Debug, Deserialize)]
struct RequestParameters {
	skip_results: Option<u32>,
	max_results: Option<u32>,
	filter_id: Option<Uuid>,
	filter_mode_id: Option<String>,
	filter_name: Option<String>,
	filter_mode_name: Option<String>,
	filter_parent_id: Option<Uuid>,
	filter_mode_parent_id: Option<String>,
}

//TODO: test filters for properties other than id
//Docs: /dev/rest_api/tags#get-all
#[get("/api/v1/tags/all")]
pub async fn get_all(data: web::Data<AppState>, req: HttpRequest, request_parameters: web::Query<RequestParameters>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let filters = Filters {
		id: request_parameters.filter_id.map(|x| {
			(x, request_parameters.filter_mode_id.clone().unwrap_or_default().into())
		}),
		name: request_parameters.filter_name.clone().map(|x| {
			(x, request_parameters.filter_mode_name.clone().unwrap_or_default().into())
		}),
		parent_id: request_parameters.filter_parent_id.map(|x| {
			(x, request_parameters.filter_mode_parent_id.clone().unwrap_or_default().into())
		}),
		user_id: Some((user_id, NumberFilterModes::Exact)),
		..Default::default()
	};

	let result = super::TagLoader::new(&data.pool)
	.set_query_parameters(
		QueryParameters::default()
			.set_max_results_opt(request_parameters.max_results)
			.set_skip_results_opt(request_parameters.skip_results)
			.set_filters(filters)
	)
	.get().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

//Docs: /dev/rest_api/tags#get-by-id
#[get("/api/v1/tags/{tag_id}")]
pub async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, tag_id: web::Path<Uuid>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::TagLoader::new(&data.pool)
		.set_filter_id(*tag_id, NumberFilterModes::Exact)
		.set_filter_user_id(user_id, NumberFilterModes::Exact)
		.get_first().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => {
			if e.to_string().starts_with("no item of type unknown found") {
				return HttpResponse::NotFound().body(format!("{{\"error\":\"specified item of type tag not found with filter id={tag_id}\"}}"));
			}
		
			return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"));
		}
	}
}

#[derive(Deserialize)]
struct TagPost {
	name: String,
	parent_id: Option<Uuid>
}

//Docs: /dev/rest_api/tags#create-tag
#[post("/api/v1/tags")]
pub async fn post(data: web::Data<AppState>, req: HttpRequest, body: web::Json<TagPost>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Tag::default()
		.set_name(body.name.clone())
		.set_parent_id_opt(body.parent_id)
		.set_user_id(user_id)
		.create(&data.pool).await;

	match result {
		Ok(x) => return HttpResponse::Ok().body(format!("{{\"id\":\"{x}\"}}")),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

//Docs: /dev/rest_api/tags#update-tag
#[put("/api/v1/tags/{tag_id}")]
pub async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<TagPost>, tag_id: web::Path<Uuid>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Tag::default()
		.set_id(tag_id.into_inner())
		.set_name(body.name.clone())
		.set_parent_id_opt(body.parent_id)
		.set_user_id(user_id)
		.update(&data.pool).await;

		match result {
			Ok(()) => return HttpResponse::Ok().body(""),
			Err(e) => {
				if e.to_string().starts_with("you can only access items you own") {
					return HttpResponse::NotFound().body("");
				}
	
				return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"))
			},
		}
}

//Docs: /dev/rest_api/tags#delete-tag
#[delete("/api/v1/tags/{tag_id}")]
pub async fn delete(data: web::Data<AppState>, req: HttpRequest, tag_id: web::Path<Uuid>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Tag::default()
		.set_id(tag_id.into_inner())
		.set_user_id(user_id)
		.delete(&data.pool).await;

		match result {
			Ok(()) => return HttpResponse::Ok().body(""),
			Err(e) => {
				if e.to_string().starts_with("you can only access items you own") {
					return HttpResponse::NotFound().body("");
				}
	
				return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"))
			},
		}
}