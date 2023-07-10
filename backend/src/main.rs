#![deny(
  clippy::pedantic,
)]

#![allow(
  clippy::needless_return,
  clippy::unnecessary_unwrap,
  clippy::wildcard_imports,
  clippy::module_name_repetitions,
  clippy::bool_to_int_with_if,
  clippy::cast_sign_loss,
  clippy::cast_possible_wrap,
  clippy::cast_possible_truncation,
  clippy::similar_names,
  deprecated,
)]

#![feature(async_fn_in_trait)]

mod webserver;
mod access_token;
mod user;
mod config;
mod postgres;
mod currency;
mod account;
mod tag;
mod recipient;
mod transaction;
mod asset;
mod dashboard;
mod chart;

use std::fmt;
use std::error::Error;

#[cfg(test)]
use deadpool_postgres::Pool;

use config::*;
use webserver::initialize_webserver;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let config = initialize_config();

  let pool = postgres::get_connection(&config).await;

  user::init(&config, &pool).await;
  initialize_webserver(config, pool).await?;
  
  return Ok(());
}

#[derive(Debug, Clone)]
enum CustomError {
  NoItemFound {
    item_type: String
  },
  SpecifiedItemNotFound {
    item_type: String,
    filter: String
  },
  InvalidItem {
    reason: String
  },
  InvalidCredentials,
  MissingCookie {
    cookie: String
  },
  MissingProperty {
    property: String,
    item_type: String
  }
}

impl fmt::Display for CustomError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return match self {
      CustomError::NoItemFound {item_type} => write!(f, "no item of type {item_type} found"),
      CustomError::SpecifiedItemNotFound {item_type, filter} => write!(f, "specified item of type {item_type} not found with filter {filter}"),
      CustomError::InvalidItem {reason} => write!(f, "the given item is invalid, because {reason}"),
      CustomError::InvalidCredentials => write!(f, "the given credentials are invalid"),
      CustomError::MissingCookie {cookie} => write!(f, "cookie {cookie} not set"),
      CustomError::MissingProperty {property, item_type} => write!(f, "Missing property {property} on type {item_type}"),
    }
  }
}

impl Error for CustomError {

}

#[cfg(test)]
async fn setup() -> (Config, Pool) {
  let config = initialize_config();
  postgres::delete_testing_databases(&config).await;
  let pool = postgres::get_connection(&config).await;
  user::init(&config, &pool).await;
  return (config, pool);
}

#[cfg(test)]
async fn teardown(config: &Config) {
  postgres::delete_database(&config).await;
}