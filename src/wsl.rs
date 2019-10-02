use std::os::unix::process::CommandExt;
use std::path::{Path};
use std::process::Command;

use crate::{MkLink};
use crate::error::{Error};

fn wsl_to_windows_path(path: &Path) -> Result<String, Error> {
    let program = "wslpath";
    Command::new(program)
        .arg("-m")
        .arg(path.as_os_str())
        .output()
        .map_err(Error::for_program(program))
        .map(|it| it.stdout)
        .map(String::from_utf8)
        .map(|it| it.unwrap())
        .map(|it| it.trim().into())
}

impl MkLink {
    pub fn run(&self) -> Result<(), Error> {
        let program = "./target/debug/mklink.exe";
        let mut cmd = Command::new(program);
        if self.hard {
            cmd.arg("-h");
        }
        if self.file {
            cmd.arg("-f");
        }
        if self.dir {
            cmd.arg("-d");
        }
        for path in [&self.target, &self.link].iter() {
            cmd.arg(wsl_to_windows_path(path.as_path())?);
        }
        Err(cmd.exec())
            .map_err(Error::for_program(program))
    }
}
