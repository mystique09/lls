use std::{env, fmt::Display, path::PathBuf, sync::Arc};

#[derive(Debug, PartialEq, Clone)]
pub enum Arg {
    Help,
    All,
    FileOnly,
    DirOnly,
    IncludeSymlinks,
    Path(PathBuf),
    Unknown(Arc<str>),
}

impl From<Arc<str>> for Arg {
    fn from(value: Arc<str>) -> Self {
        match value.as_ref() {
            "--help" | "-h" => Self::Help,
            "--all" | "-a" => Self::All,
            "--file" | "-f" => Self::FileOnly,
            "--dir" | "-d" => Self::DirOnly,
            "--symlinks" | "-s" => Self::IncludeSymlinks,
            path if !path.contains('-') => Self::Path(PathBuf::from(value.as_ref())),
            _ => Self::Unknown(value),
        }
    }
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Help => write!(f, "--help"),
            Self::All => write!(f, "--all"),
            Self::FileOnly => write!(f, "--file"),
            Self::DirOnly => write!(f, "--dir"),
            Self::IncludeSymlinks => write!(f, "--symlinks"),
            Self::Path(p) => write!(f, "{}", p.to_str().unwrap()),
            Self::Unknown(flag) => write!(f, "{}", flag),
        }
    }
}

pub fn get_flags() -> Arc<[Arg]> {
    let args = env::args().skip(1);
    let mut command_flags: Vec<Arg> = vec![];

    for arg in args.into_iter() {
        let arg = arg.as_str();
        command_flags.push(Arg::from(Arc::from(arg)));
    }

    command_flags.into()
}
