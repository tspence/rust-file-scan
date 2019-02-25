extern crate dotenv;

use filescandb;
use std::time::Instant;

fn main() {
    
    // Initialize sqlite
    let connection = filescandb::establish_connection();

    // Start by scanning subfolders of current
    let now = Instant::now();
    let folder_result = filescandb::list_files_in_folder("/users/tspence/fbsource/fbcode".to_string());
    match folder_result {
        Err(e) => println!("Err: {}", e.to_string()),
        Ok(folder) => {
            let elapsed = now.elapsed();
            let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
            println!("Captured {} items in seconds: {}", filescandb::total_items(&folder), sec);

            // Now insert items into the database
            let now = Instant::now();
            filescandb::write(&folder);
            let elapsed = now.elapsed();
            let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
            println!("Captured {} items in seconds: {}", filescandb::total_items(&folder), sec);    
        }
    }
}
