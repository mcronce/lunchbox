extern crate actix_web;
extern crate bcrypt;
extern crate mysql;
extern crate serde;

use actix_web::HttpMessage;

use crate::common;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Provider {
	id: u32,
	email: String,
	password_hash: String
}

impl mysql::prelude::FromRow for Provider /* {{{ */ {
	fn from_row(row: mysql::Row) -> Self {
		Self::from_row_opt(row).expect("Failed to deserialize Provider from MySQL row")
	}

	fn from_row_opt(mut row: mysql::Row) -> Result<Self, mysql::FromRowError> {
		if(row.len() != 3) {
			return Err(mysql::FromRowError(row));
		}
		Ok(Provider{
			id: row.take(0).unwrap(),
			email: row.take(1).unwrap(),
			password_hash: row.take(2).unwrap()
		})
	}
} // }}}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuthRequest {
	email: String,
	password: String
}

#[responder]
pub(crate) async fn authorize(auth_request: actix_web::web::Json<AuthRequest>, req: actix_web::web::HttpRequest, state: common::State) -> common::ResponderResult<bool> /* {{{ */ {
	let cookie = match req.cookie("actix-session") {
		None => { return Ok(code!(BadRequest)); },
		Some(c) => c
	};

	let id = {
		let mut row = {
			let mut result = query!(state.db, "SELECT pass_hash, id FROM providers WHERE email = ?", &auth_request.email);
			match result.next() {
				None => { return Ok(code!(Unauthorized)); }
				Some(r) => r.unwrap()
			}
		};
		let hash = col!(row, 0, String);
		if(!bcrypt::verify(&auth_request.password, &hash)?) {
			return Ok(code!(Unauthorized));
		}
		col!(row, 1, u32)
	};

	query!(state.db, "INSERT INTO sessions VALUES (?, ?, DATE_ADD(NOW(), INTERVAL 1 DAY)) ON DUPLICATE KEY UPDATE provider_id = VALUES(provider_id), expiry = VALUES(expiry)", &cookie.value(), &id);
	Ok(json!(true))
} // }}}

#[responder]
pub(crate) async fn get_all(state: common::State) -> common::ResponderResult<Vec<Provider>> /* {{{ */ {
	let result = query!(state.db, "SELECT * FROM providers");
	let providers: Vec<Provider> = common::collect(result);
	Ok(json!(providers))
} // }}}

