//#![feature(async_closure)]
//#![feature(type_alias_impl_trait)]
#![allow(unused_parens)]
extern crate actix_session;
extern crate actix_web;
extern crate env_logger;
extern crate mysql;
extern crate serde;

#[macro_use]
extern crate actix_helper_macros;

use std::error::Error;

#[macro_use]
mod common;
mod env;

mod provider;
mod user;
mod paymethod;

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> /* {{{ */ {
	let threads = match env::get("LUNCHBOX_THREADS") {
		Some(s) => s.parse::<usize>()?,
		None => num_cpus::get()
	};

	let port = match env::get("LUNCHBOX_PORT") {
		Some(s) => s.parse::<u16>()?,
		None => 80
	};

	let pool = {
		let mut builder = mysql::OptsBuilder::new();
		builder.ip_or_hostname(Some(env::get_default("MYSQL_HOST", "localhost")));
		builder.tcp_port(env::get_default("MYSQL_PORT", "3306").parse::<u16>()?);
		builder.db_name(Some(env::get_default("MYSQL_DATABASE", "lunchbox")));
		builder.user(Some(env::get("MYSQL_USERNAME").ok_or("MYSQL_USERNAME is required")?));
		builder.pass(Some(env::get("MYSQL_PASSWORD").ok_or("MYSQL_PASSWORD is required")?));
		mysql::Pool::new_manual(2, threads * 2, builder)?
	};

	let data = common::WebState{
		db: pool
	};

	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();
	let result = actix_web::HttpServer::new(move || actix_web::App::new()
		.data(data.clone())
		.wrap(actix_web::middleware::Logger::default())
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
		)
		.service(actix_files::Files::new("/", "./static").index_file("index.html"))
	).bind(format!("0.0.0.0:{}", port))?.run().await?;
	Ok(result)
} // }}}

