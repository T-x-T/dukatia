use actix_web::{get, post, put, web, HttpResponse, HttpRequest, Responder};
use serde::Deserialize;
use uuid::Uuid;
use crate::webserver::{AppState, is_authorized};
use crate::traits::*;

#[derive(Debug, Deserialize)]
struct RequestParameters {
	skip_results: Option<u32>,
	max_results: Option<u32>,
	filter_id: Option<Uuid>,
	filter_mode_id: Option<String>,
	filter_name: Option<String>,
	filter_mode_name: Option<String>,
	filter_tag_id: Option<Uuid>,
	filter_mode_tag_id: Option<String>,
}

//TODO: test filters for properties other than id
#[get("/api/v1/recipients/all")]
async fn get_all(data: web::Data<AppState>, req: HttpRequest, request_parameters: web::Query<RequestParameters>) -> impl Responder {
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
		tag_id: request_parameters.filter_tag_id.map(|x| {
			(x, request_parameters.filter_mode_tag_id.clone().unwrap_or_default().into())
		}),
		user_id: Some((user_id, NumberFilterModes::ExactOrAlsoNull)),
		..Default::default()
	};

	let result = super::RecipientLoader::new(&data.pool)
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

#[get("/api/v1/recipients/{recipient_id}")]
async fn get_by_id(data: web::Data<AppState>, req: HttpRequest, recipient_id: web::Path<Uuid>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::RecipientLoader::new(&data.pool)
		.set_filter_id(*recipient_id, NumberFilterModes::Exact)
		.set_filter_user_id(user_id, NumberFilterModes::ExactOrAlsoNull)
		.get_first().await;

	match result {
		Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
		Err(e) => {
			if e.to_string().starts_with("no item of type unknown found") {
				return HttpResponse::NotFound().body(format!("{{\"error\":\"specified item of type recipient not found with filter id={recipient_id}\"}}"));
			}		
			
			return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}"));
		}
	}
}

#[derive(Deserialize)]
struct RecipientPost {
	name: String,
	tag_ids: Option<Vec<Uuid>>,
}

#[post("/api/v1/recipients")]
async fn post(data: web::Data<AppState>, req: HttpRequest, body: web::Json<RecipientPost>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Recipient::default()
		.set_name(body.name.clone())
		.set_tag_ids(body.tag_ids.clone().unwrap_or_default())
		.set_user_id(user_id)
		.create(&data.pool).await;

	match result {
		Ok(x) => return HttpResponse::Ok().body(format!("{{\"id\":\"{x}\"}}")),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[put("/api/v1/recipients/{recipient_id}")]
async fn put(data: web::Data<AppState>, req: HttpRequest, body: web::Json<RecipientPost>, recipient_id: web::Path<Uuid>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let result = super::Recipient::default()
		.set_id(*recipient_id)
		.set_name(body.name.clone())
		.set_tag_ids(body.tag_ids.clone().unwrap_or_default())
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
