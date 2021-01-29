use structopt::StructOpt;

use std::path::PathBuf;

#[derive(StructOpt)]
pub struct Opts {
    #[structopt(parse(from_os_str))]
    pub wasm_file: PathBuf,
}
