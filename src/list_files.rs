extern crate rusqlite;

pub fn list_files_in_folder(path: String) {

    // Get a list of all things in this directory
    let dirlist = std::fs::read_dir(path).unwrap();

    // Let's go into them
    for entry in dirlist {
        let path = entry.unwrap().path();
        if (path.is_dir()) {
            list_files_in_folder(path.to_str().unwrap().to_string());
        } else {
            println!("Name: {}", path.to_str().unwrap().to_string());
        }
    }
}

