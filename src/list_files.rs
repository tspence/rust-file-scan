pub fn list_files_in_folder(path: String) {
    let paths = std::fs::read_dir(path).unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}