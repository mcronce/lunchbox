extern crate actix_web;
extern crate chrono;
extern crate sqlx;
extern crate serde;

use crate::common;

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Order {
	#[serde(default)]
	id: u64,
	meal_id: u32,
	user_id: u32,
	paid: u8,
	paid_method: Option<String>
}

async fn get_by_id(id: u64, tx: &mut sqlx::Transaction<'_, sqlx::MySql>) -> Result<Option<Order>, sqlx::Error> /* {{{ */ {
	let order = sqlx::query_as!(Order, "SELECT * FROM orders WHERE id = ?", id).fetch_optional(tx).await?;
	Ok(order)
} // }}}

async fn get_by_id_and_meal_id(id: u64, meal_id: u32, tx: &mut sqlx::Transaction<'_, sqlx::MySql>) -> Result<Option<Order>, sqlx::Error> /* {{{ */ {
	let order = sqlx::query_as!(Order, "SELECT * FROM orders WHERE id = ? AND meal_id = ?", &id, &meal_id).fetch_optional(tx).await?;
	Ok(order)
} // }}}

#[responder]
pub(crate) async fn create(req: actix_web::web::Json<Order>, state: common::State) -> common::ResponderResult<Order> /* {{{ */ {
	let req = req.into_inner();
	let mut tx = state.db.begin().await?;
	let result = sqlx::query!("INSERT INTO orders (meal_id, user_id, paid, paid_method) VALUES (?, ?, ?, ?)", &req.meal_id, &req.user_id, &req.paid, &req.paid_method).execute(&mut tx).await?;
	let order = get_by_id(result.last_insert_id(), &mut tx).await?;
	Ok(json!(or_404!(order)))
} // }}}

#[responder]
pub(crate) async fn get_by_meal_id(meal_id: common::Path<u32>, state: common::State) -> common::ResponderResult<Vec<Order>> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	let orders = sqlx::query_as!(Order, "SELECT * FROM orders WHERE meal_id = ?", *meal_id).fetch_all(&mut tx).await?;
	Ok(json!(orders))
} // }}}

#[responder]
pub(crate) async fn get_all(state: common::State) -> common::ResponderResult<Vec<Order>> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	let orders = sqlx::query_as!(Order, "SELECT * FROM orders").fetch_all(&mut tx).await?;
	Ok(json!(orders))
} // }}}

#[responder]
pub(crate) async fn get_single(path: common::Path<(u64, u32)>, state: common::State) -> common::ResponderResult<Order> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	let order = get_by_id_and_meal_id(path.0.0, path.0.1, &mut tx).await?;
	Ok(json!(or_404!(order)))
} // }}}

#[responder]
pub(crate) async fn update(path: common::Path<(u64, u32)>, order: actix_web::web::Json<Order>, state: common::State) -> common::ResponderResult<Order> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	sqlx::query!("UPDATE orders SET user_id = ?, paid = ?, paid_method = ? WHERE meal_id = ? AND id = ?", &order.user_id, &order.paid, &order.paid_method, &path.0.1, &path.0.0).execute(&mut tx).await?;
	let order = get_by_id_and_meal_id(path.0.0, path.0.1, &mut tx).await?;
	Ok(json!(or_404!(order)))
} // }}}

#[responder]
pub(crate) async fn delete(path: common::Path<(u64, u32)>, state: common::State) -> common::ResponderResult<bool> /* {{{ */ {
	let mut tx = state.db.begin().await?;
	sqlx::query!("DELETE FROM orders WHERE meal_id = ? AND id = ?", &path.0.0, &path.0.1).execute(&mut tx).await?;
	Ok(json!(true))
} // }}}

