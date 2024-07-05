use crate::colors::{BLUE, PURPLE, RED, RESET};
use std::{
    fs::read_dir,
    os::windows::fs::FileTypeExt,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, RwLock,
    },
    thread,
};

use crate::data::Data;

#[derive(Debug)]
pub enum CrawlData {
    Content,
    Dir,
}

pub struct Crawler {
    data: Arc<RwLock<Data>>,
}

impl Crawler {
    pub fn new(data: Arc<RwLock<Data>>) -> Self {
        Self { data }
    }

    pub fn crawl(&self) -> Receiver<Option<CrawlData>> {
        let data = self.data.as_ref().read().unwrap();
        let data_clone = Arc::clone(&self.data);
        let path = data.get_path();
        let (tx, rx) = channel::<Option<CrawlData>>();
        let arc_tx = Arc::new(tx);

        println!("{BLUE}{path}{RESET}");

        thread::spawn(move || {
            Self::crawl_content(arc_tx, path.as_str(), data_clone, &mut 0);
        });

        rx
    }

    fn crawl_content(
        sender: Arc<Sender<Option<CrawlData>>>,
        path: &str,
        data: Arc<RwLock<Data>>,
        depth: &mut usize,
    ) {
        let sender = Arc::clone(&sender);
        let dir_content = read_dir(path);
        *depth += 1;

        match dir_content {
            Ok(contents) => {
                sender.as_ref().send(Some(CrawlData::Dir)).unwrap();

                for content in contents {
                    match content {
                        Ok(file) => {
                            let file_type = file.file_type().unwrap();
                            let mut inner = *depth;

                            if let Some(file_name) = file.file_name().to_str() {
                                if (!data.read().unwrap().is_all() && file_name.starts_with("."))
                                    || (file_type.is_dir() && data.read().unwrap().is_file_only())
                                    || (file_type.is_file()
                                        && data.read().unwrap().is_directory_only())
                                    || ((file_type.is_symlink_file() || file_type.is_symlink_dir())
                                        && !data.read().unwrap().is_symlinks_included())
                                {
                                    continue;
                                }

                                println!(
                                    "{}{PURPLE}└── {}{RESET}",
                                    " ".repeat(inner * 2),
                                    file_name
                                );

                                if file_type.is_dir() || file_type.is_symlink_dir() {
                                    println!("{}{BLUE}└── {}{RESET}", " ".repeat(inner), file_name);
                                    sender.as_ref().send(Some(CrawlData::Dir)).unwrap();
                                    Self::crawl_content(
                                        Arc::clone(&sender),
                                        file.path().as_path().to_str().unwrap(),
                                        Arc::clone(&data),
                                        &mut inner,
                                    );
                                }

                                sender.as_ref().send(Some(CrawlData::Content)).unwrap();
                            }
                        }
                        Err(e) => {
                            eprintln!("{}{RED}{}{RESET}", "".repeat(*depth * 2), e);
                        }
                    }
                }
            }
            Err(e) => eprintln!("{}{RED}{}{RESET}", "".repeat(*depth * 2), e),
        }
    }
}
