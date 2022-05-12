use crate::Row;
use crossterm::Result;
use std::fs;

#[derive(Default)]
pub struct Document {
	rows: Vec<Row>,
}

impl Document {
	pub fn open(file_name: &str) -> Result<Self> {
		let contents = fs::read_to_string(file_name)?;
		let mut rows = Vec::new();

		for value in contents.lines() {
			rows.push(Row::from(value));
		}

		Ok(Self { rows })
	}

	pub fn row(&self, index: usize) -> Option<&Row> {
		self.rows.get(index)
	}

	pub fn is_empty(&self) -> bool {
		self.rows.is_empty()
	}
}
