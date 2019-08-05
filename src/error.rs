use std::path::PathBuf;
use std::io;
use std::fmt;
use super::{InputStream, OutputStream, ExprInternalError, VarsError};

#[derive(Debug)]
pub enum Error<'a> {
    UnknownFormat,
    FileRead(PathBuf, io::Error),
    FileWrite(PathBuf, io::Error),
    Input(InputStream<'a>, io::Error),
    Output(OutputStream<'a>, io::Error),
    JsonParseVars(PathBuf, serde_json::error::Error),
    YamlParseVars(PathBuf, strict_yaml_rust::ScanError),
    Expr(ExprInternalError),
    Vars(Option<PathBuf>, VarsError),
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(stream, error) => match stream {
                InputStream::File(path, _) => write!(f, r#"Error reading from "{}": {} "#, path.display(), error),
                InputStream::Stdin(_) => write!(f, r#"Error reading from stdin: {} "#, error),
            },
            Self::Output(stream, error) => match stream {
                OutputStream::File(path, _) => write!(f, r#"Error writing to "{}": {} "#, path.display(), error),
                OutputStream::Stdout(_) => write!(f, r#"Error writing to stdout: {} "#, error),
            },
            Self::UnknownFormat => write!(f, r#"Unable to determine vars format from file extension, please use --vars-format argument"#),
            Self::JsonParseVars(path, json_error) => write!(f, r#"Unable to parse json vars file "{}": {}"#, path.display(), json_error),
            Self::YamlParseVars(path, yaml_error) => write!(f, r#"Unable to parse yaml vars file "{}": {} "#, path.display(), yaml_error),
            Self::FileRead(path, error) => write!(f, r#"Unable to read file "{}": {} "#, path.display(), error),
            Self::FileWrite(path, error) => write!(f, r#"Unable to write file "{}": {} "#, path.display(), error),
            Self::Expr(expr_err) => {
                match expr_err {
                    ExprInternalError::UnexpectedEof => write!(f, r#"Unexpected end of file in "#),
                    ExprInternalError::UnknownExpressionType => write!(f, r#"Unknown expression type"#),
                    ExprInternalError::UnknownEnv(var_name) => write!(f, r#"Environment variable "{}" is not defined"#, var_name),
                }
            }
            Self::Vars(_, VarsError::InvalidSelector(selector)) => write!(f, r#"Variable selector "{}" is invalid"#, selector),
            Self::Vars(Some(path), VarsError::NotFound(selector)) => write!(f, r#"Variable "{}" not found in "{}""#, selector, path.display()),
            Self::Vars(None, VarsError::NotFound(selector)) => write!(f, r#"No "vars-file" but input file is looking for {}"#, selector),
        }
    }
}