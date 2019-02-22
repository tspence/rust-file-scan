mod list_files; 
mod files_data;

pub mod schema;
pub mod models;

fn main() {
    
    // Initialize sqlite
    let connection = files_data::establish_connection();

    // Start by scanning subfolders of current
    list_files::list_files_in_folder(connection, "./".to_string(), 0);
}
