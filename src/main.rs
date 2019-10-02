use std::io;
use std::path::PathBuf;

use structopt::StructOpt;

#[cfg(windows)] mod windows;
#[cfg(unix)] mod wsl;

#[derive(Debug, StructOpt)]
struct MkLink {
    #[structopt(short, long)]
    hard: bool,
    #[structopt(short, long)]
    file: bool,
    #[structopt(short, long)]
    dir: bool,
    target: PathBuf,
    link: PathBuf,
}

#[derive(Debug)]
struct IOError {
    path: Option<PathBuf>,
    error: io::Error,
}

impl From<io::Error> for IOError {
    fn from(error: io::Error) -> Self {
        IOError {
            path: None,
            error,
        }
    }
}

impl IOError {
    fn for_cmd<'a>(program: &'a str) -> impl (Fn(io::Error) -> IOError) + 'a {
        move |error| IOError {
            path: Some(PathBuf::from(program)),
            error,
        }
    }
}

#[derive(Debug)]
enum Error {
    Message(String),
    IO(IOError),
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Self {
        Error::IO(error)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Message(msg)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Message(msg.into())
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        IOError::from(e).into()
    }
}

#[paw::main]
fn main(mk_link: MkLink) -> Result<(), Error> {
    println!("{:?}", mk_link);
    mk_link.run()?;
    Ok(())
}
