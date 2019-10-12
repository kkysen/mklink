use std::io;

use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkError<E> {
    pub error: E,
    pub target: bool,
    pub link: bool,
    pub program: Option<String>,
}

impl<E> LinkError<E> {
    pub fn map<E2, F: FnOnce(E) -> E2>(self, f: F) -> LinkError<E2> {
        LinkError {
            error: f(self.error),
            target: self.target,
            link: self.link,
            program: self.program,
        }
    }
    
    pub fn both(error: E) -> LinkError<E> {
        LinkError {
            error,
            target: true,
            link: true,
            program: None,
        }
    }
    
    pub fn target(error: E) -> LinkError<E> {
        LinkError {
            error,
            target: true,
            link: false,
            program: None,
        }
    }
    
    pub fn link(error: E) -> LinkError<E> {
        LinkError {
            error,
            target: false,
            link: true,
            program: None,
        }
    }
    
    pub fn none(error: E) -> LinkError<E> {
        LinkError {
            error,
            target: false,
            link: false,
            program: None,
        }
    }
    
    pub fn program(error: E, program: &str) -> LinkError<E> {
        LinkError {
            error,
            target: false,
            link: false,
            program: Some(program.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PreLinkError {
    LinkFileTypeMismatch,
    InvalidFileType,
    InferredNonExistentTarget,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DuringLinkError {
    LinkAlreadyExists,
    OS(OSError),
}

#[derive(Debug)]
pub struct OSError(pub io::Error);

impl Serialize for OSError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        self.0
            .raw_os_error()
            .map(|it| it.serialize(serializer))
            .expect("io::Error doesn't have an errno")
    }
}

impl<'de> Deserialize<'de> for OSError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        i32::deserialize(deserializer)
            .map(|it| io::Error::from_raw_os_error(it))
            .map(|it| OSError(it))
    }
}
