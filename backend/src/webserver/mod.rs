use actix_web::{dev::Service as _, web, App, HttpServer, HttpRequest, middleware};
use std::error::Error;
use futures_util::future::FutureExt;
use deadpool_postgres::Pool;

use super::CustomError;
use super::config::Config;
use super::user;
use super::currency;
use super::account;
use super::tag;
use super::recipient;
use super::transaction;
use super::reports;
use super::asset;
use super::access_token::get_user_of_token;

pub struct AppState {
	pub config: Config,
	pub pool: Pool
}

pub async fn initialize_webserver(config: Config, pool: Pool) -> std::io::Result<()> {
	println!("Starting webserver on port {}", config.api_port);
	let api_port = config.api_port;
	return HttpServer::new(move|| {
		return App::new()
			.app_data(web::Data::new(AppState {
				config: config.clone(),
				pool: pool.clone()
			}))
			.wrap(middleware::Compress::default())
			.wrap(middleware::DefaultHeaders::new().add(("Content-Type", "application/json")))
			.wrap_fn(|req, srv| {
				println!("req: {} {}", req.method(), req.path());
				srv.call(req).map(|res| {
					return res;
				})
			})
			.service(reports::rest_api::balance_over_time_per_currency)
			.service(reports::rest_api::balance_over_time_per_recipient)
			.service(reports::rest_api::balance_over_time_per_account)
			.service(reports::rest_api::balance_over_time)
			.service(reports::rest_api::total_per_currency)
			.service(reports::rest_api::spending_per_recipient_in_date_range)
			.service(reports::rest_api::spending_per_tag_in_date_range)
			.service(reports::rest_api::value_per_unit_over_time_for_asset)
			.service(reports::rest_api::amount_over_time_for_asset)
			.service(user::rest_api::post_login)
			.service(user::rest_api::put_secret)
			.service(account::rest_api::get_all)
			.service(account::rest_api::post)
			.service(account::rest_api::put)
			.service(tag::rest_api::get_all)
			.service(tag::rest_api::post)
			.service(tag::rest_api::put)
			.service(tag::rest_api::delete)
			.service(recipient::rest_api::get_all)
			.service(recipient::rest_api::post)
			.service(recipient::rest_api::put)
			.service(transaction::rest_api::get_all)
			.service(transaction::rest_api::post)
			.service(transaction::rest_api::put)
			.service(transaction::rest_api::delete)
			.service(asset::rest_api::get_all)
			.service(asset::rest_api::post)
			.service(asset::rest_api::put)
			.service(asset::rest_api::delete_by_id)
			.service(asset::rest_api::post_valuation)
			.service(currency::rest_api::get_all);
	})
		.bind(("0.0.0.0", api_port))?
		.run()
		.await;
}

pub async fn is_authorized(pool: &Pool, req: &HttpRequest) -> Result<u32, Box<dyn Error>> {
	if req.cookie("accessToken").is_none() {
    return Err(Box::new(CustomError::MissingCookie{cookie: String::from("accessToken")}));
  }

	return get_user_of_token(&pool, &req.cookie("accessToken").unwrap().value().to_string()).await;
}