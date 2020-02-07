extern crate actix_web;
extern crate mysql;
extern crate serde;

use crate::common;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PayMethod {
	#[serde(default)]
	user_id: u32,
	#[serde(default)]
	method: String,
	method_info: String
}

impl mysql::prelude::FromRow for PayMethod /* {{{ */ {
	fn from_row(row: mysql::Row) -> Self {
		Self::from_row_opt(row).expect("Failed to deserialize PayMethod from MySQL row")
	}

	fn from_row_opt(mut row: mysql::Row) -> Result<Self, mysql::FromRowError> {
		if(row.len() != 3) {
			return Err(mysql::FromRowError(row));
		}
		Ok(PayMethod{
			user_id: row.take(0).unwrap(),
			method: row.take(1).unwrap(),
			method_info: row.take(2).unwrap()
		})
	}
} // }}}

#[responder]
pub(crate) async fn create(user_id: common::Path<u32>, method: actix_web::web::Json<PayMethod>, state: common::State) -> common::ResponderResult<PayMethod> /* {{{ */ {
	let method = method.into_inner();
	query!(state.db, "INSERT INTO users_paymethods VALUES (?, ?, ?)", *user_id, &method.method, &method.method_info);
	Ok(json!(method))
} // }}}

pub(crate) fn get_by_user(user_id: u32, db: &mysql::Pool) -> Result<Vec<PayMethod>, mysql::Error> /* {{{ */ {
	let result = db.prep_exec("SELECT * FROM users_paymethods WHERE user_id = ?", params!(user_id))?;
	let methods = common::collect(result);
	Ok(methods)
} // }}}

#[responder]
pub(crate) async fn get_by_user_id(user_id: common::Path<u32>, state: common::State) -> common::ResponderResult<Vec<PayMethod>> /* {{{ */ {
	let methods = get_by_user(*user_id, &state.db)?;
	Ok(json!(methods))
} // }}}

#[responder]
pub(crate) async fn get_all(state: common::State) -> common::ResponderResult<Vec<String>> /* {{{ */ {
	let result = query!(state.db, "SELECT DISTINCT method FROM users_paymethods");
	let methods = common::collect(result);
	Ok(json!(methods))
} // }}}

#[responder]
pub(crate) async fn update(path: common::Path<(u32, String)>, method: actix_web::web::Json<PayMethod>, state: common::State) -> common::ResponderResult<PayMethod> /* {{{ */ {
	let mut method = method.into_inner();
	method.user_id = path.0;
	method.method = path.1.to_string();
	query!(state.db, "UPDATE users_paymethods SET method_info = ? WHERE user_id = ? AND method = ?", &method.method_info, &method.user_id, &method.method);
	Ok(json!(method))
} // }}}

#[responder]
pub(crate) async fn delete(path: common::Path<(u32, String)>, state: common::State) -> common::ResponderResult<bool> /* {{{ */ {
	query!(state.db, "DELETE FROM users_paymethods WHERE user_id = ? AND method = ?", &path.0, &path.1);
	Ok(json!(true))
} // }}}

