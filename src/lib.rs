use std::path::{Path, PathBuf};

use crate::link_type::{LinkHardness, LinkType, LinkFileType};
use crate::link_error::{LinkError, PreLinkError, DuringLinkError};

pub mod link_type;
pub mod link_error;

#[cfg(windows)]
mod windows;

#[cfg(not(windows))]
mod wsl;

pub fn mklink<'a>(target: &'a Path, link: &'a Path) -> Link<'a> {
    Link { target, link }
}

#[derive(Clone, Copy)]
pub struct Link<'a> {
    pub target: &'a Path,
    pub link: &'a Path,
}

impl<'a> Link<'a> {
    pub fn with_hardness(&self, hardness: LinkHardness) -> HardenedLink<'a> {
        HardenedLink { link: *self, hardness }
    }
    
    pub fn with_type(&self, link_type: LinkType) -> Result<MkLink, LinkError<PreLinkError>> {
        self.with_hardness(link_type.hardness())
            .with_type(link_type.file_type())
    }
}

#[derive(Clone, Copy)]
pub struct HardenedLink<'a> {
    pub link: Link<'a>,
    pub hardness: LinkHardness,
}

impl<'a> HardenedLink<'a> {
    pub fn with_type_unchecked(&self, file_type: LinkFileType) -> MkLink<'a> {
        let this = *self;
        MkLink {
            link: this.link,
            link_type: LinkType::new(this.hardness, file_type),
        }
    }
    
    fn get_inferred_file_type(&self) -> Result<LinkFileType, PreLinkError> {
        match self.link.target.metadata() {
            Ok(metadata) => {
                match LinkFileType::from(metadata.file_type()) {
                    Some(file_type) => Ok(file_type),
                    None => Err(PreLinkError::InvalidFileType)
                }
            }
            Err(_) => Err(PreLinkError::InferredNonExistentTarget),
        }
    }
    
    pub fn with_type(&self, file_type: LinkFileType) -> Result<MkLink<'a>, LinkError<PreLinkError>> {
        match self.get_inferred_file_type() {
            Ok(real_file_type) => {
                if real_file_type == file_type {
                    Ok(self.with_type_unchecked(file_type))
                } else {
                    Err(PreLinkError::LinkFileTypeMismatch)
                }
            }
            Err(PreLinkError::InferredNonExistentTarget) => Ok(self.with_type_unchecked(file_type)),
            Err(e) => Err(e),
        }.map_err(|it| LinkError::target(it))
    }
    
    pub fn infer_type(&self) -> Result<MkLink<'a>, LinkError<PreLinkError>> {
        self.get_inferred_file_type()
            .map(|it| self.with_type_unchecked(it))
            .map_err(|it| LinkError::target(it))
    }
    
    pub fn maybe_with_type(&self, file_type: Option<LinkFileType>) -> Result<MkLink<'a>, LinkError<PreLinkError>> {
        match file_type {
            None => self.infer_type(),
            Some(file_type) => self.with_type(file_type),
        }
    }
}

#[derive(Clone, Copy)]
pub struct MkLink<'a> {
    pub link: Link<'a>,
    pub link_type: LinkType,
}

impl<'a> MkLink<'a> {
    fn resolve_link(&self) -> Result<PathBuf, LinkError<DuringLinkError>> {
        let link = self.link.link;
        let e = Err(LinkError::link(DuringLinkError::LinkAlreadyExists));
        match link.metadata() {
            Ok(metadata) => match metadata.is_dir() {
                true => match self.link.target.file_name() {
                    Some(file_name) => {
                        let link = link.join(file_name);
                        match link.exists() {
                            true => e,
                            false => Ok(link),
                        }
                    },
                    None => e,
                },
                false => e,
            },
            Err(_) => Ok(link.to_path_buf()),
        }
    }
    
    pub fn create_and<F: FnOnce(&MkLink)>(&self, after: F) -> Result<(), LinkError<DuringLinkError>> {
        let link = self.resolve_link()?;
        let mk_link = MkLink {
            link: Link {
                target: self.link.target,
                link: link.as_path(),
            },
            link_type: self.link_type,
        };
        mk_link.create_impl()?;
        after(&mk_link);
        Ok(())
    }
    
    pub fn create(&self) -> Result<(), LinkError<DuringLinkError>> {
        self.create_and(|_| {})
    }
}
