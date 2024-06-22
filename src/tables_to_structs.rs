
use crate::database::mysql_query::MySQLQuery;
use crate::models::handlebar::Handlebar;
use crate::models::table_structure::TableStructure;
use crate::utils::table_name_to_struct_name;

pub struct TableToStructs<'life> {
    pub(crate) db_mysql: &'life MySQLQuery<'life>
}

impl<'life> TableToStructs<'life> {
    pub async fn convert(&self, tables: &Vec<String>,database:&str) {
        for table in tables {
            let fields = TableStructure::get_table_structure(self.db_mysql, table, database).await;
            let struct_fields = TableStructure::map_table_structure(&fields);
            let struct_name = table_name_to_struct_name(table);
            let handlebar = Handlebar::from(table, &struct_name, &struct_fields);
            handlebar.generate_file();

        }
    }
}

