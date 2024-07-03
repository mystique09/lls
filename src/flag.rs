use std::{env, fmt::Display, rc::Rc};

#[derive(Debug, PartialEq)]
pub enum CommandFlag {
    Help,
    All,
    FileOnly,
    DirOnly,
    Unknown(Rc<str>),
}

impl From<Rc<str>> for CommandFlag {
    fn from(value: Rc<str>) -> Self {
        match value.as_ref() {
            "--help" | "-h" => Self::Help,
            "--all" | "-a" => Self::All,
            "--file" | "-f" => Self::FileOnly,
            "--dir" | "-d" => Self::DirOnly,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for CommandFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Help => write!(f, "--help"),
            Self::All => write!(f, "--all"),
            Self::FileOnly => write!(f, "--file"),
            Self::DirOnly => write!(f, "--dir"),
            Self::Unknown(flag) => write!(f, "{}", flag),
        }
    }
}

pub fn get_flags() -> Rc<[CommandFlag]> {
    let args = env::args().skip(1);
    let mut command_flags: Vec<CommandFlag> = vec![];

    for arg in args.into_iter() {
        let arg = arg.as_str();
        command_flags.push(CommandFlag::from(Rc::from(arg)));
    }

    command_flags.into()
}
