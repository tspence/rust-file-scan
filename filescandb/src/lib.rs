extern crate dotenv;
extern crate rusqlite;
extern crate chrono;

use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;
use chrono::prelude::*;
use dotenv::dotenv;
use std::env;

use rusqlite::{ Connection };


#[derive(Debug)]
pub struct FolderModel {
    pub id: i32,
    pub parent_id: i32,
    pub name: String,
}

#[derive(Debug)]
pub struct FileModel {
    pub id: i32,
    pub folder_id: i32,
    pub name: String,
    pub hash: String,
    pub size: u64,
    pub modified_date: String,
}

pub fn establish_connection() -> Connection 
{
    dotenv().ok();

    let db_path = env::var("DATABASE_PATH")
        .expect("DATABASE_PATH must be set");
    let conn = Connection::open(db_path).unwrap();

    // Folders table
    conn.execute_batch("create table if not exists folders (
        id integer primary key not null,
        parent_id integer not null,
        name text not null
    );").unwrap();

    // Files table
    conn.execute_batch("create table if not exists files (
        id integer primary key not null,
        folder_id integer not null,
        name text not null,
        hash text not null,
        size integer not null,
        modified_date text not null
    );").unwrap();

    return conn;
}

pub fn create_folder<'a>(conn: &Connection, name: String, parent_id: i64) 
    -> i64
{
    let r = conn.execute_named(
        "INSERT INTO folders (name, parent_id) VALUES (:name, :parent_id);",
        &[(":name", &name), (":parent_id", &parent_id)],
    );

    match r {
        Ok(_updated) => return conn.last_insert_rowid(),
        Err(err) => {
            println!("Error: {}", err);
            return 0;
        }
    }
}

pub fn create_file<'a>(conn: &Connection, name: String, folder_id: i64, hash: String, size: u64, modified_date: String) 
    -> i64
{
    let size_i64 = size as i64;
    let r = conn.execute_named(
        "INSERT INTO files (name, folder_id, hash, size, modified_date) 
        VALUES (:name, :folder_id, :hash, :size, :modified_date)",
        &[(":name", &name), 
        (":folder_id", &folder_id),
        (":hash", &hash),
        (":size", &size_i64),
        (":modified_date", &modified_date),
        ]
    );

    match r {
        Ok(_updated) => return conn.last_insert_rowid(),
        Err(err) => {
            println!("Error: {}", err);
            return 0;
        }
    }
}

pub fn list_files_in_folder(conn: &Connection, path: String, parent_id: i64) 
{
    // Get a list of all things in this directory
    println!("Scanning: {} ({})", path, parent_id);
    let dirlist = std::fs::read_dir(path).unwrap();

    // Let's go into them
    for entry in dirlist {
        let child_path = entry.unwrap().path();
        let name = child_path.file_name().unwrap().to_str().unwrap().to_string();
        if child_path.is_dir() {

            // Add this directory first
            let folder_id = create_folder(conn, name, parent_id);
            list_files_in_folder(conn, child_path.to_str().unwrap().to_string(), folder_id);
        } else if parent_id != 0 {
            let md = child_path.metadata().unwrap();
            let size = md.len();
            let timestamp = md.modified().unwrap();
            let chrono_datetime_obj: DateTime<Utc> = timestamp.into();
            let chrono_time = chrono_datetime_obj.format("%+").to_string();
            let _file_id = create_file(conn, name, parent_id, "".to_string(), size, chrono_time);
        }
    }
}
