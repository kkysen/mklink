use std::{io, fmt};
use std::path::Path;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error<'a> {
    pub message: Option<&'a str>,
    pub paths: Vec<&'a Path>,
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
        if self.paths.len() <= 1 {
            DisplayOption {name: "path", option: &self.paths
                .get(0)
                .map(|it| it.display())
            }.fmt(f)?;
        } else {
            DisplayOption {name: "paths", option: &Some(
                format!("{:?}", self.paths
                    .iter()
                    .map(|it| it.display())
                    .collect::<Vec<_>>()
                )
            )}.fmt(f)?;
        }
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
            paths: Vec::new(),
            program: None,
            error: None,
        }
    }
    
    pub fn with_msg_and_path<'b>(msg: &'b str, path: &'b Path) -> Error<'b> {
        Error {
            message: Some(msg),
            paths: vec![path],
            program: None,
            error: None,
        }
    }
    
    pub fn with_msg_and_program<'b>(msg: &'b str, program: &'b str, paths: Vec<&'b Path>) -> Error<'b> {
        Error {
            message: Some(msg),
            paths,
            program: Some(program),
            error: None,
        }
    }
    
    pub fn with_error(error: io::Error, paths: Vec<&Path>) -> Error {
        Error {
            message: None,
            paths,
            program: None,
            error: Some(error),
        }
    }
    
    pub fn for_program<'b>(program: &'b str) -> impl (Fn(io::Error) -> Error<'b>) + 'b {
        move |error| Error {
            message: None,
            paths: Vec::new(),
            program: Some(program),
            error: Some(error),
        }
    }
    
    pub fn err<T>(self) -> Result<T, Error<'a>> {
        Err(self)
    }
}
