extern crate actix_web;
extern crate sqlx;
extern crate serde;

use crate::common;
use crate::paymethod;

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct User {
	#[serde(default)]
	id: u32,
	name: String
}

impl User {
	pub(crate) async fn pay_methods(&self, tx: &mut sqlx::Transaction<'_, sqlx::MySql>) -> Result<Vec<paymethod::PayMethod>, sqlx::Error> {
		paymethod::get_by_user(self.id, tx).await
	}
}

#[responder]
pub(crate) async fn create(user: actix_web::web::Json<User>, state: common::State) -> common::ResponderResult<User> /* {{{ */ {
	let mut user = user.into_inner();
	let mut tx = state.db.begin().await?;
	let result = sqlx::query!("INSERT INTO users VALUES (DEFAULT, ?)", &user.name).execute(&mut tx).await?;
	user.id = result.last_insert_id() as u32;
	Ok(json!(user))
} // }}}

#[responder]
pub(crate) async fn get_all(state: common::State) -> common::ResponderResult<Vec<User>> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	let users = sqlx::query_as!(User, "SELECT * FROM users").fetch_all(&mut tx).await?;
	Ok(json!(users))
} // }}}

#[responder]
pub(crate) async fn get_single(id: common::Path<u32>, state: common::State) -> common::ResponderResult<User> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", *id).fetch_one(&mut tx).await?;
	Ok(json!(user))
} // }}}

#[responder]
pub(crate) async fn update(id: common::Path<u32>, user: actix_web::web::Json<User>, state: common::State) -> common::ResponderResult<User> /* {{{ */ {
	let mut user = user.into_inner();
	user.id = *id;
	let mut tx = state.db.begin().await?;
	sqlx::query!("UPDATE users SET name = ? WHERE id = ?", &user.name, &user.id).execute(&mut tx).await?;
	Ok(json!(user))
} // }}}

#[responder]
pub(crate) async fn delete(id: common::Path<u32>, state: common::State) -> common::ResponderResult<bool> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	sqlx::query!("DELETE FROM users WHERE id = ?", *id).execute(&mut tx).await?;
	Ok(json!(true))
} // }}}

