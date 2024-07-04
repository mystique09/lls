use std::sync::{mpsc::channel, Arc, RwLock};

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
    let (tx, rx) = channel::<Option<CrawlData>>();

    let data = Data::default();
    let data = Arc::new(RwLock::new(data));
    let crawler = Crawler::new(data.clone(), Arc::new(tx));

    if data.as_ref().read().unwrap().is_help() {
        println!("{PURPLE}{}{RESET}", HELP);
        return;
    }

    match data.as_ref().read().unwrap().validate_args() {
        Ok(()) => {
            crawler.crawl();
        }
        Err(err) => {
            println!("{RED}Unknown flag: \"{err}\"{RESET}");
            println!("{PURPLE}{}{RESET}", HELP);
            return;
        }
    }

    while let Ok(content) = rx.recv() {
        match content {
            Some(found) => match found {
                CrawlData::Content => {
                    data.as_ref().write().unwrap().incr_files();
                }
                CrawlData::Dir => {
                    data.as_ref().write().unwrap().incr_dirs();
                }
            },
            None => break,
        }
    }

    let output = data.read().unwrap();
    println!("{GREEN}{}{RESET}", output);
}
