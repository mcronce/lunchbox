extern crate actix_web;
extern crate mysql;
extern crate serde;

mod error;
pub use error::missing_column_error;

#[derive(Clone)]
pub(crate) struct WebState {
	pub(crate) db: mysql::Pool
}

pub(crate) enum Response<T: serde::Serialize> {
	Json(T),
	Text(String),
	Builder(actix_web::dev::HttpResponseBuilder)
}

pub(crate) type Path<T> = actix_web::web::Path<T>;
pub(crate) type State = actix_web::web::Data<WebState>;
pub(crate) type ResponderResult<T: serde::Serialize> = Result<Response<T>, Box<dyn std::error::Error>>;

macro_rules! params {
	($($input: expr),*) => {{
		let mut vector = Vec::new();
		$(vector.push(mysql::Value::from($input));)*
		mysql::Params::Positional(vector)
	}}
}

macro_rules! handler {
	($func:block) => {{
		let future = actix_web::web::block(move || $func).await;
		future
			.map(|result| Ok(actix_web::HttpResponse::Ok().json(result)))
			.map_err(|_| actix_web::HttpResponse::InternalServerError())?
	}}
}

macro_rules! db_handler {
	($state:ident, $conn:ident, $func:block) => {
		handler!({
			let mut $conn = $state.db.get_conn().expect("Failed to get MySQL connection from pool");
			$func
		})
	}
}

macro_rules! code {
	($code: ident) => { common::Response::Builder(::actix_web::HttpResponse::$code()) }
}

