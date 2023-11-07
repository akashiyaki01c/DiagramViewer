#![allow(unused)]

use crate::model::diafile::DiaFile;
use ::ron;
use ron::ser::PrettyConfig;


pub fn serialize(data: &DiaFile) -> String {
	let config = PrettyConfig::new()
		.new_line(String::from("\n"))
		.indentor(String::from("\t"))
		.separate_tuple_members(true)
		.struct_names(true);
	ron::ser::to_string_pretty(data, config).unwrap()
}

pub fn deserialize(data: &str) -> DiaFile {
	ron::de::from_str(data).unwrap()
}