use std::{
    borrow::Cow,
    env::args,
    fs::{read_dir, FileType},
};

fn main() {
    let mut depth = 0;
    let mut flags = args().collect::<Vec<_>>();
    flags.remove(0);

    if flags.len() > 2 {
        eprintln!("\u{001b}[31mToo many arguments.\u{001b}[0m");
        return;
    }

    let command = flags.get(0).or(Some(&".".to_string())).unwrap().to_string();
    let all = { command == "-a" || command == "--all" };

    if command == *"--help" || command == *"-h" {
        println!("\u{001b}[34mUsage: \n\tlls <option> <path> \n\nOption: \n\t-a: include hidden files and folders.\u{001b}[0m");
        return;
    }

    if flags.len() == 2 {
        if !command.contains("-") {
            eprintln!(
                "\u{001b}[31mInvalid argument: {}\u{001b}[0m",
                flags.get(1).unwrap()
            );
            return;
        }
    }

    let command = {
        if all {
            flags.get(1).or(Some(&".".to_string())).unwrap().to_string()
        } else {
            command
        }
    };

    let str_format = command.to_string();

    let mut total_dirs = 0;
    let mut total_fs = 0;

    println!("\u{001b}[34m{}\u{001b}[0m", str_format);
    read_f(command, &mut depth, &mut total_fs, &mut total_dirs, all);

    println!("{} directorie(s), {} file(s)", total_dirs, total_fs);
}

fn read_f(fpath: String, depth: &mut usize, tfs: &mut usize, tds: &mut usize, all: bool) {
    let dirs = read_dir(&fpath);
    *depth += 1;

    match dirs {
        Ok(dir) => {
            for folder in dir {
                match folder {
                    Ok(ff) => match ff.file_type() {
                        Ok(ftype) => format_f(
                            ftype,
                            ff.file_name().to_string_lossy(),
                            &fpath,
                            depth,
                            tfs,
                            tds,
                            all,
                        ),
                        Err(why) => eprintln!("\u{001b}[31m{}\u{001b}[0m", why),
                    },
                    Err(why) => eprintln!("\u{001b}[31m{}\u{001b}[0m", why),
                }
            }
        }
        Err(why) => eprintln!("\u{001b}[31m{}\u{001b}[0m", why),
    }
}

fn format_f(
    ftype: FileType,
    fname: Cow<str>,
    fpath: &str,
    depth: &mut usize,
    tfs: &mut usize,
    tds: &mut usize,
    all: bool,
) {
    if ftype.is_dir() {
        // return if the directory is hidden
        if fname.starts_with(".") && !all {
            return;
        }

        if fname.starts_with(".git") {
            return;
        }

        *tds += 1;
        let mut inner_depth = *depth;
        let inner_fstr = format!(
            "{}\u{001b}[34m└── {}\u{001b}[0m",
            &" ".repeat(inner_depth),
            fname
        );

        println!("{}", &inner_fstr);
        read_f(
            format!("{}/{}", fpath, fname),
            &mut inner_depth,
            tfs,
            tds,
            all,
        );
    } else if ftype.is_file() {
        // return if the file is hidden
        if fname.starts_with(".") && !all {
            return;
        }

        *tfs += 1;
        println!("{}└── {}", &" ".repeat(*depth), fname);
    }
}
