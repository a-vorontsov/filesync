#[macro_use]
extern crate lazy_static;

mod cli;

use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use async_std::prelude::*;
use filesync_core::*;
use futures::{StreamExt, TryStreamExt};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::str;
use structopt::StructOpt;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub save_dir: String,
    pub port: i32,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            save_dir: "filesync/test".to_string(),
            port: 8000,
        }
    }
}

lazy_static! {
    static ref CFG: Config = confy::load("filesync-server").unwrap();
}

fn generate_ssl_config() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(
            "cert/filesync-key.pem",
            SslFiletype::PEM,
        )
        .unwrap();
    builder
        .set_certificate_chain_file("cert/filesync.pem")
        .unwrap();

    builder
}

// fn test() {
//     let args = cli::CliParams::from_args();
//     fs::create_dir_all(&args.save_dir).expect("Failed to create save directory");

//     let input = fs::read(&args.input).expect("Failed to read the input file");

//     let hash_string = generate_contents_hash(&input);
//     let compressed_path = generate_compressed_file_path(&args.save_dir, &hash_string);

//     let compressed_bytes = compress_contents(&input);
//     write_file(&PathBuf::from(compressed_path.clone()), compressed_bytes);

//     let compressed_file =
//         fs::read(compressed_path.clone()).expect("Failed to read the compressed file");
//     let decompressed_bytes = decompress_contents(&compressed_file);
//     write_file(&args.output, decompressed_bytes);
// }

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;

        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;

        let filepath = format!("{}/{}", CFG.save_dir, &filename);

        async_std::fs::create_dir_all(Path::new(&filepath).parent().unwrap()).await?;

        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let args = cli::CliParams::from_args();

    let save_dir = match &args.save_dir {
        Some(x) => x.to_str().unwrap(),
        None => &CFG.save_dir,
    };

    async_std::fs::create_dir_all(&save_dir).await?;
    let save_dir_full_path = fs::canonicalize(PathBuf::from(save_dir)).unwrap();

    let port = match &args.port {
        Some(x) => x,
        None => &CFG.port,
    };

    let ip = format!("0.0.0.0:{}", port);

    let ssl_config = generate_ssl_config();

    println!("Starting filesync server on:\n\t{}", ip);
    println!("Save directory configured as:\n\t{:?}", &save_dir_full_path);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(
                web::resource("/")
                    .route(web::get().to(greet))
                    .route(web::post().to(save_file)),
            )
    })
    .bind_openssl(ip, ssl_config)?
    .run()
    .await
}
