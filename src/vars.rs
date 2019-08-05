use std::fmt;
use std::str;
use std::io;
use std::path::Path;
use std::ffi::OsStr;
use serde_json::Value as JsonValue;
use strict_yaml_rust::StrictYamlLoader;

use super::Error;

fn file_to_string<'p, 'e: 'p>(path: &'p Path) -> Result<String, Error<'e>> {
    fn internal(path: &Path) -> io::Result<String> {
        let mut file = std::fs::File::open(path)?;
        file.read_into_string()
    }
    internal(path).map_err(|e| Error::FileRead(path.into(), e))
}

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

pub enum VarsFormat {
    Json,
    Yaml,
}
impl VarsFormat {
    pub fn from_extension<P: AsRef<Path>>(path: P) -> Option<VarsFormat> {
        match path.as_ref().extension().and_then(OsStr::to_str) {
            Some("yaml") | Some("yml") => Some(VarsFormat::Yaml),
            Some("json") => Some(VarsFormat::Json),
            _ => None,
        }
    }
}
impl str::FromStr for VarsFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(VarsFormat::Json),
            "yaml" => Ok(VarsFormat::Yaml),
            _ => Err(String::from(r#"only "json" and "yaml" are supported"#)),
        }
    }
}
impl fmt::Debug for VarsFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VarsFormat::Json => write!(f, "json"),
            VarsFormat::Yaml => write!(f, "yaml"),
        }
    }
}

pub trait Vars {
    fn get(&self, path: &str) -> Result<&str, VarsError>;
}

#[derive(Debug)]
pub enum VarsError {
    NotFound(String),
    InvalidSelector(String),
}

pub struct JsonVars {
    json: JsonValue,
}
impl Vars for JsonVars {
    fn get(&self, path: &str) -> Result<&str, VarsError> {
        // let mut selector = jsonpath_lib::selector(&self.json);
        // selector(path);
        Err(VarsError::NotFound(path.into()))
    }
}
impl JsonVars {
    pub fn from_file<'a, P: AsRef<Path> + 'a>(path: P) -> Result<Self, super::Error<'a>> {
        let path = path.as_ref();
        let buffer = file_to_string(path)?;
        Ok(JsonVars {
            json: serde_json::from_str::<JsonValue>(&buffer)
                .map_err(|json_error| Error::JsonParseVars(path.into(), json_error))?
        })
    }
}

pub struct YamlVars {}
impl Vars for YamlVars {
    fn get(&self, path: &str) -> Result<&str, VarsError> {
        Err(VarsError::NotFound(path.into()))
    }
}

impl YamlVars {
    pub fn from_file<'a, P: AsRef<Path> + 'a>(path: P) -> Result<Self, super::Error<'a>> {
        let path = path.as_ref();
        let buffer = file_to_string(path)?;
        let yaml = StrictYamlLoader::load_from_str(&buffer)
            .map_err(|yaml_error| super::Error::YamlParseVars(path.into(), yaml_error));
        Ok(YamlVars {})
    }
}

pub struct EmptyVars {}
impl Vars for EmptyVars {
    fn get(&self, path: &str) -> Result<&str, VarsError> {
        Err(VarsError::NotFound(path.into()))
    }
}
impl EmptyVars {
    pub fn new() -> Self {
        EmptyVars {}
    }
}
