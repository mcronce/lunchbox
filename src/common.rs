extern crate actix_web;
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

pub(crate) fn collect<T: mysql::prelude::FromRow>(result: mysql::QueryResult) -> Vec<T> {
	result.map(|row| {
		let row = row.unwrap();
		mysql::from_row(row)
	}).collect()
}

