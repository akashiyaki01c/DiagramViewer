#![allow(unused)]

use crate::model::dia::diafile::DiaFile;
use ::ron;
use ron::{ser::PrettyConfig, error::SpannedError};

#[derive(Debug)]
pub enum DiaFileParseError {
	ParseError(String, (usize, usize)),
	CompressionError(String),
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
	if let Err(err) = dia_file {
		return Err(DiaFileParseError::ParseError(err.to_string(), (err.position.line, err.position.col)))
	}
	Ok(dia_file.unwrap())
}

pub fn serialize_comp(data: &DiaFile) -> Vec<u8> {
	zstd::encode_all(serialize(data).as_bytes(), 3).expect("Compressed Error")
}

pub fn deserialize_comp(data: &[u8]) -> Result<DiaFile, DiaFileParseError>{
	let bytes = zstd::decode_all(data);
	if bytes.is_err() {
		return Err(DiaFileParseError::CompressionError(String::from("Decompressed Error")))
	}
	deserialize(&String::from_utf8(zstd::decode_all(bytes.unwrap().as_slice()).unwrap()).unwrap())
}