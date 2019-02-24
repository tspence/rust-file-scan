mod lib;

extern crate diesel;
extern crate dotenv;

fn main() {
    
    // Initialize sqlite
    let connection = lib::establish_connection();

    // Start by scanning subfolders of current
    lib::list_files_in_folder(connection, "./".to_string(), 0);
}
