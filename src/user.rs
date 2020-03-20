extern crate actix_web;
extern crate mysql;
extern crate serde;

use crate::common;
use crate::paymethod;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
	#[serde(default)]
	id: u32,
	name: String
}

impl User {
	pub(crate) fn pay_methods(&self, db: &mysql::Pool) -> Result<Vec<paymethod::PayMethod>, mysql::Error> {
		paymethod::get_by_user(self.id, db)
	}
}

impl mysql::prelude::FromRow for User /* {{{ */ {
	fn from_row(row: mysql::Row) -> Self {
		Self::from_row_opt(row).expect("Failed to deserialize User from MySQL row")
	}

	fn from_row_opt(mut row: mysql::Row) -> Result<Self, mysql::FromRowError> {
		if(row.len() != 2) {
			return Err(mysql::FromRowError(row));
		}
		Ok(User{
			id: row.take(0).unwrap(),
			name: row.take(1).unwrap()
		})
	}
} // }}}

#[responder]
pub(crate) async fn create(user: actix_web::web::Json<User>, state: common::State) -> common::ResponderResult<User> /* {{{ */ {
	let mut user = user.into_inner();
	let result = query!(state.db, "INSERT INTO users VALUES (DEFAULT, ?)", &user.name);
	user.id = result.last_insert_id() as u32;
	Ok(json!(user))
} // }}}

#[responder]
pub(crate) async fn get_all(state: common::State) -> common::ResponderResult<Vec<User>> /* {{{ */ {
	let result = query!(state.db, "SELECT * FROM users");
	let users = common::collect(result);
	Ok(json!(users))
} // }}}

#[responder]
pub(crate) async fn get_single(id: common::Path<u32>, state: common::State) -> common::ResponderResult<User> /* {{{ */ {
	let mut result = query!(state.db, "SELECT * FROM users WHERE id = ?", *id);
	let row = result.next().unwrap()?;
	let user = mysql::from_row(row);
	Ok(json!(user))
} // }}}

#[responder]
pub(crate) async fn update(id: common::Path<u32>, user: actix_web::web::Json<User>, state: common::State) -> common::ResponderResult<User> /* {{{ */ {
	let mut user = user.into_inner();
	user.id = *id;
	query!(state.db, "UPDATE users SET name = ? WHERE id = ?", &user.name, &user.id);
	Ok(json!(user))
} // }}}

#[responder]
pub(crate) async fn delete(id: common::Path<u32>, state: common::State) -> common::ResponderResult<bool> /* {{{ */ {
	query!(state.db, "DELETE FROM users WHERE id = ?", *id);
	Ok(json!(true))
} // }}}

