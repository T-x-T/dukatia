use actix_web::{dev::Service as _, web, App, HttpServer, middleware};
use futures_util::future::FutureExt;
use deadpool_postgres::Pool;
use super::config::Config;
use super::user;
use super::currency;
use super::account;
use super::tag;
use super::recipient;
use super::transaction;

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
			.service(account::rest_api::get_all)
			.service(account::rest_api::post)
			.service(account::rest_api::put)
			.service(tag::rest_api::get_all)
			.service(tag::rest_api::post)
			.service(tag::rest_api::put)
			.service(recipient::rest_api::get_all)
			.service(recipient::rest_api::post)
			.service(recipient::rest_api::put)
			.service(transaction::rest_api::get_all)
			.service(transaction::rest_api::post)
			.service(transaction::rest_api::put)
			.service(transaction::rest_api::delete)
			.service(currency::rest_api::get_all);
	})
		.bind(("0.0.0.0", api_port))?
		.run()
		.await;
}