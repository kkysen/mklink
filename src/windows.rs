use crate::{LinkType, MkLink, LinkError, DuringLinkError};
use std::path::Path;
use std::io;
use std::fs::hard_link;
use std::os::windows::fs::{symlink_dir, symlink_file};
use crate::link_error::OSError;

impl LinkType {
    fn creator(&self) -> fn(&Path, &Path) -> io::Result<()> {
        // need closures since these functions have generic lifetimes, closures monomorphize them
        match self {
            LinkType::File => |a, b| symlink_file(a, b),
            LinkType::Directory => |a, b| symlink_dir(a, b),
            LinkType::Hard => |a, b| hard_link(a, b),
            LinkType::Junction => |a, b| junction::create(a, b),
        }
    }
}

impl<'a> MkLink<'a> {
    pub(crate) fn create_impl(&self) -> Result<(), LinkError<DuringLinkError>> {
        let link = self.link;
        self.link_type
            .creator()
            (link.target, link.link)
            .map_err(|it| DuringLinkError::OS(OSError(it)))
            .map_err(|it| LinkError::both(it))
    }
}
