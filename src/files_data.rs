extern crate rusqlite;

use rusqlite::{Connection, Result, NO_PARAMS};

static mut conn: Option<rusqlite::Connection> = None;

pub fn setup() -> Result<()> {

    // Open the database
    println!("Initializing database...");
    let local = Connection::open("filescan.db").unwrap();

    // Create the folder table
    local.execute(
        "create table if not exists folders (
             id integer primary key,
             parent_id integer null,
             name text not null
         )",
        NO_PARAMS,
    )?;

    // Create the file table
    local.execute(
        "create table if not exists files (
             id integer primary key,
             parent_id integer,
             name text not null,
             size integer,
             modified_date text not null
         )",
        NO_PARAMS,
    )?;

    // Done
    unsafe {
        conn = Some(local);
    }
    return Ok(());
}

pub fn add_folder(path: String) -> Result<()> {
    let mut stmt = try!(conn.prepare("INSERT INTO folders (name) VALUES (:name)"));
    stmt.execute_named(&[(":name", path)])
}

pub fn add_file(path: String) -> Result<()> {
    let mut stmt = try!(conn.prepare("INSERT INTO folders (name) VALUES (:name)"));
    stmt.execute_named(&[(":name", path)])
}

pub fn report() -> Result<()> {
    /*unsafe {
        match &conn {
            None => (),
            Some(v) => {
                v.close();
            },
        }
    }*/
    println!("Done");
    return Ok(());
}