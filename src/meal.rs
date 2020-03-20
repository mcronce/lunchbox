extern crate actix_web;
extern crate chrono;
extern crate mysql;
extern crate serde;

use crate::common;

#[derive(serde::Serialize, serde::Deserialize)]
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

impl mysql::prelude::FromRow for Meal /* {{{ */ {
	fn from_row(row: mysql::Row) -> Self {
		Self::from_row_opt(row).expect("Failed to deserialize Meal from MySQL row")
	}

	fn from_row_opt(mut row: mysql::Row) -> Result<Self, mysql::FromRowError> {
		if(row.len() != 8) {
			return Err(mysql::FromRowError(row));
		}
		Ok(Meal{
			id: row.take(0).unwrap(),
			user_id: row.take(1).unwrap(),
			restaurant: row.take(2).unwrap(),
			opened: row.take(3).unwrap(),
			closed: row.take(4).unwrap(),
			ordered: row.take(5).unwrap(),
			acquired: row.take(6).unwrap(),
			delivered: row.take(7).unwrap()
		})
	}
} // }}}

async fn get_by_id(id: u32, db: &mysql::Pool) -> Result<Option<Meal>, mysql::Error> /* {{{ */ {
	let mut result = db.prep_exec("SELECT * FROM meals WHERE id = ?", params!(id))?;
	let row = match result.next() {
		None => { return Ok(None) }
		Some(r) => r
	}?;
	let meal = mysql::from_row(row);
	Ok(Some(meal))
} // }}}

#[responder]
pub(crate) async fn create(req: actix_web::web::Json<MealRequest>, state: common::State) -> common::ResponderResult<Meal> /* {{{ */ {
	let req = req.into_inner();
	let result = query!(state.db, "INSERT INTO meals (user_id, restaurant, opened) VALUES (?, ?, NOW())", &req.user_id, &req.restaurant);
	let meal = get_by_id(result.last_insert_id() as u32, &state.db).await?;
	Ok(json!(or_404!(meal)))
} // }}}

#[responder]
pub(crate) async fn get_all(state: common::State) -> common::ResponderResult<Vec<Meal>> /* {{{ */ {
	let result = query!(state.db, "SELECT * FROM meals");
	let meals = common::collect(result);
	Ok(json!(meals))
} // }}}

#[responder]
pub(crate) async fn get_single(id: common::Path<u32>, state: common::State) -> common::ResponderResult<Meal> /* {{{ */ {
	let meal = get_by_id(*id, &state.db).await?;
	Ok(json!(or_404!(meal)))
} // }}}

#[responder]
pub(crate) async fn update(id: common::Path<u32>, req: actix_web::web::Json<MealRequest>, state: common::State) -> common::ResponderResult<Meal> /* {{{ */ {
	query!(state.db, "UPDATE meals SET restaurant = ? WHERE id = ?", &req.restaurant, *id);
	let meal = get_by_id(*id, &state.db).await?;
	Ok(json!(or_404!(meal)))
} // }}}

// TODO:  Require these to be in the correct state before even issuing the UPDATE query; 400 if not correct
#[responder]
pub(crate) async fn close(id: common::Path<u32>, state: common::State) -> common::ResponderResult<Meal> /* {{{ */ {
	query!(state.db, "UPDATE meals SET closed = NOW() WHERE id = ? AND closed IS NULL", *id);
	let meal = get_by_id(*id, &state.db).await?;
	Ok(json!(or_404!(meal)))
} // }}}

#[responder]
pub(crate) async fn order(id: common::Path<u32>, state: common::State) -> common::ResponderResult<Meal> /* {{{ */ {
	query!(state.db, "UPDATE meals SET ordered = NOW() WHERE id = ? AND ordered IS NULL AND closed IS NOT NULL", *id);
	let meal = get_by_id(*id, &state.db).await?;
	Ok(json!(or_404!(meal)))
} // }}}

#[responder]
pub(crate) async fn acquire(id: common::Path<u32>, state: common::State) -> common::ResponderResult<Meal> /* {{{ */ {
	query!(state.db, "UPDATE meals SET acquired = NOW() WHERE id = ? AND acquired IS NULL AND ordered IS NOT NULL", *id);
	let meal = get_by_id(*id, &state.db).await?;
	Ok(json!(or_404!(meal)))
} // }}}

#[responder]
pub(crate) async fn deliver(id: common::Path<u32>, state: common::State) -> common::ResponderResult<Meal> /* {{{ */ {
	query!(state.db, "UPDATE meals SET delivered = NOW() WHERE id = ? AND delivered IS NULL AND acquired IS NOT NULL", *id);
	let meal = get_by_id(*id, &state.db).await?;
	Ok(json!(or_404!(meal)))
} // }}}

#[responder]
pub(crate) async fn delete(id: common::Path<u32>, state: common::State) -> common::ResponderResult<bool> /* {{{ */ {
	query!(state.db, "DELETE FROM meals WHERE id = ?", &*id);
	Ok(json!(true))
} // }}}


