//#![feature(async_closure)]
//#![feature(type_alias_impl_trait)]
#![allow(unused_parens)]
extern crate actix_web;
extern crate env_logger;
extern crate mysql;
extern crate r2d2;
extern crate r2d2_mysql;
extern crate serde;

use std::error::Error;

#[macro_use]
mod common;
mod env;

mod user;

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
		let manager = r2d2_mysql::MysqlConnectionManager::new(builder);
		r2d2::Pool::builder().max_size(threads as u32 * 2).build(manager)?
	};

	std::env::set_var("RUST_LOG", "actix_web=info");
	env_logger::init();
	let result = actix_web::HttpServer::new(move || {
		actix_web::App::new()
			.data(pool.clone())
			.wrap(actix_web::middleware::Logger::default())
			.route("/users", actix_web::web::post().to(user::create))
			.route("/users", actix_web::web::get().to(user::get_all))
			.route("/users/{id}", actix_web::web::get().to(user::get_single))
			.route("/users/{id}", actix_web::web::post().to(user::update))
			.route("/users/{id}", actix_web::web::delete().to(user::delete))
	}).bind(format!("0.0.0.0:{}", port))?.run().await?;
	Ok(result)
} // }}}
