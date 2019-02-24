#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel::expression::sql_literal::sql;
use dotenv::dotenv;
use std::env;

table! {
    files (id) {
        id -> Integer,
        folder_id -> Integer,
        name -> Text,
        hash -> Text,
        size -> Integer,
        modified_date -> Text,
    }
}

table! {
    folders (id) {
        id -> Integer,
        parent_id -> Integer,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    files,
    folders,
);

#[derive(Queryable)]
pub struct FolderModel {
    pub id: i32,
    pub parent_id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct FileModel {
    pub id: i32,
    pub folder_id: i32,
    pub name: String,
    pub hash: String,
    pub size: i32,
    pub modified_date: String,
}

#[derive(Insertable)]
#[table_name="folders"]
pub struct NewFolder<'a> {
    pub name: &'a str,
    pub parent_id: &'a i32,
}

#[derive(Insertable)]
#[table_name="files"]
pub struct NewFile<'a> {
    pub name: &'a str,
    pub folder_id: &'a i32,
    pub hash: &'a str,
    pub size: &'a i32,
    pub modified_date: &'a str,
}


static g_next_file_id: Option<i32> = None;
static g_next_folder_id: Option<i32> = None;


pub fn get_next_file_id(conn: &SqliteConnection) 
{
    if (g_next_file_id == None) {
        let g_next_file_id = sql("select max(id) from files;")
            .get_result(&conn)
            .expect("Error executing raw SQL")
            + 1;
    }

    // Increment
    let num = g_next_file_id.unwrap();
    g_next_file_id = num + 1;
    return num;
}

pub fn get_next_folder_id(conn: &SqliteConnection) 
{
    if (g_next_folder_id == None) {
        let g_next_folder_id = sql("select max(id) from folders;")
            .get_result(&conn)
            .expect("Error executing raw SQL")
            + 1;
    }

    // Increment
    let num = g_next_folder_id.unwrap();
    g_next_folder_id = num + 1;
    return num;
}

pub fn establish_connection() -> SqliteConnection 
{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_folder<'a>(conn: &SqliteConnection, name: &'a str, parent_id: i32) -> i32 
{

    let new_folder = NewFolder {
        id: get_next_folder_id(conn),
        name: name,
        parent_id: &parent_id,
    };

    let r = diesel::insert_into(folders::table)
        .values(&new_folder)
        .execute(conn)
        .unwrap();

    return new_folder.id;
}

pub fn create_file<'a>(conn: &SqliteConnection, 
    name: &'a str, 
    folder_id: i32, 
    hash: &'a str,
    size: i32,
    modified_date: &'a str,) -> i32 
{

    let new_file = NewFile {
        id: get_next_file_id(conn),
        name: name,
        folder_id: &folder_id,
        hash: hash,
        size: &size,
        modified_date: modified_date,
    };

    let r = diesel::insert_into(files::table)
        .values(&new_file)
        .execute(conn)
        .unwrap();

    return new_file.id;
}

pub fn list_files_in_folder(conn: &SqliteConnection, path: String, parent_id: i32) 
{

    // Get a list of all things in this directory
    println!("Scanning: {} ({})", path, parent_id);
    let dirlist = std::fs::read_dir(path).unwrap();

    // Let's go into them
    for entry in dirlist {
        let child_path = entry.unwrap().path();
        let name = child_path.file_name().unwrap().to_str().unwrap();
        if child_path.is_dir() {

            // Add this directory first
            let folder_id = create_folder(conn, name, parent_id);
            list_files_in_folder(conn, child_path.to_str().unwrap().to_string(), folder_id);
        } else if parent_id != 0 {
            let md = child_path.metadata().unwrap();
            let file_id = create_file(conn, name, parent_id, "", 0, "");
            println!("Name: {} ({})", child_path.to_str().unwrap().to_string(), file_id);
        }
    }
}
