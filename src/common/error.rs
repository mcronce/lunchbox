extern crate mysql;

use std::error::Error;

// MissingColumnError {{{
#[derive(Debug)]
pub struct MissingColumnError {
	source: mysql::Row,
	index: usize
}

impl std::fmt::Display for MissingColumnError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Unable to read column {} from result", self.index)
	}
}

impl std::error::Error for MissingColumnError {
	fn description(&self) -> &str {
		"Missing the requested column number"
	}

	fn source(&self) -> Option<&(dyn Error + 'static)> {
		None
	}
}

pub fn missing_column_error(source: mysql::Row, index: usize) -> MissingColumnError {
	MissingColumnError{source, index}
}
// }}}

