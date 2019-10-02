use std::path::PathBuf;

use structopt::StructOpt;
use std::io;

mod error;
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

#[paw::main]
fn main(mk_link: MkLink) -> Result<(), io::Error> {
    if let Err(e) = mk_link.run() {
        eprint!("{}", e);
    }
    Ok(())
}
