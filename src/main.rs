#[macro_use]
extern crate std;

use std::io;
use std::io::stdout;
use std::path::PathBuf;
use std::process::exit;

use structopt::StructOpt;

use serde::{Serialize, Deserialize};
use mklink::{MkLink, mklink};
use mklink::link_error::{PreLinkError, DuringLinkError, LinkError};
use mklink::link_type::LinkHardness::{Hard, Soft};
use mklink::link_type::LinkFileType::{File, Directory};

#[derive(Debug, StructOpt)]
#[structopt(
name = env ! ("CARGO_PKG_NAME"),
version = env ! ("CARGO_PKG_VERSION"),
author = env ! ("CARGO_PKG_AUTHORS"),
about = env ! ("CARGO_PKG_DESCRIPTION"),
)]
struct MkLinkArgs {
    #[structopt(short, long)]
    hard: bool,
    #[structopt(short, long)]
    file: bool,
    #[structopt(short, long)]
    dir: bool,
    #[structopt(long)]
    raw: bool,
    target: PathBuf,
    link: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
enum AnyLinkError {
    InvalidArgs(),
    Pre(PreLinkError),
    During(DuringLinkError),
}

struct Error(LinkError<AnyLinkError>);

impl Error {
    fn print(self, raw: bool) {
        match raw {
            true => self.raw_print(),
            false => self.pretty_print(),
        }
    }
    
    fn raw_print(self) {
        let e = self.0.map(|it| match it {
            AnyLinkError::During(e) => e,
            _ => {
                Error(LinkError::none(it)).pretty_print();
                panic!("Invalid Error for --raw mode")
            }
        });
        // serialize LinkError<DuringLinkError>
        // other errors are checked on WSL side first
        bincode::serialize_into(stdout(), &e)
            .expect("serialization failed");
    }
    
    fn pretty_print(self) {
        let e = self.0;
        eprintln!("{:?}", e);
    }
}

impl MkLinkArgs {
    fn as_mk_link(&self) -> Result<MkLink, LinkError<AnyLinkError>> {
        if self.file && self.dir {
            return Err(LinkError::none(AnyLinkError::InvalidArgs()));
        }
        let hardness = match self.hard {
            true => Hard,
            false => Soft,
        };
        let file_type = match self.file {
            true => match self.dir {
                true => return Err(LinkError::none(AnyLinkError::InvalidArgs())),
                false => Some(File),
            },
            false => match self.dir {
                true => Some(Directory),
                false => None,
            },
        };
        let link = mklink(self.target.as_path(), self.link.as_path());
        let link = link.with_hardness(hardness);
        let link = match self.raw {
            true => Ok(link.with_type_unchecked(file_type.expect("--raw requires a file type"))),
            false => link.maybe_with_type(file_type),
        };
        link.map_err(|it| it.map(|it| AnyLinkError::Pre(it)))
    }
    
    fn run(&self) -> Result<MkLink, LinkError<AnyLinkError>> {
        let mk_link = self.as_mk_link()?;
        mk_link.create()
            .map_err(|it| it.map(|it| AnyLinkError::During(it)))?;
        Ok(mk_link)
    }
    
    fn print(&self, mk_link: &MkLink) {
        println!("created a {}: \"{}\" -> \"{}\"",
                 mk_link.link_type.name(),
                 mk_link.link.target.display(),
                 mk_link.link.link.display(),
        )
    }
    
    fn handled_run(&self) -> ! {
        let exit_code = match self.run() {
            Ok(mk_link) => {
                self.print(&mk_link);
                0
            }
            Err(e) => {
                Error(e).print(self.raw);
                1
            }
        };
        exit(exit_code);
    }
}

#[paw::main]
fn main(mk_link: MkLinkArgs) -> Result<(), io::Error> {
    mk_link.handled_run()
}
