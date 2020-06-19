use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct CliParams {
    #[structopt(
        short = "s",
        long = "save-dir",
        parse(from_os_str),
    )]
    pub save_dir: Option<PathBuf>,

    #[structopt(short, long)]
    pub port: Option<i32>,
}
