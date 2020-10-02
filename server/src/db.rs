use crate::models::*;
use crate::schema::files;
use diesel::prelude::*;

pub fn create_file(connection: &SqliteConnection, file_name: &str, file_hash: &str) -> Result<(), diesel::result::Error> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let new_file = NewFile {
        file_name,
        file_hash,
        date_created: &current_time,
        date_modified: &current_time,
    };

    diesel::insert_into(files::table)
        .values(&new_file)
        .execute(connection)?;

    Ok(())
}
