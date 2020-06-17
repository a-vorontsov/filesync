mod cli;

use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use async_std::prelude::*;
use filesync_core::*;
use futures::{StreamExt, TryStreamExt};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

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
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
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

    async_std::fs::create_dir_all("./tmp").await?;

    let args = cli::CliParams::from_args();

    let ip = format!("0.0.0.0:{}", &args.port);

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::resource("/")
                .route(web::get().to(greet))
                .route(web::post().to(save_file)),
        )
    })
    .bind(ip)?
    .run()
    .await
}
