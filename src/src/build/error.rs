use glob::{GlobError, PatternError};
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::{env, io};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("prologue not found")]
    PrologueNotFound,
    #[error("{0}:{1}: `{{` is missing")]
    ExprNotOpened(usize, usize),
    #[error("{0}:{1}: `}}` is missing")]
    ExprNotClosed(usize, usize),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}: invalid unicode sequence in path")]
    PathEncoding(PathBuf),
    #[error("{0}: file stem not found in path")]
    NoFileStem(PathBuf),
    #[error(transparent)]
    Pattern(#[from] PatternError),
    #[error(transparent)]
    Glob(#[from] GlobError),
    #[error("{0}: {1}")]
    IO(PathBuf, io::Error),
    #[error(transparent)]
    Var(#[from] env::VarError),
    #[error("{0}:{1}")]
    Parse(PathBuf, ParseError),
}

pub struct Errors(Vec<Error>);

impl From<Vec<Error>> for Errors {
    fn from(errors: Vec<Error>) -> Self {
        Self(errors)
    }
}

impl<T> From<T> for Errors
where
    T: Into<Error>,
{
    fn from(value: T) -> Self {
        Self(vec![value.into()])
    }
}

impl Debug for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for Errors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in &self.0 {
            f.write_fmt(format_args!("error: {}\n", x))?;
        }

        Ok(())
    }
}

impl std::error::Error for Errors {}
