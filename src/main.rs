use std::io::{stdin, stdout};

mod interpreter;
pub use interpreter::*;

mod program;
pub use program::*;

fn main() {
    let source = "(\"Hello world\")[p,]";

	let mut stdin = stdin();
	let mut stdout = stdout();

    let mut interpreter: Interpreter<HashGrid> = InterpreterBuilder::from_source(source)
        .reader(&mut stdin)
        .writer(&mut stdout)
        .build();
}
