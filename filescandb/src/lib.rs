extern crate dotenv;
extern crate rusqlite;
extern crate chrono;

use dotenv::dotenv;
use std::env;

use rusqlite::{ Connection, Transaction };

#[derive(Debug)]
pub struct FileModel {
    pub id: i64,
    pub parent_folder_id: i64,
    pub name: String,
    pub hash: String,
    pub size: u64,
    pub modified_date: String,
}

#[derive(Debug)]
pub struct FolderModel {
    pub id: i64,
    pub parent_folder_id: i64,
    pub name: String,

    pub folders: Vec<FolderModel>,
    pub files: Vec<FileModel>,
}

pub fn total_items(f: &FolderModel) 
    -> usize
{
    let mut count = 1 + f.files.len();
    for child in &(f.folders) {
        count = count + total_items(&child);
    }
    return count;
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
        parent_folder_id integer not null,
        name text not null
    );").unwrap();

    // Files table
    conn.execute_batch("create table if not exists files (
        id integer primary key not null,
        parent_folder_id integer not null,
        name text not null,
        hash text not null,
        size integer not null,
        modified_date text not null
    );").unwrap();

    return conn;
}

pub fn write(folder: &mut FolderModel) 
    -> ()
{
    let mut conn = establish_connection();
    let mut tx = conn.transaction().unwrap();

    internal_write(&mut tx, folder);

    let _r = tx.commit().unwrap();
}

pub fn internal_write(conn: &mut Transaction, folder: &mut FolderModel)
    -> ()
{
    // Insert this folder
    let id = create_folder(conn, folder.name.clone(), folder.parent_folder_id);
    folder.id = id;

    // Insert all files within this folder
    for mut child_file in &mut folder.files {
        child_file.parent_folder_id = id;
        create_file(conn, child_file.name.clone(), id, child_file.hash.clone(), child_file.size, child_file.modified_date.clone());
    }

    // Insert all child folders
    for mut child_folder in &mut folder.folders {
        child_folder.parent_folder_id = id;
        internal_write(conn, &mut child_folder);
    }
}


pub fn create_folder<'a>(conn: &mut Transaction, name: String, parent_folder_id: i64) 
    -> i64
{
    let mut stmt = conn.prepare_cached("INSERT INTO folders (name, parent_folder_id) VALUES (:name, :parent_folder_id);").unwrap();
    let r = stmt.execute_named(&[(":name", &name), (":parent_folder_id", &parent_folder_id)]);

    match r {
        Ok(_updated) => return conn.last_insert_rowid(),
        Err(err) => {
            println!("Error: {}", err);
            return 0;
        }
    }
}

pub fn create_file<'a>(conn: &Connection, name: String, parent_folder_id: i64, hash: String, size: u64, modified_date: String) 
    -> i64
{
    let mut stmt = conn.prepare_cached("INSERT INTO files (name, parent_folder_id, hash, size, modified_date) 
        VALUES (:name, :parent_folder_id, :hash, :size, :modified_date)").unwrap();
    let size_i64 = size as i64;
    let r = stmt.execute_named(
        &[(":name", &name), 
        (":parent_folder_id", &parent_folder_id),
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

pub fn list_files_in_folder(path: String) 
    -> Result<FolderModel, String>
{
    // Start our result
    let mut parent_folder = FolderModel {
        id: 0,
        parent_folder_id: 0,
        name: path.to_string(),
        folders: Vec::<FolderModel>::new(),
        files: Vec::<FileModel>::new()
    };

    // Get a list of all things in this directory
    //try!(let mut dirlist = std::fs::read_dir(path).map_err(|e| e.to_string()));
    let try_dirlist = std::fs::read_dir(path);
    match try_dirlist {
        Err(e) => return Err(e.to_string()),
        Ok(dirlist) =>

            // Let's go into all children
            for tryentry in dirlist {

                // Figure out path and name
                let entry = tryentry.unwrap();
                let file_type = entry.file_type().unwrap();
                let child_path = entry.path();
                let name = child_path.file_name().unwrap().to_str().unwrap().to_string();

                // If it's a directory, it goes in one place
                if file_type.is_dir() {
                    let child_folder = list_files_in_folder(child_path.to_str().unwrap().to_string());
                    match child_folder {
                        Err(e) => return Err(e),
                        Ok(val) => parent_folder.folders.push(val),
                    }

                // Otherwise capture files
                } else if file_type.is_file() {
                    let trymd = child_path.metadata();
                    match trymd {
                        Err(e) => println!("Cannot observe metadata for {}: {}", child_path.to_str().unwrap(), e.to_string()),
                        Ok(md) => {
                            let size = md.len();
                            //let timestamp = md.modified().unwrap();
                            //let chrono_datetime_obj: DateTime<Utc> = timestamp.into();
                            //let chrono_time = chrono_datetime_obj.format("%+").to_string();
                            let file = FileModel {
                                id: 0,
                                parent_folder_id: 0,
                                name: name,
                                hash: "".to_string(),
                                size: size,
                                modified_date: "".to_string()//chrono_time,
                            };
                            parent_folder.files.push(file);

                        }
                    }
                }
            },
    }


    // Here's the folder
    return Ok(parent_folder);
}
