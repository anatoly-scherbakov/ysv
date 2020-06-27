use csv::{StringRecord, ByteRecord};
use linked_hash_map::LinkedHashMap;
use crate::options::Variables;

#[derive(Debug)]
pub enum Expression {
    Input(usize),
    Slice { start: usize, end: usize },
    Replace { replace: LinkedHashMap<String, String> },
    Variable { name: String },
    Uppercase,
    Lowercase,
    LineNumber,
}


#[derive(Debug)]
pub struct Transformer {
    pub headers: StringRecord,
    pub columns: Vec<Vec<Expression>>,
}


fn safe_to_utf8(bytes: &[u8]) -> String {
    match String::from_utf8(bytes.to_vec()) {
        Ok(value) => value,
        Err(_err) => String::new(),
    }
}


fn replace_with_mapping(value: String, mapping: &LinkedHashMap<String, String>) -> String {
    let mut result: String = value;

    for (from, to) in mapping.iter() {
        result = result.replace(from, to);
    }

    result
}


fn input(row: &ByteRecord, index: &usize) -> Option<String> {
    match row.get(*index) {
        Some(bytes) => Some(safe_to_utf8(bytes)),
        None => None,
    }
}


fn apply_line_number(line_number: u32) -> Option<String> {
    Some(line_number.to_string())
}


impl Expression {
    pub fn apply(&self, value: Option<String>, row: &ByteRecord, variables: &Variables) -> Option<String> {
        match self {
            Expression::Input(index) => input(row, index),
            Expression::Slice { start: _start, end: _end } => match value {
                Some(content) => Some(content),
                None => None,
            },

            Expression::Lowercase => match value {
                Some(content) => Some(content.to_lowercase()),
                None => None,
            },
            Expression::Uppercase => match value {
                Some(content) => Some(content.to_uppercase()),
                None => None,
            },

            Expression::Replace { replace } => match value {
                Some(content) => Some(replace_with_mapping(content, replace)),
                None => None,
            },

            Expression::Variable { name } => match variables.get(name) {
                // This is awfully dirty
                Some(value) => Some(value.clone()),
                None => Some(String::from("")),
            },

            Expression::LineNumber => apply_line_number(0),
        }
    }
}
