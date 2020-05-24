use std::io;
use csv::{ByteRecord, StringRecord, Reader, Writer};
use std::collections::BTreeMap;

use crate::config::{Config, Column, create_transformer};

#[derive(Debug)]
pub enum Expression {
    Input(usize),
}


#[derive(Debug)]
pub struct Transformer {
    columns: Vec<Vec<Expression>>,
}


fn get_input_columns_index_map(headers: &StringRecord) -> BTreeMap<String, usize> {
    // FIXME awful function, I do not know the proper method yet
    let mut mapping = BTreeMap::new();

    for (index, value) in headers.iter().enumerate() {
        mapping.insert(String::from(value), index);
    }

    mapping
}


fn transform(record: ByteRecord, config: &Config, headers: &StringRecord) -> ByteRecord {
    for (name, column) in &config.columns {
        eprintln!("{} -> {:?}", name, column);
    }
    record
}


pub fn process(config: Config) {
    eprintln!("Using config: {:#?}", config);

    let mut reader = Reader::from_reader(io::stdin());
    let mut writer = Writer::from_writer(io::stdout());

    let headers = reader.headers().expect("Where are my headers?!").clone();
    writer.write_record(&headers).expect("woo");

    let transformer = create_transformer(&config, &headers);
    eprintln!("{:?}", transformer);

    for result in reader.byte_records() {
        match result {
            Ok(record) => writer.write_record(&transform(
                record, &config, &headers,
            )).expect("boo!"),
            Err(err) => {
                eprintln!("ERROR reading CSV from <stdin>: {}", err);
            }
        };
    }

    writer.flush().expect("cannot flush!");
}