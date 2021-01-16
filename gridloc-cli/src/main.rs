use std::io::{stdin, stdout, BufReader};
use std::path::PathBuf;

use gridloc::*;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "gridloc")]
struct Options {
    /// Optional delay to be inserted between each instruction.
    #[structopt(short, long)]
    delay: Option<f64>,
    /// Enable program visualizer
    #[structopt(long)]
    visual: bool,
    /// File from which to read source code.
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() {
    let options = Options::from_args();
    let source = "(\"Hello world 12345\")s";

    let mut interpreter: Interpreter<HashGrid> = InterpreterBuilder::from_source(source)
        .reader(Box::new(BufReader::new(stdin())))
        .writer(Box::new(stdout()))
        .build();
    interpreter.run();
}
