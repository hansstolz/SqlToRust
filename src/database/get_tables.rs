use sqlx::{MySql, Pool, Row};

pub async fn get_tables_in(db: &Pool<MySql>, schema:&str) -> Vec<String> {
    let sql = format!("select table_name as table_name from information_schema.tables where (table_type = 'BASE TABLE' or table_type='VIEW') and table_schema ='{}' order by table_name", schema);

    let result = sqlx::query(&sql)
        .fetch_all(db)
        .await;

    match result {
        Ok(result) => result.iter().map(|row| row.get("table_name")).collect(),
        Err(e) => {
            println!("Error: {:?}", e);
            return vec![];
        }
    }
}