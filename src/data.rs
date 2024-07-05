use crate::flag::{get_flags, Arg};
use std::{fmt::Display, sync::Arc};

#[derive(Debug)]
pub struct Data {
    flags: Arc<[Arg]>,
    pub files: usize,
    pub dirs: usize,
}

impl Data {
    pub fn new(flags: Arc<[Arg]>) -> Self {
        Self {
            flags,
            files: 0,
            dirs: 0,
        }
    }

    pub fn files(&self) -> usize {
        self.files
    }

    pub fn dirs(&self) -> usize {
        self.dirs
    }

    pub fn get_path(&self) -> String {
        let mut path = self.flags.iter().filter_map(|f| match f {
            Arg::Path(p) => Some(p),
            _ => None,
        });

        if let Some(path) = path.next() {
            path.to_string_lossy().into_owned()
        } else {
            "".to_string()
        }
    }

    pub fn flags(&self) -> Arc<[Arg]> {
        self.flags.clone()
    }

    pub fn incr_files(&mut self) {
        self.files += 1;
    }

    pub fn incr_dirs(&mut self) {
        self.dirs += 1;
    }

    pub fn set_flags(&mut self, flags: Arc<[Arg]>) {
        self.flags = flags;
    }

    pub fn is_help(&self) -> bool {
        self.flags.first().eq(&Some(&Arg::Help))
    }

    pub fn is_all(&self) -> bool {
        self.flags.contains(&Arg::All)
    }

    pub fn is_file_only(&self) -> bool {
        self.flags.contains(&Arg::FileOnly)
    }

    pub fn is_directory_only(&self) -> bool {
        self.flags.contains(&Arg::DirOnly)
    }

    pub fn validate_args(&self) -> Result<(), Arc<str>> {
        for flag in self.flags.iter() {
            match flag {
                Arg::Unknown(f) => return Err(f.clone()),
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
