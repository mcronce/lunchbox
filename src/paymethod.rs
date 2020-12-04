extern crate actix_web;
extern crate sqlx;
extern crate serde;

use crate::common;

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct PayMethod {
	#[serde(default)]
	user_id: u32,
	#[serde(default)]
	method: String,
	method_info: String
}

#[responder]
pub(crate) async fn create(user_id: common::Path<u32>, method: actix_web::web::Json<PayMethod>, state: common::State) -> common::ResponderResult<PayMethod> /* {{{ */ {
	let method = method.into_inner();
	let mut tx = state.db.begin().await?;
	sqlx::query!("INSERT INTO users_paymethods VALUES (?, ?, ?)", *user_id, &method.method, &method.method_info).execute(&mut tx).await?;
	Ok(json!(method))
} // }}}

pub(crate) async fn get_by_user(user_id: u32, tx: &mut sqlx::Transaction<'_, sqlx::MySql>) -> Result<Vec<PayMethod>, sqlx::Error> /* {{{ */ {
	let methods = sqlx::query_as!(PayMethod, "SELECT * FROM users_paymethods WHERE user_id = ?", user_id).fetch_all(tx).await?;
	Ok(methods)
} // }}}

#[responder]
pub(crate) async fn get_by_user_id(user_id: common::Path<u32>, state: common::State) -> common::ResponderResult<Vec<PayMethod>> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	let methods = get_by_user(*user_id, &mut tx).await?;
	Ok(json!(methods))
} // }}}

#[responder]
pub(crate) async fn get_all(state: common::State) -> common::ResponderResult<Vec<String>> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	let methods = sqlx::query!("SELECT DISTINCT method FROM users_paymethods").fetch_all(&mut tx).await?.into_iter().map(|r| r.method).collect();
	Ok(json!(methods))
} // }}}

#[responder]
pub(crate) async fn update(path: common::Path<(u32, String)>, method: actix_web::web::Json<PayMethod>, state: common::State) -> common::ResponderResult<PayMethod> /* {{{ */ {
	let mut method = method.into_inner();
	method.user_id = path.0.0;
	method.method = path.0.1.to_string();
	let mut tx = state.db.begin().await?;
	sqlx::query!("UPDATE users_paymethods SET method_info = ? WHERE user_id = ? AND method = ?", &method.method_info, &method.user_id, &method.method).execute(&mut tx).await?;
	Ok(json!(method))
} // }}}

#[responder]
pub(crate) async fn delete(path: common::Path<(u32, String)>, state: common::State) -> common::ResponderResult<bool> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	sqlx::query!("DELETE FROM users_paymethods WHERE user_id = ? AND method = ?", &path.0.0, &path.0.1).execute(&mut tx).await?;
	Ok(json!(true))
} // }}}

