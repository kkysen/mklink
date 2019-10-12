use std::fs::FileType;
use crate::link_type::LinkFileType::{File, Directory};
use crate::link_type::LinkHardness::{Soft, Hard};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LinkHardness {
    Soft,
    Hard,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LinkFileType {
    File,
    Directory,
}

impl LinkFileType {
    pub fn from(file_type: FileType) -> Option<LinkFileType> {
        if file_type.is_file() || file_type.is_symlink() {
            Some(File)
        } else if file_type.is_dir() {
            Some(Directory)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LinkType {
    File,
    Directory,
    Hard,
    Junction,
}

impl LinkType {
    pub fn hardness(&self) -> LinkHardness {
        match self {
            LinkType::File => Soft,
            LinkType::Directory => Soft,
            LinkType::Hard => Hard,
            LinkType::Junction => Hard,
        }
    }
    
    pub fn file_type(&self) -> LinkFileType {
        match self {
            LinkType::File => File,
            LinkType::Directory => Directory,
            LinkType::Hard => File,
            LinkType::Junction => Directory,
        }
    }
    
    pub fn is_hard(&self) -> bool {
        self.hardness() == Hard
    }
    
    pub fn is_soft(&self) -> bool {
        self.hardness() == Soft
    }
    
    pub fn is_file(&self) -> bool {
        self.file_type() == File
    }
    
    pub fn is_dir(&self) -> bool {
        self.file_type() == Directory
    }
    
    pub fn new(hardness: LinkHardness, file_type: LinkFileType) -> LinkType {
        match hardness {
            Soft => match file_type {
                File => LinkType::File,
                Directory => LinkType::Directory,
            },
            Hard => match file_type {
                File => LinkType::Hard,
                Directory => LinkType::Junction,
            },
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            LinkType::File => "symbolic file link",
            LinkType::Directory => "symbolic directory link",
            LinkType::Hard => "hard link",
            LinkType::Junction => "directory junction",
        }
    }
}
