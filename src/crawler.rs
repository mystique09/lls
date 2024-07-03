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
        self.sender.send("hello".into()).unwrap();
        self.sender.send("world".into()).unwrap();
        self.sender.send("done".into()).unwrap();

        while let Ok(data) = self.receiver.recv() {
            match data.as_ref() {
                "done" => break,
                _ => println!("{}", data),
            }
        }
    }

    fn crawl_files(&self) {
        todo!()
    }

    fn crawl_dirs(&self) {
        todo!()
    }
}
