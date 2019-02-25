#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;

use diesel::prelude::*;
use diesel::insert_into;
use dotenv::dotenv;
use std::env;

use models;
use schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    Connection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_folder<'a>(conn: &SqliteConnection, name: &'a str, parent_id: i32) -> i32 {

    let new_folder = NewFolder {
        name: name,
        parent_id: &parent_id,
    };

    diesel::insert_into(schema::folders::table)
        .values(&new_folder)
        .execute(conn);
    return 0;
}

pub fn create_file<'a>(conn: &SqliteConnection, 
    name: &'a str, 
    folder_id: i32, 
    hash: &'a str,
    size: i32,
    modified_date: &'a str,) -> i32 {

    let new_file = NewFile {
        name: name,
        folder_id: &folder_id,
        hash: hash,
        size: &size,
        modified_date: modified_date,
    };

    diesel::insert_into(schema::files::table)
        .values(&new_file)
        .execute(conn);
    return 0;
}

pub fn list_files_in_folder(conn: &SqliteConnection, path: String, parent_id: i32) {

    // Get a list of all things in this directory
    let dirlist = std::fs::read_dir(path).unwrap();

    // Let's go into them
    for entry in dirlist {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_str().unwrap();
        if path.is_dir() {

            // Add this directory first
            let this_folder_id = create_folder(conn, name, parent_id);
            list_files_in_folder(conn, path.to_str().unwrap().to_string(), this_folder_id);
        } else if parent_id != 0 {
            let _md = path.metadata().unwrap();
            let _file = create_file(conn, name, parent_id, "", 0, "");
            println!("Name: {}", path.to_str().unwrap().to_string());
        }
    }
}

