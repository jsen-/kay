use std::io;
use std::env::var_os as env_var;

use super::{Vars, VarsError};

#[derive(Debug)]
pub enum ExprInternalError {
    UnexpectedEof,
    UnknownExpressionType,
    UnknownEnv(String),
}

pub enum ExprError {
    Vars(VarsError),
    Input(io::Error),
    Output(io::Error),
    Internal(ExprInternalError),
}

impl From<VarsError> for ExprError {
    fn from(from: VarsError) -> ExprError {
        ExprError::Vars(from)
    }
}
impl From<ExprInternalError> for ExprError {
    fn from(from: ExprInternalError) -> ExprError {
        ExprError::Internal(from)
    }
}

enum ExprType {
    Env,
    Var,
}

pub fn translate_expr<R: Iterator<Item = io::Result<char>>, W: io::Write>(
    input_chars: &mut R,
    output: &mut W,
    vars: &Box<dyn Vars>,
) -> Result<(), ExprError> {
    let mut s = String::new();
    loop {
        match input_chars.next() {
            None => {
                return Err(ExprInternalError::UnexpectedEof)?
            }
            Some(Err(e)) => return Err(ExprError::Input(e)),
            Some(Ok(ch)) if ch == '}' => break,
            Some(Ok(ch)) => s.push(ch),
        }
    }

    if s.bytes().len() < 4 {
        return Err(ExprInternalError::UnknownExpressionType)?;
    }

    let (expr_type, expr_path) = if &s[0..4] == "env " {
        (ExprType::Env, &s[4..])
    } else if &s[0..4] == "var " {
        (ExprType::Var, &s[4..])
    } else {
        return Err(ExprInternalError::UnknownExpressionType)?;
    };
    match expr_type {
        ExprType::Var => {
            output.write(vars.get(expr_path)?.as_bytes()).map_err(ExprError::Output)?;
            Ok(())
        },
        ExprType::Env => match env_var(expr_path) {
            Some(value) => {
                use std::os::unix::ffi::OsStrExt; // linux only for now ... sorry
                output.write(value.as_bytes()).map_err(ExprError::Output)?;
                Ok(())
            }
            None => Err(ExprInternalError::UnknownEnv(expr_path.into()))?,
        },
    }
}
