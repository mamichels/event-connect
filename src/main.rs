use clap::Parser;

mod sink;
mod source;

#[allow(non_camel_case_types)]
#[derive(clap::ValueEnum, Debug, Clone)]
enum SourceTypeParam {
    /// Source type for reading data from stdin
    std_in,
}

#[allow(non_camel_case_types)]
#[derive(clap::ValueEnum, Debug, Clone)]
enum SinkTypeParam {
    /// Sink type for writing data to standard output
    std_out,
}

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Required
    #[clap(short = 'd', long, value_name = "type", value_enum)]
    sink: SinkTypeParam,

    /// Required
    #[clap(short = 's', long, value_name = "type", value_enum)]
    source: SourceTypeParam,

}

fn main() {
    let args = Args::parse();
    connect(&args);
}


fn connect(args: &Args) {
    let source = source::build(&args);
    let sink = sink::build(&args);

    loop {
        match source.read() {
            Ok(data) => sink.write(data),
            Err(e) => println!("{}", e),
        }
    }
}
