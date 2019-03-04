extern crate dotenv;
extern crate rusqlite;

use filescandb;
use rusqlite::{ Connection, };
use std::time::Instant;

fn main() 
{    
    // Start by scanning subfolders of current
    let now = Instant::now();
    let folder_result = filescandb::list_files_in_folder("/users/tspence/fbsource/fbcode".to_string());
    match folder_result {
        Err(e) => println!("Err: {}", e.to_string()),
        Ok(mut folder) => {
            let elapsed = now.elapsed();
            let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
            println!("Captured {} file and folder records in {} seconds.", folder.total_items(), sec);

            // Prepare to begin working on the database
            {
                filescandb::context::initialize_database();
            }

            // Now insert items into the database
            let now = Instant::now();
            filescandb::write_folder_nested(&mut folder);
            let elapsed = now.elapsed();
            let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
            println!("Inserted {} items into the database in {} seconds.", folder.total_items(), sec);    

            // Read an item back
            {
                let conn = Connection::open("rustfilescan.db").unwrap();
                let mut ctxt = filescandb::context::RustFileScanDbContext::new(&conn);
                let mut read_back = ctxt.retrieve_folder(folder.id).unwrap();
                println!("Comparing {} {} to read back {} {}", folder.id, folder.name, read_back.id, read_back.name);
                read_back.name = read_back.name + " hi mom!";
                ctxt.update_folder(&read_back).unwrap();
                let read_back2 = ctxt.retrieve_folder(folder.id).unwrap();
                println!("Comparing {} {} to readback2: {} {}", folder.id, folder.name, read_back2.id, read_back2.name);
            }
        }
    }
}
