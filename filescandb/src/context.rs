extern crate rusqlite;

use rusqlite::{ Connection, Statement };

pub struct RustFileScanDbContext<'a> {
    pub conn: &'a Connection,
    pub insert_folder_stmt: Statement<'a>,
    pub insert_file_stmt: Statement<'a>,
}

impl<'a> RustFileScanDbContext<'a>
{
    pub fn new(
        conn: &'a Connection
        ) -> Self
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

    pub fn initialize(self: &RustFileScanDbContext<'a>) -> ()
    {
        // clean tables
        self.conn.execute_batch("drop table if exists files;").unwrap();
        self.conn.execute_batch("drop table if exists folders;").unwrap();

        // Folders table
        self.conn.execute_batch("create table if not exists folders (
            id integer primary key not null,
            parent_folder_id integer not null,
            name text not null
        );").unwrap();

        // Files table
        self.conn.execute_batch("create table if not exists files (
            id integer primary key not null,
            parent_folder_id integer not null,
            name text not null,
            hash text not null,
            size integer not null,
            modified_date text not null
        );").unwrap();
    }
}
