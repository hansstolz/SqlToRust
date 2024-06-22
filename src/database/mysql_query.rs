use sqlx::mysql::{MySqlQueryResult, MySqlRow};
use sqlx::{ FromRow, MySql, Pool};
pub struct MySQLQuery<'life> {
    pub pool: &'life Pool<MySql>,
}
impl<'life> MySQLQuery<'life> {

    fn check_result(result: Result<MySqlQueryResult, sqlx::Error>)->bool {
        match result {
            Ok(_) => {
                true
            }
            Err(err) => {
                println!("Error: {:?}", err);
                false
            }
        }
    }
     pub(crate) async fn read_all<T>(&self, sql: &str) -> Vec<T>
    where
        T: Send + Unpin + for<'r> FromRow<'r, MySqlRow>,
    {

        let records = sqlx::query_as::<MySql, T>(sql).fetch_all(self.pool).await;

        match records {
            Ok(records) => {records}
            Err(err) => {println!("Error: {:?}", err); vec![]}
        }


    }

     pub(crate) async fn execute_sql(pool: &Pool<MySql>, sql:&str) ->bool {
        let result =    sqlx::query(sql).execute(pool).await;
         MySQLQuery::check_result(result)
    }

}
