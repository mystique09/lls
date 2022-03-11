use std::{
    borrow::Cow,
    fs::{read_dir, FileType},
};

fn main() {
    //let mut str_format = format!("{}\n", );
    let mut depth = 0;
    let f_path = String::from("src/");
    let mut str_format = format!("{}\n", f_path);
    //str_format.push_str(&" ----".repeat(depth)); //.push_str("└──");
    read_f(f_path, &mut str_format, &mut depth);
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
                        Err(why) => println!("{}", why),
                    },
                    Err(why) => eprintln!("{}", why),
                }
            }
        }
        Err(why) => println!("ERROR: ({})", why),
    }
}

fn format_f(
    ftype: FileType,
    fname: Cow<str>,
    fstr: &mut String,
    fpath: &String,
    depth: &mut usize,
) {
    if ftype.is_dir() {
        let mut inner_depth = *depth;
        let mut inner_fstr = String::from(format!("\n{}└── {}", &" ".repeat(inner_depth), fname));
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
