extern crate actix_web;
extern crate mysql;
extern crate r2d2;
extern crate r2d2_mysql;

pub(crate) type Path<T> = actix_web::web::Path<T>;
pub(crate) type DatabasePool = actix_web::web::Data<r2d2::Pool<r2d2_mysql::MysqlConnectionManager>>;
pub(crate) type HandlerResult = Result<actix_web::HttpResponse, actix_web::Error>;

macro_rules! params {
	($($input: expr),*) => {{
		let mut vector = Vec::new();
		$(vector.push(mysql::Value::from($input));)*
		mysql::Params::Positional(vector)
	}}
}

macro_rules! handler {
	($func:block) => {{
		let future = actix_web::web::block(move || $func).await;
		future
			.map(|result| Ok(actix_web::HttpResponse::Ok().json(result)))
			.map_err(|_| actix_web::HttpResponse::InternalServerError())?
	}}
}

macro_rules! db_handler {
	($db:ident, $conn:ident, $func:block) => {
		handler!({
			let mut $conn = $db.get().unwrap();
			$func
		})
	}
}

