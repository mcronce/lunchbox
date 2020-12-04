extern crate actix_web;
extern crate bcrypt;
extern crate sqlx;
extern crate serde;

use actix_web::HttpMessage;

use crate::common;

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Provider {
	id: u32,
	email: String,
	password_hash: String
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuthRequest {
	email: String,
	password: String
}

#[derive(serde::Deserialize)]
pub struct ProviderRequest {
	id: u32,
	email: String,
	password: String,
}

// TODO:  Make this a middleware
async fn check_session(req: &actix_web::web::HttpRequest, tx: &mut sqlx::Transaction<'_, sqlx::MySql>) -> Result<Option<Provider>, sqlx::Error> {
	let cookie = match req.cookie("actix-session") {
		None => { return Ok(None); },
		Some(c) => c
	};

	let result = sqlx::query_as!(
		Provider,
		"SELECT providers.* FROM sessions INNER JOIN providers ON sessions.provider_id = providers.id WHERE cookie = ? AND expiry > NOW()",
		&cookie.value()
	).fetch_optional(tx).await?;
	Ok(result)
}

#[responder]
pub(crate) async fn authorize(auth_request: actix_web::web::Json<AuthRequest>, req: actix_web::web::HttpRequest, state: common::State) -> common::ResponderResult<bool> /* {{{ */ {
	let cookie = match req.cookie("actix-session") {
		None => { return Ok(code!(BadRequest)); },
		Some(c) => c
	};

	let mut tx = state.db.begin().await?;

	let id = {
		let result = sqlx::query!("SELECT password_hash, id FROM providers WHERE email = ?", &auth_request.email).fetch_optional(&mut tx).await?;
		let result = match result {
			Some(v) => v,
			None => return Ok(code!(Unauthorized))
		};
		if(!bcrypt::verify(&auth_request.password, &result.password_hash)?) {
			return Ok(code!(Unauthorized));
		}
		result.id
	};

	sqlx::query!("INSERT INTO sessions VALUES (?, ?, DATE_ADD(NOW(), INTERVAL 1 DAY)) ON DUPLICATE KEY UPDATE provider_id = VALUES(provider_id), expiry = VALUES(expiry)", &cookie.value(), &id).execute(&state.db).await?;
	Ok(json!(true))
} // }}}

#[responder]
pub(crate) async fn create(body: actix_web::web::Json<ProviderRequest>, req: actix_web::web::HttpRequest, state: common::State) -> common::ResponderResult<Provider> {
	let mut tx = state.db.begin().await?;
	if(check_session(&req, &mut tx).await?.is_none()) {
		return Ok(code!(Unauthorized));
	}
	let body = body.into_inner();
	let hash = bcrypt::hash(body.password, bcrypt::DEFAULT_COST)?;
	sqlx::query!("INSERT INTO providers VALUES (?, ?, ?)", &body.id, &body.email, &hash).execute(&mut tx).await?;
	let provider = Provider{
		id: body.id,
		email: body.email,
		password_hash: hash
	};
	Ok(json!(provider))
}

#[responder]
pub(crate) async fn get_all(req: actix_web::web::HttpRequest, state: common::State) -> common::ResponderResult<Vec<Provider>> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	if(check_session(&req, &mut tx).await?.is_none()) {
		return Ok(code!(Unauthorized));
	}
	let providers = sqlx::query_as!(Provider, "SELECT * FROM providers").fetch_all(&mut tx).await?;
	Ok(json!(providers))
} // }}}

