use std::fmt::Formatter;

use regex::Regex;
use serde::{Deserialize, Serialize, Serializer, de::{Visitor, Error}, Deserializer};

pub mod diagram;
pub mod line;
pub mod root;
pub mod route;
pub mod station;
pub mod train;
pub mod train_type;
pub mod settings;

/// 時刻を表す構造体
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Time(pub usize);

impl Time {
    pub fn new(hour: usize, minute: usize, second: usize) -> Self {
        Self(hour * 3600 + minute * 60 + second)
    }

    pub fn hour(&self) -> usize {
        self.0 / 3600
    }
    pub fn minute(&self) -> usize {
        self.0 / 60 % 60
    }
    pub fn second(&self) -> usize {
        self.0 % 60
    }
}

impl ToString for Time {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour(), self.minute(), self.second())
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        serializer.serialize_str(&format!("{:02}:{:02}:{:02}", self.hour(), self.minute(), self.second()))
    }
}

struct TimeVisitor;

impl<'de> Visitor<'de> for TimeVisitor {
    type Value = Time;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }


    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error, {
        if v.len() != 8 {
            return Err(Error::custom("長さが8以外です。"))
        }
        let regex = Regex::new(r"(\d\d):(\d\d):(\d\d)").unwrap();
        if !regex.is_match(v) {
            return Err(Error::custom(""))
        }
        let caps = regex.captures(v).unwrap();
        let mut time: usize = 0;
        println!("{:?}", caps);
        for i in 0..caps.len() {
            match i {
                1 => time += caps.get(1).unwrap().as_str().parse::<usize>().unwrap() * 3600,
                2 => time += caps.get(2).unwrap().as_str().parse::<usize>().unwrap() * 60,
                3 => time += caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                _ => {}
            }
        }
        Ok(Time(time))
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        deserializer.deserialize_str(TimeVisitor)
    }
}