use lls::{
    colors::{GREEN, PURPLE, RED, RESET},
    data::Data,
};

const HELP: &str = r#"Usage:
lls <option> <path>

[options]
    --all, -a: include hidden files and folders.
    --help, -h: prints this help message.
    --file, -f: crawl only the files.
    --dir, -d: crawl only the directories."#;

fn main() {
    let data = Data::default();

    if data.is_help() {
        println!("{PURPLE}{}{RESET}", HELP);
    }

    match data.validate_flags() {
        Ok(()) => {}
        Err(err) => {
            println!("{RED}Unknown flag: \"{err}\"{RESET}");
        }
    }

    println!("{GREEN}{}{RESET}", data);
}
