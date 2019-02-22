#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{NewFolder, FolderModel, NewFile, FileModel};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_folder<'a>(conn: &PgConnection, name: &'a str, parent_id: i32) -> FolderModel {

    let new_folder = NewFolder {
        name: name,
        parent_id: &parent_id,
    };

    diesel::insert_into(schema::folders::table)
        .values(&new_folder)
        .get_result(conn)
        .expect("Error saving new folder")
}

pub fn create_file<'a>(conn: &PgConnection, 
    name: &'a str, 
    folder_id: i32, 
    hash: &'a str,
    size: i32,
    modified_date: &'a str,) -> FileModel {

    let new_file = NewFile {
        name: name,
        folder_id: &folder_id,
        hash: hash,
        size: &size,
        modified_date: modified_date,
    };

    diesel::insert_into(schema::files::table)
        .values(&new_file)
        .get_result(conn)
        .expect("Error saving new file")
}

pub fn list_files_in_folder(conn: &PgConnection, path: String, parent_id: i32) {

    // Get a list of all things in this directory
    let dirlist = std::fs::read_dir(path).unwrap();

    // Let's go into them
    for entry in dirlist {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_str().unwrap();
        if path.is_dir() {

            // Add this directory first
            let folder = create_folder(conn, name, parent_id);
            list_files_in_folder(conn, path.to_str().unwrap().to_string(), folder.id);
        } else if parent_id != 0 {
            let md = path.metadata().unwrap();
            let file = create_file(conn, name, parent_id, "", 0, "");
            println!("Name: {}", path.to_str().unwrap().to_string());
        }
    }
}

