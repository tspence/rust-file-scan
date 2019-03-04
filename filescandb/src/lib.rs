extern crate lazy_static;
extern crate dotenv;
extern crate rusqlite;
extern crate chrono;

use rusqlite::{ Connection, };
use chrono::*;

pub mod models;
pub mod context;



pub fn write_folder_nested(folder: &mut models::FolderModel) 
    -> ()
{
    let conn = Connection::open("rustfilescan.db").unwrap();
    let mut ctxt = context::RustFileScanDbContext::new(&conn);
    conn.execute_batch("BEGIN TRANSACTION;").unwrap();

    internal_write(&mut ctxt, folder);

    // Commit the transaction
    conn.execute_batch("COMMIT TRANSACTION;").unwrap();
}

pub fn internal_write(ctxt: &mut context::RustFileScanDbContext, folder: &mut models::FolderModel)
    -> ()
{
    // Insert this folder
    let id = ctxt.create_folder(folder).unwrap();

    // Insert all files within this folder
    for mut child_file in &mut folder.files {
        child_file.parent_folder_id = id;
        ctxt.create_file(child_file).unwrap();
    }

    // Insert all child folders
    for mut child_folder in &mut folder.folders {
        child_folder.parent_folder_id = id;
        internal_write(ctxt, child_folder);
    }
}

pub fn list_files_in_folder(path: String) 
    -> Result<models::FolderModel, std::io::Error>
{
    // Start our result
    let mut parent_folder = models::FolderModel {
        id: 0,
        parent_folder_id: 0,
        name: path.to_string(),
        folders: Vec::<models::FolderModel>::new(),
        files: Vec::<models::FileModel>::new(),
    };

    // Get a list of all things in this directory
    let dirlist = std::fs::read_dir(path)?;

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
                Err(e) => println!("Cannot observe folder {}: {}", child_path.to_str().unwrap(), e.to_string()),
                Ok(val) => parent_folder.folders.push(val),
            }

        // Otherwise capture files
        } else if file_type.is_file() {
            let trymd = child_path.metadata();
            match trymd {
                Err(e) => println!("Cannot observe metadata for {}: {}", child_path.to_str().unwrap(), e.to_string()),
                Ok(md) => {
                    let size = md.len();
                    let timestamp = md.modified().unwrap();
                    let chrono_time: DateTime<Utc> = timestamp.into();
                    let file = models::FileModel {
                        id: 0,
                        parent_folder_id: 0,
                        name: name,
                        hash: "".to_string(),
                        size: size,
                        modified_date: chrono_time.to_rfc3339(),
                    };
                    parent_folder.files.push(file);
                }
            }
        } else {
            //println!("Cannot observe {}: Neither file nor directory.", name);
        }
    }


    // Here's the folder
    return Ok(parent_folder);
}
