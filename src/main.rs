use std::io::{stdin, stdout};

mod interpreter;
pub use interpreter::*;

mod parser;
pub use parser::*;

mod program;
pub use program::*;

fn main() {
    let source = "(\"Hello world\")[p,]";

	let mut stdin = stdin();
	let mut stdout = stdout();

    let mut interpreter: Interpreter<SimpleGrid> = InterpreterBuilder::from_source(source)
        .reader(&mut stdin)
        .writer(&mut stdout)
        .build();
}
