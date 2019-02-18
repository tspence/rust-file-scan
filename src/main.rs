mod list_files;

fn main() {
    println!("Hello, world!");
    list_files::list_files_in_folder("./".to_string())
}
