use std::io;
use std::path::PathBuf;

use structopt::StructOpt;

#[cfg(windows)] mod windows;
#[cfg(unix)] mod wsl;

#[derive(Debug, StructOpt)]
struct MkLink {
    #[structopt(short, long)]
    hard: bool,
    target: PathBuf,
    link: PathBuf,
}

#[derive(Debug)]
struct Error {
    path: Option<PathBuf>,
    error: io::Error,
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            path: None,
            error,
        }
    }
}

impl Error {
    fn for_cmd<'a>(program: &'a str) -> impl (Fn(io::Error) -> Error) + 'a {
        move |error| Error {
            path: Some(PathBuf::from(program)),
            error,
        }
    }
}

#[paw::main]
fn main(mk_link: MkLink) -> Result<(), Error> {
    println!("{:?}", mk_link);
    mk_link.run()?;
    Ok(())
}
