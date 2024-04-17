use std::io::{BufWriter, stdout, Write};
use crate::sink::sink::{Sink, SinkType};

pub struct StdOutSink;

impl StdOutSink {
    const TYPE: SinkType = SinkType::StdOut;

    pub(crate) fn new() -> StdOutSink {
        println!("Writing data to: {:?}", Self::TYPE.value());
        StdOutSink
    }
}

impl Sink for StdOutSink {
    fn write(&self, data: Vec<u8>) {
        let stdout = stdout();
        let mut handle = BufWriter::new(stdout.lock());
        for byte in data {
            handle.write(&[byte]).unwrap();
        }
        handle.flush().unwrap();
    }
}
