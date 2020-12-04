//#![feature(async_closure)]
//#![feature(type_alias_impl_trait)]
#![allow(unused_parens)]
extern crate actix_session;
extern crate actix_web;
extern crate env_logger;
extern crate sqlx;
extern crate serde;

#[macro_use]
extern crate actix_helper_macros;

use std::error::Error;

#[macro_use]
mod common;

mod provider;
mod user;
mod paymethod;
mod meal;
mod order;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> /* {{{ */ {
	let port = match std::env::var("LUNCHBOX_PORT") {
		Ok(s) => s.parse::<u16>()?,
		Err(std::env::VarError::NotPresent) => 80,
		Err(std::env::VarError::NotUnicode(s)) => panic!("LUNCHBOX_PORT was not unicode: {:?}", s)
	};

	let pool = {
		let url = std::env::var("DATABASE_URL").unwrap();
		sqlx::pool::Pool::connect(&url).await?
	};

	let data = common::WebState{
		db: pool
	};

	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();
	let result = actix_web::HttpServer::new(move || actix_web::App::new()
		.data(data.clone())
		.wrap(actix_web::middleware::Logger::default())
		.wrap(actix_web::middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
		.wrap(actix_session::CookieSession::private(&[0; 32]).secure(false)) // TODO: Real key
		.service(actix_web::web::scope("/api")
			.route("/authorize", actix_web::web::post().to(provider::authorize))
			.service(actix_web::web::scope("/provider")
				.route("/providers", actix_web::web::get().to(provider::get_all))
				.route("/providers", actix_web::web::post().to(provider::create))
			)
			.route("/users", actix_web::web::post().to(user::create))
			.route("/users", actix_web::web::get().to(user::get_all))
			.route("/users/{id}", actix_web::web::get().to(user::get_single))
			.route("/users/{id}", actix_web::web::post().to(user::update))
			.route("/users/{id}", actix_web::web::delete().to(user::delete))
			.route("/paymethods", actix_web::web::get().to(paymethod::get_all))
			.route("/users/{id}/paymethods", actix_web::web::post().to(paymethod::create))
			.route("/users/{id}/paymethods", actix_web::web::get().to(paymethod::get_by_user_id))
			.route("/users/{id}/paymethods/{name}", actix_web::web::post().to(paymethod::update))
			.route("/users/{id}/paymethods/{name}", actix_web::web::delete().to(paymethod::delete))
			.route("/meals", actix_web::web::post().to(meal::create))
			.route("/meals", actix_web::web::get().to(meal::get_all))
			.route("/meals/{id}", actix_web::web::get().to(meal::get_single))
			.route("/meals/{id}", actix_web::web::post().to(meal::update))
			.route("/meals/{id}", actix_web::web::delete().to(meal::delete))
			.route("/meals/{id}/close", actix_web::web::get().to(meal::close))
			.route("/meals/{id}/order", actix_web::web::get().to(meal::order))
			.route("/meals/{id}/acquire", actix_web::web::get().to(meal::acquire))
			.route("/meals/{id}/deliver", actix_web::web::get().to(meal::deliver))
			.route("/meals/{id}/orders", actix_web::web::post().to(order::create))
			.route("/meals/{id}/orders", actix_web::web::get().to(order::get_by_meal_id))
			.route("/orders", actix_web::web::get().to(order::get_all))
			.route("/meals/{meal_id}/orders/{id}", actix_web::web::get().to(order::get_single))
			.route("/meals/{meal_id}/orders/{id}", actix_web::web::post().to(order::update))
			.route("/meals/{meal_id}/orders/{id}", actix_web::web::delete().to(order::delete))
		)
		.service(actix_files::Files::new("/", "./static").index_file("index.html"))
	).bind(format!("0.0.0.0:{}", port))?.run().await?;
	Ok(result)
} // }}}

