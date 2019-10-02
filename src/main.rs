use std::path::PathBuf;

use structopt::StructOpt;
use std::io;

#[macro_use]
extern crate std;

mod error;
#[cfg(windows)] mod windows;
#[cfg(unix)] mod wsl;

#[derive(Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = cfg,
)]
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

#[paw::main]
fn main(mk_link: MkLink) -> Result<(), io::Error> {
    if let Err(e) = mk_link.run() {
        eprint!("{}", e);
    }
    Ok(())
}
