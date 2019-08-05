use std::path::PathBuf;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::fs::File;

use super::Error;

#[derive(Debug)]
pub enum InputStream<'a> {
    File(PathBuf, BufReader<File>),
    Stdin(io::StdinLock<'a>),
}

impl<'a> InputStream<'a> {
    pub fn from_file(path: PathBuf) -> Result<InputStream<'a>, Error<'a>> {
        let file = File::open(&path)
            .map_err(|e| Error::FileRead(path.clone(), e))?;
        Ok(InputStream::File(path, BufReader::new(file)))
    }
    pub fn from_stdin(stdin: &'a io::Stdin) -> InputStream<'a> {
        InputStream::Stdin(stdin.lock())
    }
}
impl<'a> Read for InputStream<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::File(_, file) => file.read(buf),
            Self::Stdin(stdin) => stdin.read(buf),
        }
    }
}

#[derive(Debug)]
pub enum OutputStream<'a> {
    File(PathBuf, BufWriter<File>),
    Stdout(io::StdoutLock<'a>),
}
impl<'a> OutputStream<'a> {
    pub fn from_file(path: PathBuf) -> Result<OutputStream<'a>, Error<'a>> {
        let file = File::open(&path)
            .map_err(|e| Error::FileWrite(path.clone(), e))?;
        Ok(OutputStream::File(path, BufWriter::new(file)))
    }
    pub fn from_stdin(stdout: &io::Stdout) -> OutputStream {
        OutputStream::Stdout(stdout.lock())
    }
}
impl<'a> Write for OutputStream<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::File(_, file) => file.write(buf),
            Self::Stdout(stdin) => stdin.write(buf),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::File(_, file) => file.flush(),
            Self::Stdout(stdin) => stdin.flush(),
        }
    }
}