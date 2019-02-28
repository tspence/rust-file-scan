extern crate rusqlite;

use rusqlite::{ Connection, Transaction, CachedStatement };

pub struct FileScanDbContext<'a> {

    // The path to the database
    pub file_name: String,

    // The connection to the database, if connected
    pub conn: Connection,
    pub tx: Option<Transaction<'a>>,

    // Cached statements
    pub stmt_insert_file: CachedStatement<'a>,
    pub stmt_insert_folder: CachedStatement<'a>,
}
