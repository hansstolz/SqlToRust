#![allow(non_snake_case)]

use check_keyword::CheckKeyword;
use sqlx::FromRow;
use crate::database::mysql_query::MySQLQuery;
use crate::models::mysql_types::MySqlType;
use crate::models::struct_fields::StructFields;

#[derive(Clone, FromRow, Debug)]
pub struct TableStructure {
    pub(crate) Field: String,
    pub(crate) Type: String,
    pub(crate) Null: String,
    pub(crate) Key: String,
    pub(crate) Default: Option<String>,
}


impl TableStructure {

    pub(crate) async fn get_table_structure(mysql: &MySQLQuery<'_>, table_name:&str, database:&str) ->Vec<TableStructure> {
        let sql = format!("describe {}.{}",database, table_name);

        let fields: Vec<TableStructure> =mysql.read_all( &sql).await;
        fields
    }


    pub(crate) fn map_table_structure(fields: &Vec<TableStructure>)->Vec<StructFields> {
        let mut rust_fields =vec![];

        for field in fields {
            let is_primary_key = field.Key.to_lowercase()=="pri";
            let nullable = field.Null.to_lowercase() == "yes";
            let mut rust_type=MySqlType::get_map(&field.Type);
            let is_keyword = field.Field.is_keyword();

            if nullable || is_primary_key {
                rust_type = format!("Option<{}>", rust_type);
            }

            let rust_field = StructFields{
                rust_type,
                field_name: field.Field.clone(),
                is_primary_key,
                is_keyword,
                default_value:field.Default.clone().unwrap_or("".to_string())
            };

            rust_fields.push(rust_field);
        }
        rust_fields
    }
}
