use std::fmt::{Display, Error, Formatter, Write};
use std::os::unix::process::CommandExt;
use std::path::{Component, PathBuf};
use std::process::Command;

use itertools::Itertools;
use std::ffi::CString;

struct Flag {
    arg: Option<String>,
}

impl Flag {
    fn is_flag(arg: &String) -> bool {
        arg.starts_with("/")
    }
    fn new(arg: String) -> Flag {
        if Flag::is_flag(&arg) {
            Flag { arg: Some(arg) }
        } else {
            Flag { arg: None }
        }
    }
    fn none() -> Flag {
        Flag { arg: None }
    }
}

impl Display for Flag {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if let Some(flag) = &self.arg {
            f.write_str(flag.as_str())?;
        }
        Ok(())
    }
}

struct PathArg {
    path: PathBuf,
}

impl PathArg {
    fn new(path: String) -> PathArg {
        PathArg { path: PathBuf::from(path) }
    }
    
    fn drive(&self) -> Option<(String, usize)> {
        if !self.path.starts_with("/mnt/") {
            ()
        }
        let drive = self.path.components().nth(2)?;
        let drive = drive.as_os_str().to_str()?;
        if !drive.chars().all(|c| c.is_ascii_alphabetic()) {
            ()
        }
        let drive = drive.to_ascii_uppercase();
        Some((drive, 3))
    }
    
    fn prefix(&self) -> (String, usize) {
        match self.drive() {
            None => ("".into(), 0),
            Some((drive, skipped)) => (format!("{}:\\", drive), skipped),
        }
    }
}

impl Display for PathArg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let (prefix, skipped) = self.prefix();
        f.write_str(prefix.as_str())?;
        self.path.components()
            .skip(skipped)
            .map(|component| match component {
                Component::Prefix(prefix) => prefix.as_os_str().to_str().unwrap(),
                Component::RootDir => panic!("cannot convert WSL path to Windows path"),
                Component::CurDir => ".",
                Component::ParentDir => "..",
                Component::Normal(s) => s.to_str().unwrap(),
            })
            .intersperse("\\")
            .map(|s| f.write_str(s))
            .fold_results((), |_, e| e)?;
        Ok(())
    }
}

struct QuotedArg {
    arg: String,
}

impl Display for QuotedArg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_char('"')?;
        f.write_str(self.arg.as_str())?;
        f.write_char('"')?;
        Ok(())
    }
}

enum Arg {
    Flag(Flag),
    Path(PathArg),
    #[allow(dead_code)] Quoted(QuotedArg),
    #[allow(dead_code)] UnQuoted(String),
}

impl Display for Arg {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Arg::Flag(arg) => arg.fmt(f)?,
            Arg::Path(arg) => arg.fmt(f)?,
            Arg::Quoted(arg) => arg.fmt(f)?,
            Arg::UnQuoted(arg) => f.write_str(arg.as_str())?,
        };
        Ok(())
    }
}

fn quote(s: String) -> String {
    let quote = s.contains(" ");
    if quote {
        format!("\"{}\"", s)
    } else {
        s
    }
}

struct Args {
    flag: Flag,
    link: String,
    target: String,
}

impl Args {
    fn new() -> Option<Args> {
        let mut args = std::env::args().peekable();
        if args.len() > 4 {
            ()
        }
        args.next()?;
        let flag = if args.peek().map_or(false, Flag::is_flag) {
            Flag::new(args.next().unwrap())
        } else {
            Flag::none()
        };
        let link = args.next()?;
        let target = args.next()?;
        Some(Args {
            flag,
            link,
            target,
        })
    }
    
    fn args(self) -> [Arg; 3] {
        [
            Arg::Flag(self.flag),
            Arg::Path(PathArg::new(self.link)),
            Arg::Path(PathArg::new(self.target)),
        ]
    }
}

struct MkLink {
    args: [Arg; 3],
}

impl MkLink {
    fn new(args: Args) -> MkLink {
        MkLink {args: args.args()}
    }
    
    fn cmd<'a>(&'a self) -> impl Iterator<Item = String> + 'a {
        self.args
            .iter()
            .map(|arg| format!("{}", arg))
            .map(quote)
    }
}

pub fn run() {
    let args = Args::new()
        .map(MkLink::new)
        .map_or(vec![], |it| it.cmd().collect_vec());
    println!("{:?}", Command::new("cmd.exe")
        .arg("/C")
        .arg("mklink")
        .args(Args::new()
            .map(MkLink::new)
            .map_or(vec![], |it| it.cmd().collect_vec())));
//    Command::new("cmd.exe")
//        .arg("/C")
//        .arg("mklink")
//        .args(args)
//        .exec();
    
    let f = |s: &str| CString::new(s).unwrap();
    
    nix::unistd::execvp(&f("cmd.exe"), &[
        f("cmd.exe"),
        f("/C"),
        f("mklink"),
        f("/J"),
        f("hello"),
        f("\"C:\\Users\\Khyber\\OneDrive\\Khyber\\Documents\\Columbia\\Sophomore\\Fall\\Contemporary Civilization - Marwa Elshakry\""),
    ]);
}
