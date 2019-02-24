extern crate dotenv;

use filescandb;


fn main() {
    
    // Initialize sqlite
    let connection = filescandb::establish_connection();

    // Start by scanning subfolders of current
    filescandb::list_files_in_folder(&connection, "./".to_string(), 0);
}
