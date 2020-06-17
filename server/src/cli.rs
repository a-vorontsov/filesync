use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CliParams {
    // #[structopt(parse(from_os_str))]
    // pub input: PathBuf,

    // #[structopt(
    //     short = "s",
    //     long = "save-dir",
    //     parse(from_os_str),
    //     default_value = "filesync"
    // )]
    // pub save_dir: PathBuf,

    // #[structopt(short, long, parse(from_os_str), default_value = "asdf")]
    // pub output: PathBuf,

    #[structopt(short, long)]
    pub port: i32,
}
