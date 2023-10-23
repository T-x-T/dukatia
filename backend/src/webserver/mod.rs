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
use super::asset;
use super::dashboard;
use super::chart;
use super::budget;
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
			.service(user::rest_api::post_login)
			.service(user::rest_api::post_logout)
			.service(user::rest_api::put_secret)
			.service(account::rest_api::get_all)
			.service(account::rest_api::get_by_id)
			.service(account::rest_api::post)
			.service(account::rest_api::put)
			.service(tag::rest_api::get_all)
			.service(tag::rest_api::get_by_id)
			.service(tag::rest_api::post)
			.service(tag::rest_api::put)
			.service(tag::rest_api::delete)
			.service(recipient::rest_api::get_all)
			.service(recipient::rest_api::get_by_id)
			.service(recipient::rest_api::post)
			.service(recipient::rest_api::put)
			.service(transaction::rest_api::get_all)
			.service(transaction::rest_api::summary)
			.service(transaction::rest_api::get_by_id)
			.service(transaction::rest_api::post)
			.service(transaction::rest_api::put)
			.service(transaction::rest_api::delete)
			.service(asset::rest_api::get_all)
			.service(asset::rest_api::get_by_id)
			.service(asset::rest_api::post)
			.service(asset::rest_api::put)
			.service(asset::rest_api::delete_by_id)
			.service(asset::rest_api::post_valuation)
			.service(asset::rest_api::get_valuation_history_by_asset_id)
			.service(asset::rest_api::replace_valuation_history_of_asset)
			.service(dashboard::rest_api::get_all_of_user)
			.service(chart::rest_api::get_by_id)
			.service(chart::rest_api::get_chart_data_by_id)
			.service(chart::rest_api::get_chart_data_by_type_filter_collection)
			.service(chart::rest_api::get_all_charts_in_dashboard)
			.service(chart::rest_api::post)
			.service(chart::rest_api::put)
			.service(chart::rest_api::delete)
			.service(currency::rest_api::get_all)
			.service(currency::rest_api::get_by_id)
			.service(currency::rest_api::post)
			.service(currency::rest_api::put)
			.service(budget::rest_api::delete)
			.service(budget::rest_api::get_all)
			.service(budget::rest_api::get_by_id)
			.service(budget::rest_api::post)
			.service(budget::rest_api::put);
	})
		.bind(("0.0.0.0", api_port))?
		.run()
		.await;
}

pub async fn is_authorized(pool: &Pool, req: &HttpRequest, session_expiry_days: u32) -> Result<u32, Box<dyn Error>> {
	if req.cookie("accessToken").is_none() {
    return Err(Box::new(CustomError::MissingCookie{cookie: String::from("accessToken")}));
  }

	#[allow(clippy::needless_question_mark)] //otherwise vscode freaks out for some reason
	return Ok(get_user_of_token(pool, &req.cookie("accessToken").unwrap().value().to_string(), session_expiry_days).await?);
}