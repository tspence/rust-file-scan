#[macro_use]

extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_folder<'a>(conn: &PgConnection, name: &'a str, parent_id: &'a i32) -> FolderModel {
    use schema::folders;

    let new_folder = NewFolder<'a> {
        name: name,
        parent_id: parent_id,
    };

    diesel::insert_into(folders::table)
        .values(&new_folder)
        .get_result(conn)
        .expect("Error saving new folder")
}

pub fn create_file<'a>(conn: &PgConnection, 
    name: &'a str, 
    folder_id: &'a i32, 
    hash: &'a str,
    size: &'a i64,
    modified_date: &'a str,) -> FileModel {

    use schema::files;

    let new_file = NewFile<'a> {
        name: name,
        folder_id: folder_id,
        hash: hash,
        size: size,
        modified_date: modified_date,
    };

    diesel::insert_into(files::table)
        .values(&new_file)
        .get_result(conn)
        .expect("Error saving new file")
}
