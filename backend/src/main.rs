#![allow(
  clippy::needless_return,
  clippy::needless_borrow,
  clippy::or_fun_call,
  clippy::redundant_field_names,
)]
#![feature(btree_drain_filter)]
#![feature(drain_filter)]

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
      CustomError::NoItemFound {item_type} => write!(f, "no item of type {} found", item_type),
      CustomError::SpecifiedItemNotFound {item_type, filter} => write!(f, "specified item of type {} not found with filter {}", item_type, filter),
      CustomError::InvalidItem {reason} => write!(f, "the given item is invalid, because {}", reason),
      CustomError::InvalidCredentials => write!(f, "the given credentials are invalid"),
      CustomError::MissingCookie {cookie} => write!(f, "cookie {} not set", cookie),
      CustomError::MissingProperty {property, item_type} => write!(f, "Missing property {} on type {}", property, item_type),
    }
  }
}

impl Error for CustomError {

}

#[allow(dead_code)]
async fn setup() -> (Config, Pool) {
  let config = initialize_config();
  postgres::delete_testing_databases(&config).await;
  let pool = postgres::get_connection(&config).await;
  user::init(&config, &pool).await;
  return (config, pool);
}

#[allow(dead_code)]
async fn teardown(config: &Config) {
  postgres::delete_database(&config).await;
}