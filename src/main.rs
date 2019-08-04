#![allow(dead_code, unused_variables, unused_imports)]

mod parse;
mod position_iterator;
mod read_chars;

use position_iterator::PositionIterator;
use read_chars::ReadChars;

use std::ffi::OsStr;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use strict_yaml_rust::StrictYamlLoader;
use structopt::StructOpt;

#[derive(Debug, snafu::Snafu)]
enum Error {
    #[snafu(display(
        r#"Could not determine "varsfile" format. Please provide --vars-format argument"#
    ))]
    UnknownFormat,

    #[snafu(display(r#"Unable to read "{}""#, path.display()))]
    ReadFile { path: PathBuf, io_err: io::Error },

    #[snafu(display(r#"Unable to write "{}""#, path.display()))]
    WriteFile { path: PathBuf, io_err: io::Error },
    // #[snafu(display("Could not save config to {}: {}", filename.display(), source))]
    // SaveConfig {
    //     filename: PathBuf,
    //     source: std::io::Error,
    // },
    // #[snafu(display("The user id {} is invalid", user_id))]
    // UserIdInvalid { user_id: i32, backtrace: Backtrace },
}
type Result<T> = std::result::Result<T, Error>;

trait FileToString {
    fn read_into_string(&mut self) -> io::Result<String>;
}
impl<T> FileToString for T
where
    T: std::io::Read,
{
    fn read_into_string(&mut self) -> io::Result<String> {
        let mut buffer = String::new();
        self.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}

fn file_to_string(path: &Path) -> Result<String> {
    let mut file = std::fs::File::open(path).map_err(|e| Error::ReadFile {
        path: path.into(),
        io_err: e,
    })?;
    file.read_into_string().map_err(|e| Error::ReadFile {
        path: path.into(),
        io_err: e,
    })
}

enum VarsFormat {
    Json,
    Yaml,
}
impl VarsFormat {
    pub fn from_extension<P: AsRef<Path>>(path: P) -> Option<VarsFormat> {
        match path.as_ref().extension().and_then(OsStr::to_str) {
            Some("yaml") => Some(VarsFormat::Yaml),
            Some("json") => Some(VarsFormat::Json),
            _ => None,
        }
    }
}

pub trait Vars {
    fn get(&self, path: &str) -> Option<&str>;
}

struct JsonVars {}
impl Vars for JsonVars {
    fn get(&self, path: &str) -> Option<&str> {
        None
    }
}
impl JsonVars {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(JsonVars {})
    }
}

struct YamlVars {}
impl Vars for YamlVars {
    fn get(&self, path: &str) -> Option<&str> {
        None
    }
}

impl YamlVars {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let buffer = file_to_string(path.as_ref())?;
        let yaml = StrictYamlLoader::load_from_str(&buffer);
        Ok(YamlVars {})
    }
}

struct EmptyVars {}
impl Vars for EmptyVars {
    fn get(&self, path: &str) -> Option<&str> {
        None
    }
}
impl EmptyVars {
    pub fn new() -> Self {
        EmptyVars {}
    }
}

impl std::str::FromStr for VarsFormat {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "json" => Ok(VarsFormat::Json),
            "yaml" => Ok(VarsFormat::Yaml),
            _ => Err(String::from(r#"only "json" and "yaml" are supported"#)),
        }
    }
}

impl std::fmt::Debug for VarsFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VarsFormat::Json => write!(f, "json"),
            VarsFormat::Yaml => write!(f, "yaml"),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "kay",
    about = "replace ${...} expressions in text",
    rename_all = "kebab-case"
)]
struct Opt {
    #[structopt(short = "i", long = "--input-file", parse(from_os_str))]
    pub input_file: Option<PathBuf>,

    #[structopt(short = "v", long = "--vars-file", parse(from_os_str))]
    pub vars_file: Option<PathBuf>,

    #[structopt(long = "--vars-format")]
    pub vars_format: Option<VarsFormat>,

    #[structopt(short = "o", long = "--output-file", parse(from_os_str))]
    pub output_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut input: Box<dyn Read> = if let Some(input_file) = opt.input_file {
        let file = std::fs::File::open(&input_file).map_err(|e| Error::ReadFile {
            path: input_file,
            io_err: e,
        })?;
        Box::new(BufReader::new(file))
    } else {
        Box::new(stdin.lock())
    };

    let mut output: Box<dyn Write> = if let Some(output_file) = opt.output_file {
        let file = std::fs::File::create(&output_file).map_err(|e| Error::WriteFile {
            path: output_file,
            io_err: e,
        })?;
        Box::new(BufWriter::new(file))
    } else {
        Box::new(stdout.lock())
    };

    let vars: Box<dyn Vars> = if let Some(vars_file) = opt.vars_file {
        let format = match (opt.vars_format, VarsFormat::from_extension(&vars_file)) {
            (Some(f), _) => f,
            (None, Some(f)) => f,
            _ => Err(Error::UnknownFormat)?,
        };
        match format {
            VarsFormat::Json => Box::new(JsonVars::from_file(&vars_file)?),
            VarsFormat::Yaml => Box::new(YamlVars::from_file(&vars_file)?),
        }
    } else {
        Box::new(EmptyVars::new())
    };

    let mut input_chars = PositionIterator::from(input.chars());

    if let Err(err) = parse::translate(&mut input_chars, &mut output, &vars) {
        eprintln!(
            r#"Error [line: {} col: {}] {}"#,
            input_chars.line(),
            input_chars.col(),
            err,
        );
        std::process::exit(1);
    }
    Ok(())
}
