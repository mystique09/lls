use std::{
    borrow::Cow,
    env::args,
    fs::{read_dir, FileType},
};

fn main() {
    let mut depth = 0;
    //let f_path = String::from("src/");
    let mut flags: Vec<String> = vec![];
    for flag in args() {
        flags.push(flag);
    }
    flags.remove(0);

    if flags.len() > 1 {
        eprintln!("\u{001b}[31mToo many arguments.\u{001b}[0m");
        return;
    }

    let command = flags.get(0).or(Some(&".".to_string())).unwrap().to_string();

    if command == *"help" {
        println!("\u{001b}[32mUsage: lls <path>\u{001b}[0m");
        return;
    }

    let mut str_format = command.to_string();

    read_f(command, &mut str_format, &mut depth);
    println!("{}", str_format);
}

fn read_f(fpath: String, fstr: &mut String, depth: &mut usize) {
    let dirs = read_dir(&fpath);
    *depth += 1;

    match dirs {
        Ok(dir) => {
            for folder in dir {
                match folder {
                    Ok(ff) => match ff.file_type() {
                        Ok(ftype) => {
                            format_f(ftype, ff.file_name().to_string_lossy(), fstr, &fpath, depth)
                        }
                        Err(why) => eprintln!("\u{001b}[31m{}\u{001b}[0m", why),
                    },
                    Err(why) => eprintln!("\u{001b}[31m{}\u{001b}[0m", why),
                }
            }
        }
        Err(why) => eprintln!("\u{001b}[31m{}\u{001b}[0m", why),
    }
}

fn format_f(ftype: FileType, fname: Cow<str>, fstr: &mut String, fpath: &str, depth: &mut usize) {
    if ftype.is_dir() {
        let mut inner_depth = *depth;
        let mut inner_fstr = format!(
            "\n{}\u{001b}[34m└── {}\u{001b}[0m",
            &" ".repeat(inner_depth),
            fname
        );
        read_f(
            format!("{}/{}", fpath, fname),
            &mut inner_fstr,
            &mut inner_depth,
        );
        fstr.push_str(&inner_fstr.to_string());
    } else if ftype.is_file() {
        fstr.push_str(&format!("\n{}└── {}", &" ".repeat(*depth), fname).into_boxed_str());
    }
}
