extern crate dotenv;
extern crate rusqlite;
extern crate chrono;

pub mod models;
pub mod filescan;
pub mod context;

use rusqlite::{ Connection };

pub fn establish_connection<'a>(filename: String)
    -> context::FileScanDbContext<'a>
{
    let conn = Connection::open(&filename).unwrap();

    // clean tables
    conn.execute_batch("drop table if exists files;").unwrap();
    conn.execute_batch("drop table if exists folders;").unwrap();

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

    // Open a transaction
    let folder_stmt = conn.prepare_cached("INSERT INTO folders (name, parent_folder_id) VALUES (:name, :parent_folder_id);").unwrap();
    let file_stmt = conn.prepare_cached("INSERT INTO files (name, parent_folder_id, hash, size, modified_date) 
        VALUES (:name, :parent_folder_id, :hash, :size, :modified_date)").unwrap();

    return context::FileScanDbContext {
        file_name: filename,
        conn: conn,
        tx: None,
        stmt_insert_file: file_stmt,
        stmt_insert_folder: folder_stmt,
    };
}

pub fn write_to_database(ctxt: &mut context::FileScanDbContext, folder: &mut models::FolderModel) 
    -> ()
{
    ctxt.tx = Some(ctxt.conn.transaction().unwrap()); 

    // Prepare statements and insert sql as fast as possible
    {
        internal_write(ctxt, folder);
    }

    // Commit the transaction
    let _r = ctxt.tx.unwrap().commit().unwrap();
    ctxt.tx = None;
}

pub fn internal_write(ctxt: &mut context::FileScanDbContext, folder: &mut models::FolderModel)
    -> ()
{
    // Insert this folder
    let id = create_folder(ctxt, folder);
    folder.id = id;

    // Insert all files within this folder
    for mut child_file in &mut folder.files {
        child_file.parent_folder_id = id;
        create_file(ctxt, child_file);
    }

    // Insert all child folders
    for mut child_folder in &mut folder.folders {
        child_folder.parent_folder_id = id;
        internal_write(ctxt, &mut child_folder);
    }
}


pub fn create_folder<'a>(ctxt: &mut context::FileScanDbContext, folder: &models::FolderModel)
    -> i64
{
    let r = ctxt.stmt_insert_folder.execute_named(&[(":name", &folder.name), (":parent_folder_id", &folder.parent_folder_id)]);

    match r {
        Ok(_updated) => return ctxt.tx.unwrap().last_insert_rowid(),
        Err(err) => {
            println!("Error: {}", err);
            return 0;
        }
    }
}

pub fn create_file<'a>(ctxt: &mut context::FileScanDbContext, file: &models::FileModel)
    -> i64
{
    let size_i64 = file.size as i64;
    let r = ctxt.stmt_insert_file.execute_named(
        &[(":name", &file.name), 
        (":parent_folder_id", &file.parent_folder_id),
        (":hash", &file.hash),
        (":size", &size_i64),
        (":modified_date", &file.modified_date),
        ]
    );

    match r {
        Ok(_updated) => return ctxt.tx.unwrap().last_insert_rowid(),
        Err(err) => {
            println!("Error: {}", err);
            return 0;
        }
    }
}

