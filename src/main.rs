mod list_files; 
mod files_data;
extern crate rusqlite;

fn main() {

    // Initialize sqlite
    files_data::setup().unwrap();

    // Start by scanning subfolders of current
    list_files::list_files_in_folder("./".to_string());

    // List everything we found
    let _result = files_data::report();
}
