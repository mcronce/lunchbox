extern crate actix_web;
extern crate mysql;
extern crate serde;

use crate::common;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
	#[serde(default)]
	id: u32,
	name: String
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

pub(crate) async fn create(user: actix_web::web::Json<User>, state: common::State) -> common::HandlerResult /* {{{ */ {
	db_handler!(state, conn, {
		let mut user = user.into_inner();
		let result = conn.prep_exec("INSERT INTO users VALUES (DEFAULT, ?)", params!(&user.name))?;
		user.id = result.last_insert_id() as u32;
		Ok::<User, mysql::Error>(user)
	})
} // }}}

pub(crate) async fn get_all(state: common::State) -> common::HandlerResult /* {{{ */ {
	db_handler!(state, conn, {
		let result = conn.prep_exec("SELECT * FROM users", ())?;
		let users = result.map(|row| {
			let row = row.unwrap();
			mysql::from_row(row)
		}).collect();
		Ok::<Vec<User>, mysql::Error>(users)
	})
} // }}}

pub(crate) async fn get_single(id: common::Path<u32>, state: common::State) -> common::HandlerResult /* {{{ */ {
	db_handler!(state, conn, {
		let mut result = conn.prep_exec("SELECT * FROM users WHERE id = ?", params!(*id))?;
		let row = result.next().unwrap()?;
		let user = mysql::from_row(row);
		Ok::<User, mysql::Error>(user)
	})
} // }}}

pub(crate) async fn update(id: common::Path<u32>, user: actix_web::web::Json<User>, state: common::State) -> common::HandlerResult /* {{{ */ {
	db_handler!(state, conn, {
		let mut user = user.into_inner();
		user.id = id.into_inner();
		conn.prep_exec("UPDATE users SET name = ? WHERE id = ?", params!(&user.name, &user.id))?;
		Ok::<User, mysql::Error>(user)
	})
} // }}}

pub(crate) async fn delete(id: common::Path<u32>, state: common::State) -> common::HandlerResult /* {{{ */ {
	db_handler!(state, conn, {
		conn.prep_exec("DELETE FROM users WHERE id = ?", params!(*id))?;
		Ok::<bool, mysql::Error>(true)
	})
} // }}}

