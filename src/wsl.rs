use crate::{MkLink, LinkError, DuringLinkError};
use std::path::Path;
use std::process::{Command, Stdio};
use std::io;
use crate::LinkFileType::{Directory, File};
use crate::link_error::OSError;

fn program_err(error: io::Error, program: &str) -> LinkError<DuringLinkError> {
    LinkError::program(error, program)
        .map(|it| OSError(it))
        .map(|it| DuringLinkError::OS(it))
}

fn wsl_to_windows_path(path: &Path) -> Result<String, LinkError<DuringLinkError>> {
    let program = "wslpath";
    Command::new(program)
        .arg("-m")
        .arg(path.as_os_str())
        .output()
        .map_err(|it| program_err(it, program))
        .map(|it| it.stdout)
        .map(String::from_utf8)
        .map(|it| it.unwrap())
        .map(|it| it.trim().into())
}

impl<'a> MkLink<'a> {
    pub(crate) fn create_impl(&self) -> Result<(), LinkError<DuringLinkError>> {
        let program = "mklink.exe";
        let mut cmd = Command::new(program);
        let link_type = self.link_type;
        let link = self.link;
        if link_type.is_hard() {
            cmd.arg("-h");
        }
        cmd.arg(match link_type.file_type() {
            File => "-f",
            Directory => "-d",
        });
        cmd.arg("--raw");
        for path in [link.target, link.link].iter() {
            cmd.arg(wsl_to_windows_path(path)?);
        }
        cmd.stdout(Stdio::piped());
        let child = cmd.spawn()
            .map_err(|it| program_err(it, program))?;
        let output = child.wait_with_output()
            .map_err(|it| program_err(it, program))?;
        if output.status.success() {
            return Ok(());
        }
        let error: LinkError<DuringLinkError> = bincode::deserialize(output.stdout.as_slice())
            .expect("deserialization error");
        Err(error)
    }
}
