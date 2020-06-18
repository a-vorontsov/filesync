use rayon::prelude::*;
use reqwest::blocking;
use std::fs;
use std::time::Duration;
use walkdir::WalkDir;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let paths: Vec<_> = WalkDir::new("filesync")
        .into_iter()
        .map(|r| r.unwrap())
        .filter(|r| !r.file_type().is_dir())
        .collect();

    paths.into_par_iter().for_each(|entry| {
        let client = blocking::Client::new();
        let file_path = fs::canonicalize(&entry.path()).unwrap();
        let file_bytes = fs::read(&file_path.to_str().unwrap()).unwrap();
        let mime_type = mime_guess::from_path(&file_path)
            .first_or_octet_stream()
            .to_string();

        let part = blocking::multipart::Part::bytes(file_bytes)
            .file_name(format!("{:?}", entry.file_name()))
            .mime_str(&mime_type)
            .unwrap();

        let form = blocking::multipart::Form::new().part("file", part);

        let response = client
            .post("http://192.168.0.25:5000")
            .multipart(form)
            .timeout(Duration::new(300, 0))
            .send()
            .unwrap();

        println!("Response: {:#?}", response.status());
    });

    Ok(())
}
