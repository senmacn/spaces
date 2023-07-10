use diesel::{prelude::*, sql_query};
use diesel::sqlite::SqliteConnection;
use std::fs;
use std::path::Path;

use crate::utils::path::full_path;

diesel::table! {
    project_item (uuid) {
        uuid -> VarChar,
        name -> VarChar,
        description -> VarChar,
    }
}

struct User ();

// Check if a database file exists, and create one if it does not.
pub fn init_db() -> SqliteConnection {
    if !db_file_exists() {
        create_db_file();
    }
    
    let mut conn = get_connection();
    let a =sql_query("Create Table `test` IF NOT EXISTS ").load::<User>(&mut conn);
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

// Get the path where the database file should be located.
fn get_db_path() -> String {
    String::from(full_path("database.sqlite").to_str().unwrap())
}
