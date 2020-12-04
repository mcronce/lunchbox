extern crate actix_web;
extern crate chrono;
extern crate sqlx;
extern crate serde;

pub(crate) use actix_helper_macros::*;

#[derive(Clone)]
pub(crate) struct WebState {
	pub(crate) db: sqlx::mysql::MySqlPool
}

pub(crate) type Path<T> = actix_web::web::Path<T>;
pub(crate) type State = actix_web::web::Data<WebState>;

pub(crate) fn zerotime() -> chrono::NaiveDateTime {
	chrono::NaiveDateTime::from_timestamp(0, 0)
}

