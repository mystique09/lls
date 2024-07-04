use crate::colors::{BLUE, PURPLE, RED, RESET};
use std::{
    fs::read_dir,
    os::windows::fs::FileTypeExt,
    sync::{mpsc::Sender, Arc, RwLock},
};

use crate::data::Data;

#[derive(Debug)]
pub enum CrawlData {
    Content,
    Dir,
}

pub struct Crawler {
    data: Arc<RwLock<Data>>,
    sender: Arc<Sender<Option<CrawlData>>>,
}

impl Crawler {
    pub fn new(data: Arc<RwLock<Data>>, sender: Arc<Sender<Option<CrawlData>>>) -> Self {
        Self { data, sender }
    }

    pub fn crawl(&self) {
        let data = self.data.as_ref().read().unwrap();
        let data_clone = Arc::clone(&self.data);
        let path = data.get_path();

        println!("{BLUE}{path}{RESET}");
        self.crawl_content(path.as_str(), data_clone, &mut 0);
    }

    fn crawl_content(&self, path: &str, data: Arc<RwLock<Data>>, depth: &mut usize) {
        let dir_content = read_dir(path);
        *depth = *depth + 1;

        match dir_content {
            Ok(contents) => {
                self.sender.send(Some(CrawlData::Dir)).unwrap();

                for content in contents {
                    match content {
                        Ok(file) => {
                            let file_type = file.file_type().unwrap();
                            let mut inner = *depth;

                            if file_type.is_dir() || file_type.is_symlink_dir() {
                                println!(
                                    "{}{BLUE}└── {}{RESET}",
                                    " ".repeat(inner),
                                    file.file_name().to_str().unwrap()
                                );
                                self.sender.send(Some(CrawlData::Dir)).unwrap();
                                self.crawl_content(
                                    file.path().as_path().to_str().unwrap(),
                                    Arc::clone(&data),
                                    &mut inner,
                                );
                            }

                            match file.file_name().to_str() {
                                Some(file_name) => println!(
                                    "{}{PURPLE}└── {}{RESET}",
                                    " ".repeat(inner * 2),
                                    file_name
                                ),
                                None => {}
                            }

                            self.sender.send(Some(CrawlData::Content)).unwrap();
                        }
                        Err(e) => {
                            eprintln!("{}{RED}{}{RESET}", "".repeat(*depth * 2), e);
                        }
                    }
                }
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => println!("{RED}{}{RESET}", e),
                _ => eprintln!("{}{RED}{}{RESET}", "".repeat(*depth * 2), e),
            },
        }

        self.sender.send(None).unwrap();
    }
}
