use crate::flag::{get_flags, CommandFlag};
use std::{fmt::Display, ops::Add, rc::Rc};

pub trait DataTrait {
    fn incr_files(&mut self);
    fn incr_dirs(&mut self);
    fn incr_depth(&mut self);
    fn set_flags(&mut self, flags: Rc<[CommandFlag]>);
    fn is_help(&self) -> bool;
    fn is_all(&self) -> bool;
    fn is_file_only(&self) -> bool;
    fn is_directory_only(&self) -> bool;
    fn validate_flags(&self) -> Result<(), &Rc<str>>;
}

#[derive(Debug)]
pub struct Data {
    flags: Rc<[CommandFlag]>,
    depth: usize,
    files: usize,
    dirs: usize,
}

impl Data {
    pub fn new(flags: Rc<[CommandFlag]>) -> Self {
        Self {
            flags,
            depth: 0,
            files: 0,
            dirs: 0,
        }
    }

    fn depth(&self) -> usize {
        self.depth
    }

    fn files(&self) -> usize {
        self.files
    }

    fn dirs(&self) -> usize {
        self.dirs
    }
}

impl DataTrait for Data {
    fn incr_files(&mut self) {
        self.files = self.files.add(1);
    }

    fn incr_dirs(&mut self) {
        self.dirs = self.dirs.add(1);
    }

    fn incr_depth(&mut self) {
        self.depth = self.depth.add(1);
    }

    fn set_flags(&mut self, flags: Rc<[CommandFlag]>) {
        self.flags = flags;
    }

    fn is_help(&self) -> bool {
        self.flags.first().eq(&Some(&CommandFlag::Help))
    }

    fn is_all(&self) -> bool {
        self.flags.contains(&CommandFlag::All)
    }

    fn is_file_only(&self) -> bool {
        self.flags.contains(&CommandFlag::FileOnly)
    }

    fn is_directory_only(&self) -> bool {
        self.flags.contains(&CommandFlag::DirOnly)
    }

    fn validate_flags(&self) -> Result<(), &Rc<str>> {
        for flag in self.flags.iter() {
            match flag {
                CommandFlag::Unknown(f) => return Err(f),
                _ => continue,
            }
        }

        Ok(())
    }
}

impl Default for Data {
    fn default() -> Self {
        let flags = get_flags();

        Self::new(flags)
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (dirs, files) = (self.dirs(), self.files());
        let output = format!("{dirs} directories, {files} files");

        write!(f, "{}", output.as_str())
    }
}
