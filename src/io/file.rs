#![allow(unused)]

use crate::model::diafile::DiaFile;
use ::ron;
use ron::{ser::PrettyConfig, error::SpannedError};

#[derive(Debug)]
pub enum DiaFileParseError {
	ParseError(String, (usize, usize)),
}

pub fn serialize(data: &DiaFile) -> String {
	let config = PrettyConfig::new()
		.new_line(String::from("\n"))
		.indentor(String::from("\t"))
		.separate_tuple_members(true)
		.struct_names(true);
	ron::ser::to_string_pretty(data, config).unwrap()
}

pub fn deserialize(data: &str) -> Result<DiaFile, DiaFileParseError> {
	let dia_file = ron::de::from_str(data);
	if dia_file.is_err() {
		let err: SpannedError = dia_file.unwrap_err();
		return Err(DiaFileParseError::ParseError(err.to_string(), (err.position.line, err.position.col)))
	}
	Ok(dia_file.unwrap())
}