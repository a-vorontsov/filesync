use rayon::prelude::*;
use reqwest::blocking;
use std::fs;
use std::time::Duration;
use walkdir::WalkDir;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    WalkDir::new("filesync")
        .into_iter()
        .map(|r| r.unwrap())
        .filter(|r| !r.file_type().is_dir())
        .for_each(|r| println!("{:#?}", r.path().display()));

    let paths: Vec<_> = WalkDir::new("filesync")
        .into_iter()
        .map(|r| r.unwrap())
        .filter(|r| !r.file_type().is_dir())
        .collect();

    paths.into_par_iter().for_each(|entry| {
        let client = blocking::Client::builder()
            .timeout(Duration::from_secs(500))
            .danger_accept_invalid_certs(true)
            .use_native_tls()
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap();

        let absolute_file_path = fs::canonicalize(&entry.path()).unwrap();
        let relative_file_path = &entry.path().strip_prefix("filesync").unwrap().display();

        println!("{}", relative_file_path.to_string());

        let file_bytes = fs::read(&absolute_file_path.to_str().unwrap()).unwrap();

        let mime_type = mime_guess::from_path(&absolute_file_path)
            .first_or_octet_stream()
            .to_string();

        let part = blocking::multipart::Part::bytes(file_bytes)
            .file_name(relative_file_path.to_string())
            .mime_str(&mime_type)
            .unwrap();

        let form = blocking::multipart::Form::new().part("file", part);

        let response = client
            .post("http://localhost:8000")
            .multipart(form)
            .send()
            .unwrap();

        println!("Response: {:#?}", response.status());
    });

    Ok(())
}
