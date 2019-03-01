#[derive(Debug)]
pub struct FileModel {
    pub id: i64,
    pub parent_folder_id: i64,
    pub name: String,
    pub hash: String,
    pub size: u64,
    pub modified_date: String,
}

#[derive(Debug)]
pub struct FolderModel {
    pub id: i64,
    pub parent_folder_id: i64,
    pub name: String,

    pub folders: Vec<FolderModel>,
    pub files: Vec<FileModel>,
}

impl FolderModel {
    pub fn total_items(&self)
        -> usize
    {
        let mut count = 1 + self.files.len();
        for child in &(self.folders) {
            count = count + child.total_items();
        }
        return count;
    }
}
