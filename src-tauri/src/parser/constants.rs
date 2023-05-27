use std::{collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::logger::logger;

use super::tokens::Token;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Constant {
    Binary(i64),
    Integer(i64),
    Float(f64),
    Hexadecimal(i64),
    Octal(i64),
    #[allow(unused)]
    None,
}

#[derive(Deserialize, Serialize)]
pub enum Target {
    #[allow(unused)]
    Integer = 0,
    #[allow(unused)]
    Float = 1,
    #[allow(unused)]
    Binary = 2,
    #[allow(unused)]
    Hexadecimal = 3,
    #[allow(unused)]
    Octal = 4,
}

//custom deserializer for Constant to serialize hex, binary, and octal numbers from string to int
impl<'de> Deserialize<'de> for Constant {
    fn deserialize<D>(deserializer: D) -> Result<Constant, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut map = HashMap::<&str, String>::deserialize(deserializer)?;
        let mut constant = Constant::None;
        for (key, value) in map.drain() {
            match key {
                "Binary" => {
                    let s = value.trim_start_matches("0b");
                    let i = i64::from_str_radix(s, 2).unwrap();
                    constant = Constant::Binary(i);
                },
                "Integer" => {
                    let i = value.parse::<i64>().unwrap();
                    constant = Constant::Integer(i);
                },
                "Float" => {
                    let f = value.parse::<f64>().unwrap();
                    constant = Constant::Float(f);
                },
                "Hexadecimal" => {
                    let s = value.trim_start_matches("0x");
                    let i = i64::from_str_radix(s, 16).unwrap();
                    constant = Constant::Hexadecimal(i);
                },
                "Octal" => {
                    let s = value.trim_start_matches("0o");
                    let i = i64::from_str_radix(s, 8).unwrap();
                    constant = Constant::Octal(i);
                },
                "None" => {
                    constant = Constant::None;
                },
                _ => {
                    logger::error!("Error: Unknown key: {}", key);
                    constant = Constant::None;
                },
            }
        }
        Ok(constant)
    }
}

impl Serialize for Constant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = HashMap::<&str, String>::new();
        match self {
            Constant::Binary(i) => {
                let s = format!("0b{:b}", i);
                map.insert("Binary", s);
            },
            Constant::Integer(i) => {
                let s = format!("{}", i);
                map.insert("Integer", s);
            },
            Constant::Float(f) => {
                let s = format!("{:.}", f);
                map.insert("Float", s);
            },
            Constant::Hexadecimal(i) => {
                let s = format!("0x{:x}", i);
                map.insert("Hexadecimal", s);
            },
            Constant::Octal(i) => {
                let s = format!("0o{:o}", i);
                map.insert("Octal", s);
            },
            Constant::None => {
                let s = format!("None");
                map.insert("None", s);
            },
        }
        map.serialize(serializer)
    }
}

impl Constant {
    pub fn from_tok(t: Token) -> Self {
        match t {
            Token::Binary(i) => Self::Binary(i),
            Token::Integer(i) => Self::Integer(i),
            Token::Float(f) => Self::Float(f),
            Token::Hexadecimal(i) => Self::Hexadecimal(i),
            Token::Octal(i) => Self::Octal(i),
            _ => {
                logger::error!("Could not convert Token into Constant");
                Self::None
            },
        }
    }
    pub fn into_target_constant(self, target: Target) -> Constant {
        match target {
            Target::Integer => {
                let i = self.into();
                Constant::Integer(i)
            },
            Target::Float => {
                let f = self.into();
                Constant::Float(f)
            },
            Target::Binary => {
                let i = self.into();
                Constant::Binary(i)
            },
            Target::Hexadecimal => {
                let i = self.into();
                Constant::Hexadecimal(i)
            },
            Target::Octal => {
                let i = self.into();
                Constant::Octal(i)
            },
        }
    }
}

impl Into<i64> for Constant {
    fn into(self) -> i64 {
        match self {
            Self::Binary(i) => i,
            Self::Integer(i) => i,
            Self::Float(f) => {
                let address = &f as *const f64 as *const i64;
                unsafe { *address }
            },
            Self::Hexadecimal(i) => i,
            Self::Octal(i) => i,
            _ => {
                logger::error!("Could not convert Constant into i64");
                0
            },
        }
    }
}

impl Into<f64> for Constant {
    fn into(self) -> f64 {
        match self {
            Self::Binary(i) => i as f64,
            Self::Integer(i) => i as f64,
            Self::Float(f) => f,
            Self::Hexadecimal(i) => i as f64,
            Self::Octal(i) => i as f64,
            _ => {
                logger::error!("Could not convert Constant into f64");
                0.0
            },
        }
    }
}