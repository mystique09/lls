use std::rc::Rc;

use lls::{
    colors::{GREEN, PURPLE, RED, RESET},
    crawler::Crawler,
    data::{Data, DataTrait},
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
    let data = Rc::new(data);
    let crawler = Crawler::new(Rc::clone(&data));

    if data.is_help() {
        println!("{PURPLE}{}{RESET}", HELP);
        return;
    }

    match data.validate_flags() {
        Ok(()) => {
            crawler.crawl();
        }
        Err(err) => {
            println!("{RED}Unknown flag: \"{err}\"{RESET}");
            println!("{PURPLE}{}{RESET}", HELP);
            return;
        }
    }

    println!("{GREEN}{}{RESET}", data);
}
