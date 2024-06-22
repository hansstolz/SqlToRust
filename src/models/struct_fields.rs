use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct StructFields {
    pub rust_type: String,
    pub field_name: String,
    pub is_primary_key: bool,
    pub is_keyword: bool,
    pub default_value:String,
}


impl StructFields {
     fn get_field_name(&self)->String {
       if self.is_keyword {
           format!("r#{}", self.field_name)
       } else {
           format!("{}", self.field_name)
       }
    }

    fn get_default_value(&self)->String {
        if self.default_value.is_empty() {
            format!("")
        } else {
            format!("//default value = {}", self.default_value)
        }
    }

    pub fn get_field_type(&self)->String {
        let field = self.get_field_name();
        format!("{} : {}, {} {}", field,  self.rust_type,
                if self.is_primary_key { "//primary key" } else { "" },self.get_default_value() )

    }


}




