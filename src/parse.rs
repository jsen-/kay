use super::{PositionIterator, Vars};
use std::io::{self, Read, Write};

enum Intent {
    Continue,
    Stop,
}

enum ExprType {
    Env,
    Var,
}

fn translate_expr<R: Iterator<Item = io::Result<char>>, W: Write>(
    input_chars: &mut R,
    output: &mut W,
    vars: &Box<dyn Vars>,
) -> io::Result<()> {
    let mut s = String::new();
    loop {
        match input_chars.next() {
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid expression: unexpected end of input",
                ))
            }
            Some(Err(e)) => return Err(e),
            Some(Ok(ch)) if ch == '}' => break,
            Some(Ok(ch)) => s.push(ch),
        }
    }

    if s.bytes().len() < 4 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid expression: unknown expression type",
        ));
    }

    let (expr_type, expr_path) = if &s[0..4] == "env " {
        (ExprType::Env, &s[4..])
    } else if &s[0..4] == "var " {
        (ExprType::Var, &s[4..])
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid expression: unknown expression type",
        ));
    };
    match expr_type {
        ExprType::Var => match vars.get(expr_path) {
            Some(value) => {
                output.write(value.as_bytes())?;
                Ok(())
            }
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    r#"Invalid expression: expression "{}" not found in vars"#,
                    expr_path
                ),
            )),
        },
        ExprType::Env => match std::env::var_os(expr_path) {
            Some(value) => {
                use std::os::unix::ffi::OsStrExt; // linux only for now ... sorry
                output.write(value.as_bytes())?;
                Ok(())
            }
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    r#"Invalid expression: environment variable "{}" not found"#,
                    expr_path
                ),
            )),
        },
    }
}

pub fn translate<R: Iterator<Item = io::Result<char>>, W: Write>(
    input_chars: &mut R,
    output: &mut W,
    vars: &Box<dyn Vars>,
) -> io::Result<()> {
    let mut slash = false;
    let mut dollar = false;

    while let Some(rch) = input_chars.next() {
        match rch {
            Err(e) => return Err(e),
            Ok(ch) => {
                if slash {
                    if ch == '$' {
                        output.write(b"$")?;
                    } else if ch == '\\' {
                        output.write(b"\\\\")?;
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
                    output.write(s.as_bytes())?;
                }
            }
        }
    }
    if slash {
        output.write(b"\\")?;
    } else if dollar {
        output.write(b"$")?;
    }

    Ok(())
}
