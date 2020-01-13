use std::env;

#[inline]
pub fn get_default(key: &str, default: &str) -> String {
	match env::var_os(key) {
		Some(val) => match val.into_string() {
			Ok(val) => val,
			Err(e) => panic!(e)
		},
		None => default.to_string()
	}
}

#[inline]
pub fn get(key: &str) -> Option<String> {
	match env::var_os(key) {
		Some(val) => Some(match val.into_string() {
			Ok(val) => val,
			Err(e) => panic!(e)
		}),
		None => None
	}
}

