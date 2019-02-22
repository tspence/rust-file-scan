use super::schema::folders;
use super::schema::files;

#[derive(Queryable)]
pub struct FolderModel {
    pub id: i32,
    pub parent_id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct FileModel {
    pub id: i32,
    pub folder_id: i32,
    pub name: String,
    pub hash: String,
    pub size: i32,
    pub modified_date: String,
}

#[derive(Insertable)]
#[table_name="folders"]
pub struct NewFolder<'a> {
    pub name: &'a str,
    pub parent_id: &'a i32,
}

#[derive(Insertable)]
#[table_name="files"]
pub struct NewFile<'a> {
    pub name: &'a str,
    pub folder_id: &'a i32,
    pub hash: &'a str,
    pub size: &'a i32,
    pub modified_date: &'a str,
}
