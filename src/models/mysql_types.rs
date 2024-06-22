use std::str::FromStr;
use strum_macros::EnumString;
use crate::utils::remove_size;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, EnumString)]
pub enum MySqlType {
    tinyint,
    smallint,
    mediumint,
    int,
    bigint,
    float,
    double,
    decimal,
    date,
    datetime,
    time,
    timestamp,
    year,
    char,
    varchar,
    binary,
    varbinary,
    tinyblob,
    blob,
    mediumblob,
    longblob,
    tinytext,
    text,
    mediumtext,
    longtext,
    unknown
}

impl MySqlType {
    pub fn map_to_rust(&self)->String {
        let  rust_type:&str;

        match self {
            MySqlType::tinyint => rust_type="i8",
            MySqlType::smallint => rust_type="i16",
            MySqlType::mediumint => rust_type="i32",
            MySqlType::int => rust_type="i32",
            MySqlType::bigint => rust_type="i64",
            MySqlType::float => rust_type="f32",
            MySqlType::double => rust_type="f64",
            MySqlType::decimal => rust_type="f64",
            MySqlType::date => rust_type="NaiveDate",
            MySqlType::datetime => rust_type="NaiveDateTime",
            MySqlType::time => rust_type="NaiveTime",
            MySqlType::timestamp => rust_type="NaiveDateTime",
            MySqlType::year => rust_type="i32",
            MySqlType::char => rust_type="String",
            MySqlType::varchar => rust_type="String",
            MySqlType::binary => rust_type="Vec<u8>",
            MySqlType::varbinary => rust_type="Vec<u8>",
            MySqlType::tinyblob => rust_type="Vec<u8>",
            MySqlType::blob => rust_type="Vec<u8>",
            MySqlType::mediumblob => rust_type="Vec<u8>",
            MySqlType::longblob => rust_type="Vec<u8>",
            MySqlType::tinytext => rust_type="String",
            MySqlType::text => rust_type="String",
            MySqlType::mediumtext => rust_type="String",
            MySqlType::longtext => rust_type="String",
            MySqlType::unknown => rust_type="unknown",
        }

        rust_type.to_string()
    }

    pub fn get_map(sql_type:&str) -> String {
        let norm_type = remove_size(sql_type).to_lowercase();
        if let Some(a_type) = MySqlType::from_str(&norm_type).ok() {
            return  a_type.map_to_rust()
        }
        return "unknown".to_string()
    }
}