pub fn list_files_in_folder(conn: &PgConnection, path: String, parent_id: i32) {

    // Get a list of all things in this directory
    let dirlist = std::fs::read_dir(path).unwrap();

    // Let's go into them
    for entry in dirlist {
        let path = entry.unwrap().path();
        if (path.is_dir()) {

            // Add this directory first
            let folder = create_folder(conn, path.file_name(), parent_id);
            list_files_in_folder(conn, path.to_str().unwrap().to_string(), folder.id);
        } else if (parent_id != 0) {
            let md = path.metadata().unwrap();
            let file = create_file(conn, path.file_name(), parent_id, "", md.len(), "");
            println!("Name: {}", path.to_str().unwrap().to_string());
        }
    }
}

