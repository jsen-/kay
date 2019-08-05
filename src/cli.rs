use std::path::PathBuf;

use structopt::StructOpt;

use super::VarsFormat;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "kay",
    about = "replace ${...} expressions in text",
    rename_all = "kebab-case"
)]
pub struct Opt {
    #[structopt(short = "i", long = "--input-file", parse(from_os_str))]
    pub input_file: Option<PathBuf>,

    #[structopt(short = "v", long = "--vars-file", parse(from_os_str))]
    pub vars_file: Option<PathBuf>,

    #[structopt(long = "--vars-format")]
    pub vars_format: Option<VarsFormat>,

    #[structopt(short = "o", long = "--output-file", parse(from_os_str))]
    pub output_file: Option<PathBuf>,
}