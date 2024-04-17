use std::io::Read;
use std::sync::mpsc::Receiver;
use crate::source::source::{Source, SourceType};

pub struct StdInSource {
    recv: Receiver<Vec<u8>>,
}

impl StdInSource {
    const TYPE: SourceType = SourceType::StdIn;

    pub(crate) fn new() -> StdInSource {
        println!("Reading data from: {:?}", Self::TYPE.value());
        StdInSource {
            recv: spawn_channel(),
        }
    }
}

impl Source for StdInSource {
    fn read(&self) -> Result<Vec<u8>, &'static str> {
        match self.recv.recv() {
            Ok(data) => Ok(data),
            Err(_) => Err("Failed to read data from stdin"),
        }
    }
}

fn spawn_channel() -> Receiver<Vec<u8>> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut buffer = Vec::new();
        loop {
            std::io::stdin().read_to_end(&mut buffer).unwrap();
            tx.send(buffer.clone()).unwrap();
            buffer.clear();
        }
    });
    println!("Spawned stdin channel on pid: {}", std::process::id());
    rx
}
