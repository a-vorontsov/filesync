use flate2::write::{ZlibDecoder, ZlibEncoder};
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn write_file(path: &PathBuf, contents: Vec<u8>) {
    fs::write(path, contents).expect("Failed to write file");
}

pub fn generate_contents_hash(contents: &Vec<u8>) -> String {
    let hash = Sha1::new().chain(&contents.clone()).finalize();
    format!("{:x}", hash)
}

pub fn generate_compressed_file_path(save_dir: &PathBuf, hash_string: &str) -> String {
    format!("{}/{}", save_dir.to_str().unwrap(), &hash_string)
}

pub fn compress_contents(contents: &Vec<u8>) -> Vec<u8> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::best());
    e.write_all(&contents.clone())
        .expect("Failed to write contents to encoder");
    let compressed_bytes = e.finish().expect("Failed to consume and flush encoder");

    compressed_bytes
}

pub fn decompress_contents(contents: &Vec<u8>) -> Vec<u8> {
    let mut d = ZlibDecoder::new(Vec::new());
    d.write_all(&contents.clone())
        .expect("Failed to write contents to decoder");
    let decompressed_bytes = d.finish().expect("Failed to consume and flush decoder");

    decompressed_bytes
}
