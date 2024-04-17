use crate::{Args, SourceTypeParam};
use crate::source::std_in_source::StdInSource;

#[derive(Debug)]
pub(crate) enum SourceType {
    /// Source type for reading data from standard input
    StdIn,
}

impl SourceType {
    /// Return the string representation of the source type
    pub(crate) fn value(&self) -> String {
        match self {
            SourceType::StdIn => "std_in".to_string(),
        }
    }

    /// Return the source type from the string representation
    fn from_param(s: &SourceTypeParam) -> Self {
        match s {
            SourceTypeParam::std_in => SourceType::StdIn,
        }
    }
}

/// Trait for reading data from a source
pub trait Source {
    /// Read data from the source
    fn read(&self) -> Result<Vec<u8>, &'static str>;
}

/// Build a source based on the source type specified in the arguments
pub fn build(args: &Args) -> Box<dyn Source> {
    match SourceType::from_param(&args.source) {
        SourceType::StdIn => Box::new(build_std_in_source()),
    }
}

fn build_std_in_source() -> StdInSource {
    StdInSource::new()
}