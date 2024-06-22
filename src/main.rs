mod tables_to_structs;
mod models;
mod database;
mod utils;

use sqlx::MySqlPool;
use crate::database::get_tables;
use crate::database::mysql_query::MySQLQuery;
use crate::database::sample_database::{DATABASE_NAME, SampleDatabase};
use crate::tables_to_structs::{TableToStructs};

#[tokio::main]
async fn main() {
    dotenv_flow::dotenv_flow().ok();

    let key_db = "DATABASE_URL";
    let database_url = dotenv_flow::var(key_db).expect("DATABASE_URL must be set");

    let pool = MySqlPool::connect(&database_url).await;

    match pool {
        Ok(pool) => {
            let success = SampleDatabase::prepare(&pool).await;
            if success {
                println!("Database prepared");
                let tables=get_tables::get_tables_in(&pool,DATABASE_NAME).await;
                let mysql = MySQLQuery{pool:&pool};
                let table_structure = TableToStructs{ db_mysql:&mysql};
                table_structure.convert(&tables, DATABASE_NAME).await;
            } else {
                println!("Database not prepared");
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }

}
