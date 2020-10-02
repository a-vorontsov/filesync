use super::schema::files;

#[derive(Queryable)]
pub struct File {
    pub id: i32,
    pub file_name: String,
    pub date_created: i64,
    pub date_modified: i64,
    pub file_hash: String,
}

#[derive(Queryable)]
pub struct FilesHistory {
    pub id: i32,
    pub file_id: String,
    pub file_hash: String,
    pub date_modified: i64,
}

#[derive(Insertable)]
#[table_name="files"]
pub struct NewFile<'a> {
    pub file_name: &'a str,
    pub file_hash: &'a str,
    pub date_created: &'a i64,
    pub date_modified: &'a i64,
}
