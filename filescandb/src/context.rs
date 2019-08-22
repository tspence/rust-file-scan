extern crate rusqlite;

use rusqlite::{ NO_PARAMS, Connection, Statement, Error };

use crate::models;

pub struct RustFileScanDbContext<'a> {
    conn: &'a Connection,
    folder_create_stmt: Option<Statement<'a>>,
    folder_retrieve_stmt: Option<Statement<'a>>,
    folder_update_stmt: Option<Statement<'a>>,
    folder_delete_stmt: Option<Statement<'a>>,
    file_create_stmt: Option<Statement<'a>>,
    file_retrieve_stmt: Option<Statement<'a>>,
    file_update_stmt: Option<Statement<'a>>,
    file_delete_stmt: Option<Statement<'a>>,
}

impl<'a> RustFileScanDbContext<'a>
{
    pub fn new(conn: &'a Connection) -> Self
    {
        // Here you go
        return RustFileScanDbContext {
            conn,
            folder_create_stmt: None,
            folder_retrieve_stmt: None,
            folder_update_stmt: None,
            folder_delete_stmt: None,
            file_create_stmt: None,
            file_retrieve_stmt: None,
            file_update_stmt: None,
            file_delete_stmt: None,
        };
    }

    pub fn find_potential_duplicate_files(&mut self)
        -> Result<Vec<models::FileModel>, Error>
    {
        // Find potential matches by size
        let mut stmt = self.conn.prepare(
            "select f.*
            from files f
            inner join (
                select size from files group by size having count(1) > 1
            ) s on s.size = f.size
            where f.hash = '';")?;
        let mut rows = stmt.query(NO_PARAMS)?;

        // Convert results into a vector of matching files
        let mut results = Vec::<models::FileModel>::new();
        while let Some(maybe_row) = rows.next() {
            let row = maybe_row?;
            let obj = models::FileModel {
                id: row.get(0),
                parent_folder_id: row.get(1),
                name: row.get(2),
                hash: row.get(3),
                size: row.get(4),
                modified_date: row.get(5),
            };
            results.push(obj);
        }

        // Here's the files that might be duplicates
        return Ok(results);
    }

    pub fn create_folder(&mut self, folder: &mut models::FolderModel) 
        -> Result<i64, Error>
    {
        if let None = &self.folder_create_stmt {
            let stmt = self.conn.prepare("INSERT INTO folders (name, parent_folder_id) VALUES (:name, :parent_folder_id);").unwrap();
            self.folder_create_stmt = Some(stmt);
        };

        self.folder_create_stmt.as_mut().unwrap().execute_named(
            &[(":name", &folder.name), 
            (":parent_folder_id", &folder.parent_folder_id)])?;

        let id = self.conn.last_insert_rowid();
        folder.id = id;
        return Ok(id);
    }

    pub fn retrieve_folder(&mut self, id: i64)
        -> Result<models::FolderModel, Error>
    {
        if let None = &self.folder_retrieve_stmt {
            let stmt = self.conn.prepare("SELECT id, name, parent_folder_id FROM folders WHERE id = :id;").unwrap();
            self.folder_retrieve_stmt = Some(stmt);
        };

        let mut rows = self.folder_retrieve_stmt.as_mut().unwrap()
            .query_named(&[(":id", &id)])?;

        while let Some(maybe_row) = rows.next() {
            let row = maybe_row?;
            let obj = models::FolderModel {
                id: row.get(0),
                name: row.get(1),
                parent_folder_id: row.get(2),
                folders: Vec::<models::FolderModel>::new(),
                files: Vec::<models::FileModel>::new(),
            };
            return Ok(obj);
        }

        return Err(Error::QueryReturnedNoRows);
    }

    pub fn update_folder(&mut self, folder: &models::FolderModel) 
        -> Result<(), Error>
    {
        if let None = &self.folder_update_stmt {
            let stmt = self.conn.prepare("UPDATE folders SET name = :name, parent_folder_id = :parent_folder_id WHERE id = :id;").unwrap();
            self.folder_update_stmt = Some(stmt);
        };

        self.folder_update_stmt.as_mut().unwrap().execute_named(
            &[(":id", &folder.id), 
            (":name", &folder.name), 
            (":parent_folder_id", &folder.parent_folder_id)])?;
        return Ok(());
    }

    pub fn delete_folder(&mut self, id: i64) 
        -> Result<(), Error>
    {
        if let None = &self.folder_delete_stmt {
            let stmt = self.conn.prepare("DELETE FROM folders WHERE id = :id;").unwrap();
            self.folder_delete_stmt = Some(stmt);
        };

        self.folder_delete_stmt.as_mut().unwrap().execute_named(
            &[(":id", &id)])?;
        return Ok(());
    }

    pub fn create_file(&mut self, file: &mut models::FileModel) 
        -> Result<i64, Error>
    {
        if let None = &self.file_create_stmt {
            let stmt = self.conn.prepare("INSERT INTO files (name, parent_folder_id, hash, size, modified_date) 
                VALUES (:name, :parent_folder_id, :hash, :size, :modified_date)").unwrap();
            self.file_create_stmt = Some(stmt);
        };

        self.file_create_stmt.as_mut().unwrap().execute_named(
            &[(":name", &file.name), 
            (":parent_folder_id", &file.parent_folder_id),
            (":hash", &file.hash),
            (":size", &file.size),
            (":modified_date", &file.modified_date),
            ]
        )?;

        let id = self.conn.last_insert_rowid();
        file.id = id;
        return Ok(id);
    }

    pub fn update_file(&mut self, file: &models::FileModel) 
        -> Result<(), Error>
    {
        if let None = &self.file_update_stmt {
            let stmt = self.conn.prepare(
                "UPDATE files 
                SET name = :name, 
                parent_folder_id = :parent_folder_id, 
                hash = :hash, 
                size = :size, 
                modified_date = :modified_date 
                WHERE id = :id;").unwrap();
            self.file_update_stmt = Some(stmt);
        };

        self.file_update_stmt.as_mut().unwrap().execute_named(
            &[(":name", &file.name), 
            (":parent_folder_id", &file.parent_folder_id),
            (":hash", &file.hash),
            (":size", &file.size),
            (":modified_date", &file.modified_date),
            ])?;
        return Ok(());
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