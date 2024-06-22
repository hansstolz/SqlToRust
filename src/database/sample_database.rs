use sqlx::{MySql, Pool};
use crate::database::mysql_query::MySQLQuery;

pub const DATABASE_NAME: &str = "dbmaster";

const CREATE_TABLE_SQL: &str = r#"
    CREATE TABLE if not exists `dbmaster`.`demo_table` (
  `uid` INT NOT NULL AUTO_INCREMENT,
  `var_char` VARCHAR(45) NOT NULL  DEFAULT 'Hello',
  `date_time` DATETIME NOT NULL,
  `decimal` DECIMAL(4,2) NULL,
  `float` FLOAT NULL,
  `long_text` LONGTEXT NULL,
  `tiny_int` TINYINT NULL,
  `blob` BLOB NULL,
   `type` TEXT NOT NULL,
  PRIMARY KEY (`uid`));
"#;

const CREATE_DATABASE_SQL: &str = r#"
    CREATE DATABASE if not exists  `dbmaster` ;
"#;

pub struct SampleDatabase {}


impl SampleDatabase {
     pub async fn create_database( db: &Pool<MySql>) ->bool {
         MySQLQuery::execute_sql(db, CREATE_DATABASE_SQL).await
    }

     async fn create_table(db: &Pool<MySql>)->bool {
         MySQLQuery::execute_sql(db, CREATE_TABLE_SQL).await
    }

    pub async fn prepare(db:&Pool<MySql>)->bool {
        return SampleDatabase::create_database(db).await &&  SampleDatabase::create_table(db).await;
    }
}