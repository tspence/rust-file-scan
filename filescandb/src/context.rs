extern crate rusqlite;

use rusqlite::{ Connection, Statement };

use crate::models;

pub struct RustFileScanDbContext<'a> {
    pub conn: &'a Connection,
    pub insert_folder_stmt: Statement<'a>,
    pub insert_file_stmt: Statement<'a>,
}

impl<'a> RustFileScanDbContext<'a>
{
    pub fn new(conn: &'a Connection) -> Self
    {
        // Construct statements for performance reasons
        let insert_folder_stmt = conn.prepare("INSERT INTO folders (name, parent_folder_id) VALUES (:name, :parent_folder_id);").unwrap();
        let insert_file_stmt = conn.prepare("INSERT INTO files (name, parent_folder_id, hash, size, modified_date) 
            VALUES (:name, :parent_folder_id, :hash, :size, :modified_date)").unwrap();

        // Here you go
        return RustFileScanDbContext {
            conn,
            insert_file_stmt,
            insert_folder_stmt,
        };
    }

    pub fn create_folder(&mut self, folder: &mut models::FolderModel) 
        -> i64
    {
        let r = self.insert_folder_stmt.execute_named(&[(":name", &folder.name), (":parent_folder_id", &folder.parent_folder_id)]);

        match r {
            Ok(_updated) => {
                let id = self.conn.last_insert_rowid();
                folder.id = id;
                return id;
            },
            Err(err) => {
                println!("Error: {}", err);
                return 0;
            }
        }
    }

    pub fn create_file(&mut self, file: &mut models::FileModel) 
        -> i64
    {
        let size_i64 = file.size as i64;
        let r = self.insert_file_stmt.execute_named(
            &[(":name", &file.name), 
            (":parent_folder_id", &file.parent_folder_id),
            (":hash", &file.hash),
            (":size", &size_i64),
            (":modified_date", &file.modified_date),
            ]
        );

        match r {
            Ok(_updated) => {
                let id = self.conn.last_insert_rowid();
                file.id = id;
                return id;
            },
            Err(err) => {
                println!("Error: {}", err);
                return 0;
            }
        }
    }
}

pub fn initialize_database() -> ()
{
    let conn = Connection::open("rustfilescan.db").unwrap();

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
}