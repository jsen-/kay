use std::io;
use std::path::PathBuf;

use structopt::StructOpt;

mod cli;
mod error;
mod parse;
mod position_iterator;
mod read_chars;
mod streams;
mod vars;

use error::Error;
use parse::{ExprError, ExprInternalError, TranslateError};
use position_iterator::PositionIterator;
use read_chars::ReadChars;
use streams::{InputStream, OutputStream};
pub use vars::Vars;
use vars::{EmptyVars, JsonVars, VarsError, VarsFormat, YamlVars};

fn cli_args<'a>(stdin: &'a io::Stdin, stdout: &'a io::Stdout) -> Result<(InputStream<'a>, OutputStream<'a>, Box<dyn Vars>, Option<PathBuf>), Error<'static>> {
    let opt = cli::Opt::from_args();

    let input = if let Some(input_file) = opt.input_file {
        InputStream::from_file(input_file)?
    } else {
        if atty::is(atty::Stream::Stdin) {
            eprintln!("No input file provided, reading from stdin ...");
        }
        InputStream::from_stdin(stdin)
    };

    let output = if let Some(output_file) = opt.output_file {
        OutputStream::from_file(output_file)?
    } else {
        OutputStream::from_stdin(stdout)
    };

    let vars: Box<dyn Vars> = if let Some(vars_file) = opt.vars_file.clone() {
        let format = match (opt.vars_format, VarsFormat::from_extension(&vars_file)) {
            (Some(f), _) => f,
            (None, Some(f)) => f,
            _ => Err(Error::UnknownFormat)?,
        };
        match format {
            VarsFormat::Json => Box::new(JsonVars::from_file(vars_file)?),
            VarsFormat::Yaml => Box::new(YamlVars::from_file(vars_file)?),
        }
    } else {
        Box::new(EmptyVars::new())
    };
    Ok((input, output, vars, opt.vars_file))
}


fn main() -> Result<(), Error<'static>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let (mut input, mut output, vars, vars_file) = cli_args(&stdin, &stdout)?;

    let mut input_chars = PositionIterator::from(input.chars());

    if let Err(err) = parse::translate(&mut input_chars, &mut output, &vars) {
        eprintln!(
            r#"Error [line: {} col: {}] {}"#,
            input_chars.line(),
            input_chars.col(),
            match err {
                TranslateError::Input(io_err) => Error::Input(input, io_err),
                TranslateError::Output(io_err) => Error::Output(output, io_err),
                TranslateError::Expr(ExprError::Output(io_err)) => Error::Output(output, io_err),
                TranslateError::Expr(ExprError::Input(io_err)) => Error::Input(input, io_err),
                TranslateError::Expr(ExprError::Vars(vars_err)) => {
                    Error::Vars(vars_file, vars_err)
                }
                TranslateError::Expr(ExprError::Internal(expr_err)) => Error::Expr(expr_err),
                TranslateError::Vars(expr_err) => Error::Vars(vars_file, expr_err),
            },
        );
        std::process::exit(1);
    }
    Ok(())
}
