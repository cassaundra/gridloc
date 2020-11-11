use std::io;
use std::io::{BufReader, BufWriter, Read, Write};

use crate::*;

mod parser;
pub use parser::*;

pub struct Interpreter<'a, G: Grid> {
    reader: Option<BufReader<&'a mut dyn Read>>,
    writer: Option<BufWriter<&'a mut dyn Write>>,
    state: ProgramState<G>,
}

impl<'a, G: Grid> Interpreter<'a, G> {
    pub fn state(&self) -> &ProgramState<G> {
        &self.state
    }

    pub fn run(&mut self) -> io::Result<()> {
        Ok(())
    }

    pub fn step(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct InterpreterBuilder<'a> {
    source: &'a str,
    reader: Option<BufReader<&'a mut dyn Read>>,
    writer: Option<BufWriter<&'a mut dyn Write>>,
}

impl<'a> InterpreterBuilder<'a> {
    pub fn from_source(source: &'a str) -> Self {
        Self {
            source,
            reader: None,
            writer: None,
        }
    }

    pub fn reader(mut self, reader: &'a mut dyn Read) -> Self {
        self.reader = Some(BufReader::new(reader));
        self
    }

    pub fn writer(mut self, writer: &'a mut dyn Write) -> Self  {
        self.writer = Some(BufWriter::new(writer));
        self
    }

    pub fn build<G: Grid>(self) -> Interpreter<'a, G> {
        let tape = SourceTape::from(self.source);
        let program_state = ProgramState::new(Box::new(tape));

        Interpreter {
            reader: self.reader,
            writer: self.writer,
            state: program_state,
        }
    }
}

struct SourceTape {
    source: Vec<u8>,
    index: usize,
}

impl From<&[u8]> for SourceTape {
    fn from(slice: &[u8]) -> SourceTape {
        Self {
            source: slice.to_owned(),
            index: 0,
        }
    }
}

impl From<&str> for SourceTape {
    fn from(s: &str) -> SourceTape {
        s.as_bytes().into()
    }
}

impl EvalTape for SourceTape {
    fn next(&mut self) -> Option<u8> {
        if self.index < self.source.len() {
            let value = self.source[self.index];
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }

    fn prev(&mut self) -> Option<u8> {
        if self.index > 0 {
            self.index -= 1;
            Some(self.source[self.index])
        } else {
            None
        }
    }
}
