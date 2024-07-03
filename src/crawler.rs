use std::{
    rc::Rc,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::data::DataTrait;

pub struct Crawler<T>
where
    T: DataTrait,
{
    data: Rc<T>,
    receiver: Receiver<Rc<str>>,
    sender: Sender<Rc<str>>,
}

impl<T> Crawler<T>
where
    T: DataTrait,
{
    pub fn new(data: Rc<T>) -> Self {
        let (tx, rx) = channel::<Rc<str>>();

        Self {
            data,
            sender: tx,
            receiver: rx,
        }
    }

    fn data(&self) -> Rc<T> {
        self.data.clone()
    }

    pub fn crawl(&self) {
        if self.data().is_all() {
            self.crawl_all();
        }

        if self.data.is_file_only() {
            self.crawl_files();
        }

        if self.data().is_directory_only() {
            self.crawl_dirs();
        }

        while let Ok(data) = self.receiver.recv() {
            match data.as_ref() {
                "done" => break,
                _ => println!("{}", data),
            }
        }
    }

    fn crawl_files(&self) {
        self.display("crawl files only".into());
        self.done();
    }

    fn crawl_dirs(&self) {
        self.display("crawl directory only".into());
        self.done();
    }

    fn crawl_all(&self) {
        self.display("crawl all".into());
        self.done();
    }

    fn display(&self, value: &str) {
        self.sender.send(value.into()).unwrap();
    }

    fn done(&self) {
        self.sender.send("done".into()).unwrap();
    }
}
