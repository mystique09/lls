use std::sync::{Arc, RwLock};

use lls::{
    colors::{GREEN, PURPLE, RED, RESET},
    crawler::{CrawlData, Crawler},
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
    let data_arc = Arc::new(RwLock::new(data));
    let crawler = Crawler::new(data_arc.clone());
    let data_ref = data_arc.as_ref();

    if data_ref.read().unwrap().is_help() {
        println!("{PURPLE}{}{RESET}", HELP);
        return;
    }

    let validate = data_ref.read().unwrap().validate_args();

    if let Err(err) = validate {
        println!("{RED}Unknown flag: \"{err}\"{RESET}");
        println!("{PURPLE}{}{RESET}", HELP);
        return;
    }

    let rx = crawler.crawl();

    while let Ok(content) = rx.recv() {
        match content {
            Some(found) => match found {
                CrawlData::Content => {
                    data_ref.write().unwrap().incr_files();
                }
                CrawlData::Dir => {
                    data_ref.write().unwrap().incr_dirs();
                }
            },
            None => break,
        }
    }

    let output = data_ref.read().unwrap();
    println!("{GREEN}{}{RESET}", output);
}
