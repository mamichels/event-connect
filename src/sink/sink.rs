use crate::{Args, SinkTypeParam};
use crate::sink::std_out_sink::StdOutSink;

#[derive(Debug)]
pub(crate) enum SinkType {
    /// Sink type for writing data to standard output
    StdOut
}

impl SinkType {
    /// Return the string representation of the sink type
    pub(crate) fn value(&self) -> String {
        match self {
            SinkType::StdOut => "std_out".to_string(),
        }
    }

    /// Return the sink type from the string representation
    fn from_param(s: &SinkTypeParam) -> Self {
        match s {
            SinkTypeParam::std_out => SinkType::StdOut,
        }
    }
}

/// Trait for writing data to a sink
pub trait Sink {
    /// Write data to the sink
    fn write(&self, data: Vec<u8>);
}

/// Build a sink based on the sink type specified in the arguments
pub fn build(args: &Args) -> Box<dyn Sink> {
    match SinkType::from_param(&args.sink) {
        SinkType::StdOut => Box::new(build_std_out_sink()),
    }
}

fn build_std_out_sink() -> StdOutSink {
    StdOutSink::new()
}
