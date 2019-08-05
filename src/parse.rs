use super::{Vars, VarsError};
use std::io::{self, Write};

mod expr;

use expr::{translate_expr};
pub use expr::{ExprError, ExprInternalError};

pub enum TranslateError {
    Input(io::Error),
    Output(io::Error),
    Expr(ExprError),
    Vars(VarsError),
}

impl From<VarsError> for TranslateError {
    fn from(from: VarsError) -> TranslateError {
        TranslateError::Vars(from)
    }
}
impl From<ExprError> for TranslateError {
    fn from(from: ExprError) -> TranslateError {
        TranslateError::Expr(from)
    }
}

pub fn translate<R: Iterator<Item = io::Result<char>>, W: Write>(
    input_chars: &mut R,
    output: &mut W,
    vars: &Box<dyn Vars>,
) -> Result<(), TranslateError> {
    let mut slash = false;
    let mut dollar = false;

    while let Some(rch) = input_chars.next() {
        match rch {
            Err(e) => return Err(TranslateError::Input(e)),
            Ok(ch) => {
                if slash {
                    if ch == '$' {
                        output.write(b"$").map_err(TranslateError::Output)?;
                    } else if ch == '\\' {
                        output.write(b"\\\\").map_err(TranslateError::Output)?;
                    }
                    slash = false;
                } else if dollar {
                    if ch == '{' {
                        translate_expr(input_chars, output, vars)?;
                    }
                    dollar = false
                } else if ch == '\\' {
                    slash = true;
                } else if ch == '$' {
                    dollar = true;
                } else {
                    let mut buf = [0u8, 0, 0, 0];
                    let s = ch.encode_utf8(&mut buf);
                    output.write(s.as_bytes()).map_err(TranslateError::Output)?;
                }
            }
        }
    }
    if slash {
        output.write(b"\\").map_err(TranslateError::Output)?;
    } else if dollar {
        output.write(b"$").map_err(TranslateError::Output)?;
    }
    Ok(())
}
