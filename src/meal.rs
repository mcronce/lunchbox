extern crate actix_web;
extern crate chrono;
extern crate sqlx;
extern crate serde;

use crate::common;

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Meal {
	#[serde(default)]
	id: u32,
	user_id: u32,
	restaurant: String,
	#[serde(default = "common::zerotime")]
	opened: chrono::NaiveDateTime,
	closed: Option<chrono::NaiveDateTime>,
	ordered: Option<chrono::NaiveDateTime>,
	acquired: Option<chrono::NaiveDateTime>,
	delivered: Option<chrono::NaiveDateTime>
}

#[derive(serde::Deserialize)]
pub struct MealRequest {
	#[serde(default)]
	user_id: u32,
	restaurant: String
}

async fn get_by_id(id: u32, tx: &mut sqlx::Transaction<'_, sqlx::MySql>) -> Result<Option<Meal>, sqlx::Error> /* {{{ */ {
	let meal = sqlx::query_as!(Meal, "SELECT * FROM meals WHERE id = ?", &id).fetch_optional(tx).await?;
	Ok(meal)
} // }}}

#[responder]
pub(crate) async fn create(req: actix_web::web::Json<MealRequest>, state: common::State) -> common::ResponderResult<Meal> /* {{{ */ {
	let req = req.into_inner();
	let mut tx = state.db.begin().await?;
	let result = sqlx::query!("INSERT INTO meals (user_id, restaurant, opened) VALUES (?, ?, NOW())", &req.user_id, &req.restaurant).execute(&mut tx).await?;
	let meal = get_by_id(result.last_insert_id() as u32, &mut tx).await?;
	Ok(json!(or_404!(meal)))
} // }}}

#[responder]
pub(crate) async fn get_all(state: common::State) -> common::ResponderResult<Vec<Meal>> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	let meals = sqlx::query_as!(Meal, "SELECT * FROM meals").fetch_all(&mut tx).await?;
	Ok(json!(meals))
} // }}}

#[responder]
pub(crate) async fn get_single(id: common::Path<u32>, state: common::State) -> common::ResponderResult<Meal> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	let meal = get_by_id(*id, &mut tx).await?;
	Ok(json!(or_404!(meal)))
} // }}}

#[responder]
pub(crate) async fn update(id: common::Path<u32>, req: actix_web::web::Json<MealRequest>, state: common::State) -> common::ResponderResult<Meal> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	sqlx::query!("UPDATE meals SET restaurant = ? WHERE id = ?", &req.restaurant, *id).execute(&mut tx).await?;
	let meal = get_by_id(*id, &mut tx).await?;
	Ok(json!(or_404!(meal)))
} // }}}

// TODO:  Require these to be in the correct state before even issuing the UPDATE query; 400 if not correct
#[responder]
pub(crate) async fn close(id: common::Path<u32>, state: common::State) -> common::ResponderResult<Meal> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	sqlx::query!("UPDATE meals SET closed = NOW() WHERE id = ? AND closed IS NULL", *id).execute(&mut tx).await?;
	let meal = get_by_id(*id, &mut tx).await?;
	Ok(json!(or_404!(meal)))
} // }}}

#[responder]
pub(crate) async fn order(id: common::Path<u32>, state: common::State) -> common::ResponderResult<Meal> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	sqlx::query!("UPDATE meals SET ordered = NOW() WHERE id = ? AND ordered IS NULL AND closed IS NOT NULL", *id).execute(&mut tx).await?;
	let meal = get_by_id(*id, &mut tx).await?;
	Ok(json!(or_404!(meal)))
} // }}}

#[responder]
pub(crate) async fn acquire(id: common::Path<u32>, state: common::State) -> common::ResponderResult<Meal> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	sqlx::query!("UPDATE meals SET acquired = NOW() WHERE id = ? AND acquired IS NULL AND ordered IS NOT NULL", *id).execute(&mut tx).await?;
	let meal = get_by_id(*id, &mut tx).await?;
	Ok(json!(or_404!(meal)))
} // }}}

#[responder]
pub(crate) async fn deliver(id: common::Path<u32>, state: common::State) -> common::ResponderResult<Meal> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	sqlx::query!("UPDATE meals SET delivered = NOW() WHERE id = ? AND delivered IS NULL AND acquired IS NOT NULL", *id).execute(&mut tx).await?;
	let meal = get_by_id(*id, &mut tx).await?;
	Ok(json!(or_404!(meal)))
} // }}}

#[responder]
pub(crate) async fn delete(id: common::Path<u32>, state: common::State) -> common::ResponderResult<bool> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	sqlx::query!("DELETE FROM meals WHERE id = ?", &*id).execute(&mut tx).await?;
	Ok(json!(true))
} // }}}


