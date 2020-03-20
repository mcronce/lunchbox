extern crate actix_web;
extern crate chrono;
extern crate mysql;
extern crate serde;

use crate::common;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Order {
	#[serde(default)]
	id: u64,
	meal_id: u32,
	user_id: u32,
	paid: u8,
	paid_method: Option<String>
}

impl mysql::prelude::FromRow for Order /* {{{ */ {
	fn from_row(row: mysql::Row) -> Self {
		Self::from_row_opt(row).expect("Failed to deserialize Order from MySQL row")
	}

	fn from_row_opt(mut row: mysql::Row) -> Result<Self, mysql::FromRowError> {
		if(row.len() != 5) {
			return Err(mysql::FromRowError(row));
		}
		Ok(Order{
			id: row.take(0).unwrap(),
			meal_id: row.take(1).unwrap(),
			user_id: row.take(2).unwrap(),
			paid: row.take(3).unwrap(),
			paid_method: row.take(4).unwrap()
		})
	}
} // }}}

async fn get_by_id(id: u64, db: &mysql::Pool) -> Result<Option<Order>, mysql::Error> /* {{{ */ {
	let mut result = db.prep_exec("SELECT * FROM orders WHERE id = ?", params!(id))?;
	let row = match result.next() {
		None => { return Ok(None) }
		Some(r) => r
	}?;
	let order = mysql::from_row(row);
	Ok(Some(order))
} // }}}

async fn get_by_id_and_meal_id(id: u64, meal_id: u32, db: &mysql::Pool) -> Result<Option<Order>, mysql::Error> /* {{{ */ {
	let mut result = db.prep_exec("SELECT * FROM orders WHERE id = ? AND meal_id = ?", params!(id, meal_id))?;
	let row = match result.next() {
		None => { return Ok(None) }
		Some(r) => r
	}?;
	let order = mysql::from_row(row);
	Ok(Some(order))
} // }}}

#[responder]
pub(crate) async fn create(req: actix_web::web::Json<Order>, state: common::State) -> common::ResponderResult<Order> /* {{{ */ {
	let req = req.into_inner();
	let result = query!(state.db, "INSERT INTO orders (meal_id, user_id, paid, paid_method) VALUES (?, ?, ?, ?)", &req.meal_id, &req.user_id, &req.paid, &req.paid_method);
	let order = get_by_id(result.last_insert_id(), &state.db).await?;
	Ok(json!(or_404!(order)))
} // }}}

#[responder]
pub(crate) async fn get_by_meal_id(meal_id: common::Path<u32>, state: common::State) -> common::ResponderResult<Vec<Order>> /* {{{ */ {
	let result = query!(state.db, "SELECT * FROM orders WHERE meal_id = ?", *meal_id);
	let orders = common::collect(result);
	Ok(json!(orders))
} // }}}

#[responder]
pub(crate) async fn get_all(state: common::State) -> common::ResponderResult<Vec<Order>> /* {{{ */ {
	let result = query!(state.db, "SELECT * FROM orders");
	let orders = common::collect(result);
	Ok(json!(orders))
} // }}}

#[responder]
pub(crate) async fn get_single(path: common::Path<(u64, u32)>, state: common::State) -> common::ResponderResult<Order> /* {{{ */ {
	let order = get_by_id_and_meal_id(path.0, path.1, &state.db).await?;
	Ok(json!(or_404!(order)))
} // }}}

#[responder]
pub(crate) async fn update(path: common::Path<(u64, u32)>, order: actix_web::web::Json<Order>, state: common::State) -> common::ResponderResult<Order> /* {{{ */ {
	query!(state.db, "UPDATE orders SET user_id = ?, paid = ?, paid_method = ? WHERE meal_id = ? AND id = ?", &order.user_id, &order.paid, &order.paid_method, &path.1, &path.0);
	let order = get_by_id_and_meal_id(path.0, path.1, &state.db).await?;
	Ok(json!(or_404!(order)))
} // }}}

#[responder]
pub(crate) async fn delete(path: common::Path<(u64, u32)>, state: common::State) -> common::ResponderResult<bool> /* {{{ */ {
	query!(state.db, "DELETE FROM orders WHERE meal_id = ? AND id = ?", &path.0, &path.1);
	Ok(json!(true))
} // }}}

