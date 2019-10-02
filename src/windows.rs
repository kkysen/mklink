use crate::{MkLink};
use std::path::{Path};
use std::os::windows::fs::{symlink_file, symlink_dir};
use std::fs::hard_link;
use std::io;
use crate::error::Error;

enum LinkType {
    File,
    Directory,
    Hard,
    Junction,
}

impl LinkType {
    fn new(is_hard: bool, is_file: bool) -> LinkType {
        if is_file {
            if is_hard {
                LinkType::Hard
            } else {
                LinkType::File
            }
        } else {
            if is_hard {
                LinkType::Junction
            } else {
                LinkType::Directory
            }
        }
    }
    
    fn creator(&self) -> fn(&Path, &Path) -> io::Result<()> {
        // need closures since these functions have generic lifetimes, closures monomorphize them
        match self {
            LinkType::File => |a, b| symlink_file(a, b),
            LinkType::Directory => |a, b| symlink_dir(a, b),
            LinkType::Hard => |a, b| hard_link(a, b),
            LinkType::Junction => |a, b| junction::create(a, b),
        }
    }
    
    fn name(&self) -> &'static str {
        match self {
            LinkType::File => "symbolic file link",
            LinkType::Directory => "symbolic directory link",
            LinkType::Hard => "hard link",
            LinkType::Junction => "directory junction",
        }
    }
}

impl MkLink {
    fn check_own_constrains(&self) -> Result<(), Error> {
        if self.file && self.dir {
            return Error::with_msg("can't specify both --file and --dir").err()
        }
        Ok(())
    }
    
    fn get_link_type(&self, is_file: bool) -> LinkType {
        LinkType::new(self.hard, is_file)
    }
    
    fn err<'a>(&'a self, msg: &'a str, path: &'a Path) -> Error<'a> {
        Error::with_msg_and_path(msg, path)
    }
    
    fn target_err<'a>(&'a self, msg: &'a str) -> Error<'a> {
        self.err(msg, &self.target)
    }
    
    fn link_err<'a>(&'a self, msg: &'a str) -> Error<'a> {
        self.err(msg, &self.link)
    }
    
    fn check_target_and_get_link_type(&self) -> Result<LinkType, Error> {
        let err = |msg| self.target_err(msg);
        match self.target.metadata() {
            Err(_) => {
                if !self.file && !self.dir {
                    err("can't infer target file type since target doesn't exist").err()
                } else {
                    Ok(self.get_link_type(self.file))
                }
            },
            Ok(metadata) => {
                let is_file = metadata.is_file();
                let is_dir = metadata.is_dir();
                if self.file && !is_file {
                    err("specified --file but target is not a file").err()
                } else if self.dir && !is_dir {
                    err("specified --dir but target is not a directory").err()
                } else if !(is_file || is_dir) {
                    err("only works on files and directories").err()
                } else {
                    Ok(self.get_link_type(is_file))
                }
            },
        }
    }
    
    fn check_link(&self) -> Result<(), Error> {
        if self.link.exists() {
            return self.link_err("link already exists").err()
        }
        Ok(())
    }
    
    pub fn run(&self) -> Result<(), Error> {
        self.check_own_constrains()?;
        let link_type = self.check_target_and_get_link_type()?;
        self.check_link()?;
        let target = self.target.as_path();
        let link = self.link.as_path();
        link_type.creator()(target, link)?;
        eprintln!("created a {}: \"{}\" -> \"{}\"", link_type.name(), link.display(), target.display());
        Ok(())
    }
}
