use actix_web::{get, post, web, HttpResponse, HttpRequest, Responder, put};
use serde::Deserialize;
use super::super::webserver::{AppState, is_authorized};
use super::*;

//Docs: /dev/rest_api/users#login
#[post("/api/v1/login")]
pub async fn post_login(data: web::Data<AppState>, body: web::Json<LoginCredentials>) -> impl Responder {
	match super::login(&data.config, &data.pool, body.into_inner()).await {
		Ok(login_result) => return HttpResponse::Ok().body(serde_json::to_string(&login_result).unwrap()),
		Err(_) => return HttpResponse::BadRequest().body(println!("{{\"error\":\"the given credentials are invalid\"}}")),
	};
}

//Docs: /dev/rest_api/users#logout
#[post("/api/v1/logout")]
pub async fn post_logout(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match User::default().set_id(user_id).logout(&data.pool, req.cookie("access_token").unwrap().value().to_string()).await {
		Ok(()) => return HttpResponse::Ok().body(""),
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	};
}

//Docs: /dev/rest_api/users#get-me
#[get("/api/v1/users/me")]
pub async fn get_me(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
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

//Docs: /dev/rest_api/users#get-all-users
#[get("/api/v1/users/all")]
pub async fn get_all(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	let users = UserLoader::new(&data.pool)
		.set_query_parameters(
			QueryParameters::default()
				.set_sort_property_opt(Some(FilterAndSortProperties::Name))
				.set_sort_direction_opt(Some(SortDirection::Asc))
			)
		.get()
		.await;

	match users {
		Ok(res) => {
			let user: Vec<&User> = res.iter().filter(|x| x.id == user_id).collect();
			
			if user.first().unwrap().superuser {
				return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap())
			}

			return HttpResponse::BadRequest().body("{\"error\":\"you need to be a superuser for this action\"}".to_string());
		},
		Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
	};
}

#[derive(Deserialize)]
struct PostUserBody {
	name: String,
	superuser: bool,
	secret: String,
}

//Docs: /dev/rest_api/users#create-user
#[post("/api/v1/users")]
pub async fn post(data: web::Data<AppState>, body: web::Json<PostUserBody>, req: HttpRequest) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match UserLoader::new(&data.pool)
		.set_filter_id(user_id, NumberFilterModes::Exact)
		.get_first()
		.await {
			Ok(user) => {
				if user.superuser {
					match User::default()
						.set_name(body.name.clone())
						.set_superuser(body.superuser)
						.set_secret(body.secret.clone())
						.encrypt_secret(&data.config.pepper)
						.create(&data.pool)
						.await {
							Ok(_) => return HttpResponse::Ok().body(""),
							Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
						};	
				}

				return HttpResponse::BadRequest().body("{\"error\":\"you need to be a superuser for this action\"}".to_string());
			},
			Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
		};
}

#[derive(Deserialize)]
struct PutUserBody {
	superuser: Option<bool>,
	secret: Option<String>,
	active: Option<bool>,
}

//Docs: /dev/rest_api/users#update-user
#[put("/api/v1/users/{req_user_id}")]
pub async fn put(data: web::Data<AppState>, body: web::Json<PutUserBody>, req: HttpRequest, req_user_id: web::Path<Uuid>) -> impl Responder {
	let user_id = match is_authorized(&data.pool, &req, data.config.session_expiry_days).await {
		Ok(x) => x,
		Err(e) => return HttpResponse::Unauthorized().body(format!("{{\"error\":\"{e}\"}}"))
	};

	match UserLoader::new(&data.pool)
		.set_filter_id(user_id, NumberFilterModes::Exact)
		.get_first()
		.await {
			Ok(user) => {
				if user.superuser {
					match UserLoader::new(&data.pool)
						.set_filter_id(*req_user_id, NumberFilterModes::Exact)
						.get_first()
						.await {
							Ok(mut user_to_edit) => {
								if body.superuser.is_some() {
									user_to_edit.set_superuser_mut(body.superuser.unwrap());
								}

								if body.secret.is_some() {
									user_to_edit.set_secret_mut(body.secret.clone().unwrap());
									user_to_edit.encrypt_secret_mut(&data.config.pepper);
								}

								if body.active.is_some() {
									if user_to_edit.id == user_id {
										return HttpResponse::BadRequest().body("{\"error\":\"you cant edit yourself!\"}")
									}

									user_to_edit.set_active_mut(body.active.unwrap());
								}

								match user_to_edit.update(&data.pool).await {
									Ok(()) => return HttpResponse::Ok().body(""),
									Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
								}
							},
							Err(e) => return HttpResponse::NotFound().body(format!("{{\"error\":\"{e}\"}}")),
						}
				}

				return HttpResponse::BadRequest().body("{\"error\":\"you need to be a superuser for this action\"}".to_string());
			},
			Err(e) => return HttpResponse::BadRequest().body(format!("{{\"error\":\"{e}\"}}")),
		};
}

#[derive(Deserialize)]
struct PutSecretBody {
	old_secret: String,
	new_secret: String,
}

//Docs: /dev/rest_api/users#update-user
#[put("/api/v1/users/me/secret")]
pub async fn put_secret(data: web::Data<AppState>, body: web::Json<PutSecretBody>, req: HttpRequest) -> impl Responder {
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