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
#[allow(type_alias_bounds)] // Even if this isn't enforced, I want to express intent explicitly to human readers
pub(crate) type ResponderResult<T: serde::Serialize> = Result<Response<T>, Box<dyn std::error::Error>>;

macro_rules! params {
	($($input: expr),*) => {{
		let mut vector = Vec::new();
		$(vector.push(mysql::Value::from($input));)*
		mysql::Params::Positional(vector)
	}}
}

macro_rules! query {
	($db: expr, $query: literal) => {{
		let db = $db.clone();
		let future = actix_web::web::block(move || {
			db.prep_exec($query, mysql::Params::Empty)
		});
		match future.await {
			Ok(v) => v,
			Err(e) => {
				println!("!!! {}", e);
				return Ok(code!(InternalServerError));
			}
		}
	}};
	($db: expr, $query: literal, $($params: expr),+) => {{
		let db = $db.clone();
		let params = params!($($params),+);
		let future = actix_web::web::block(move || {
			db.prep_exec($query, params)
		});
		match future.await {
			Ok(v) => v,
			Err(e) => {
				println!("!!! {}", e);
				return Ok(code!(InternalServerError));
			}
		}
	}};
}

macro_rules! col {
	($row: ident, $i: literal, $type: ty) => {{
		let result = $row.take::<$type, _>($i);
		if(result.is_none()) {
			return Err(Box::new(common::missing_column_error($row, $i)));
		}
		result.unwrap()
	}}
}

macro_rules! code {
	($code: ident) => { common::Response::Builder(::actix_web::HttpResponse::$code()) }
}

macro_rules! json {
	($val: ident) => { common::Response::Json($val) };
}

pub(crate) fn collect<T: mysql::prelude::FromRow>(result: mysql::QueryResult) -> Vec<T> {
	result.map(|row| {
		let row = row.unwrap();
		mysql::from_row(row)
	}).collect()
}

