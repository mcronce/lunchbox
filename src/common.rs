extern crate actix_web;
extern crate chrono;
extern crate mysql;
extern crate serde;

pub(crate) use actix_helper_macros::*;

mod error;
pub use error::missing_column_error;

#[derive(Clone)]
pub(crate) struct WebState {
	pub(crate) db: mysql::Pool
}

pub(crate) type Path<T> = actix_web::web::Path<T>;
pub(crate) type State = actix_web::web::Data<WebState>;

pub(crate) fn zerotime() -> chrono::NaiveDateTime {
	chrono::NaiveDateTime::from_timestamp(0, 0)
}

