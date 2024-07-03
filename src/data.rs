use std::{fmt::Display, ops::Add, rc::Rc};

const OUTPUT: &str = r#"
{dirs} directories, {files} files
"#;

use crate::flag::{get_flags, CommandFlag};

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

    fn incr_files(&mut self) {
        self.files = self.files.add(1);
    }

    fn incr_dirs(&mut self) {
        self.dirs = self.dirs.add(1);
    }

    fn incr_depth(&mut self) {
        self.depth = self.depth.add(1);
    }

    pub fn set_flags(&mut self, flags: Rc<[CommandFlag]>) {
        self.flags = flags;
    }

    pub fn is_help(&self) -> bool {
        self.flags.get(0).eq(&Some(&CommandFlag::Help))
    }

    pub fn is_all(&self) -> bool {
        self.flags.contains(&CommandFlag::All)
    }

    pub fn is_file_only(&self) -> bool {
        self.flags.contains(&CommandFlag::FileOnly)
    }

    pub fn is_directory_only(&self) -> bool {
        self.flags.contains(&CommandFlag::DirOnly)
    }

    pub fn validate_flags(&self) -> Result<(), &Rc<str>> {
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
