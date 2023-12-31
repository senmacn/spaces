use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::fs;
use std::path::Path;

use crate::utils::path::full_path;

use super::procedures::{CREATE_TABLE_PROJECT_ITEM, CREATE_TABLE_START_SCHEME};

pub fn init_db() -> SqliteConnection {
    if !db_file_exists() {
        create_db_file();
    }

    let mut conn = get_connection();
    db_init_tables(&mut conn);
    conn
}

pub fn get_connection() -> SqliteConnection {
    establish_connection()
}

fn establish_connection() -> SqliteConnection {
    let db_path = "sqlite://".to_string() + get_db_path().as_str();

    SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}

// Create the database file.
fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    // Create the database file.
    fs::File::create(db_path).unwrap();
}

// Check whether the database file exists.
fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

fn db_init_tables(conn: &mut SqliteConnection) {
    diesel::sql_query(CREATE_TABLE_PROJECT_ITEM)
        .execute(conn)
        .expect("Error executing create project_item table SQL query");
    diesel::sql_query(CREATE_TABLE_START_SCHEME)
        .execute(conn)
        .expect("Error executing create table start_scheme SQL query");
}

fn get_db_path() -> String {
    String::from(full_path("database.sqlite").to_str().unwrap())
}
