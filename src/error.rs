use std::{io, fmt};
use std::path::Path;
use std::fmt::{Display, Formatter, Write};

#[derive(Debug)]
pub struct Error<'a> {
    pub message: Option<&'a str>,
    pub path: Option<&'a Path>,
    pub program: Option<&'a str>,
    pub error: Option<io::Error>,
}

struct DisplayOption<'a, T : Display> {
    name: &'a str,
    option: &'a Option<T>,
}

impl<'a, T : Display> Display for DisplayOption<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        if let Some(value) = self.option {
            f.write_str(self.name)?;
            f.write_str(": ")?;
            value.fmt(f)?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        DisplayOption {name: "message", option: &self.message}.fmt(f)?;
        DisplayOption {name: "path", option: &self.path.map(|it| it.display())}.fmt(f)?;
        DisplayOption {name: "program", option: &self.program}.fmt(f)?;
        DisplayOption {name: "error", option: &self.error}.fmt(f)?;
        Ok(())
    }
}

#[allow(dead_code)] // needed b/c different OS's use different methods
impl<'a> Error<'a> {
    pub fn with_msg(msg: &str) -> Error {
        Error {
            message: Some(msg),
            path: None,
            program: None,
            error: None,
        }
    }
    
    pub fn with_msg_and_path<'b>(msg: &'b str, path: &'b Path) -> Error<'b> {
        Error {
            message: Some(msg),
            path: Some(path),
            program: None,
            error: None,
        }
    }
    pub fn for_program<'b>(program: &'b str) -> impl (Fn(io::Error) -> Error<'b>) + 'b {
        move |error| Error {
            message: None,
            path: None,
            program: Some(program),
            error: Some(error),
        }
    }
    
    pub fn err<T>(self) -> Result<T, Error<'a>> {
        Err(self)
    }
}

impl From<io::Error> for Error<'_> {
    fn from(error: io::Error) -> Self {
        Error {
            message: None,
            path: None,
            program: None,
            error: Some(error),
        }
    }
}
