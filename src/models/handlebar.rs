use std::collections::HashMap;
use handlebars::{Handlebars};
use serde::Serialize;
use crate::models::struct_fields::StructFields;
use crate::utils::{load_file_as_lines, write_struct_to_file};

#[derive(Serialize)]
pub struct Handlebar {
    pub struct_name: String,
    pub table_name: String,
    pub struct_fields: Vec<String>,
}


impl Handlebar {

    pub fn from(table_name:&str,struct_name:&str,struct_fields: &Vec<StructFields>)->Handlebar {

        Handlebar{
            struct_name:struct_name.to_string(),
            table_name: table_name.to_string(),
            struct_fields: struct_fields.iter().map(|r| r.get_field_type()).collect(),
        }
    }

    pub fn generate_file(&self) {
        let mut handlebars = Handlebars::new();

        let source = load_file_as_lines("src/struct.template").join("\n");
        //let source = "Hello {{ name }}";
        handlebars
            .register_template_string("struct", source)
            .unwrap();



        let mut recs = HashMap::new();
        recs.insert("struct", self);


        let result =handlebars.render("struct", &recs);

        let path = format!("src/generated_structs/{}.rs", self.table_name);
        match result {
            Ok(str) => write_struct_to_file(&path, &str).unwrap(),
            Err(error) => panic!("Error create struct: {:?}", error),
        }

    }


}