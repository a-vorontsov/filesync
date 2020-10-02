-- Your SQL goes here
CREATE TABLE files (
    id INTEGER PRIMARY KEY NOT NULL,
    file_name TEXT NOT NULL,
    date_created INTEGER NOT NULL,
    date_modified INTEGER NOT NULL,
    file_hash TEXT NOT NULL
);

CREATE TABLE files_history (
    id INTEGER PRIMARY KEY NOT NULL,
    file_id INTEGER NOT NULL,
    file_hash TEXT NOT NULL,
    date_modified INTEGER NOT NULL,
    FOREIGN KEY(file_id) REFERENCES files(id),
    FOREIGN KEY(file_hash) REFERENCES files(file_hash),
    FOREIGN KEY(date_modified) REFERENCES files(date_modified)
);
