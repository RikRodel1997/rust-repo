mod db;

use db::{Db, DbTypes};
use dotenvy::dotenv;
use std::env;
use std::process::Command;
use std::str;

fn main() {
    dotenv().ok();

    let db = Db {
        db_type: DbTypes::MySql,
        host: env::var("DB_HOST").expect("DB_HOST should be set"),
        port: env::var("DB_PORT")
            .expect("DB_PORT should be set")
            .parse()
            .expect("Unable to parse DB_PORT"),
        user: env::var("DB_USER").expect("DB_USER should be set"),
        password: env::var("DB_PASSWORD").expect("DB_PASSWORD should be set"),
        database: env::var("DB_NAME").expect("DB_NAME should be set"),
    };

    db.get_tables();
}
