use std::io;
use std::io::{BufReader, BufWriter, Read, Write};

use crate::*;

mod parser;
pub use parser::*;

#[derive(Default)]
pub struct Interpreter<'a, G: Grid> {
    source: &'a str,
    reader: Option<BufReader<&'a mut dyn Read>>,
    writer: Option<BufWriter<&'a mut dyn Write>>,
    state: ProgramState<G>,
}

impl<'a, G: Grid> Interpreter<'a, G> {
    pub fn run(&mut self) -> io::Result<()> {
        unimplemented!()
    }

    pub fn step(&mut self) -> io::Result<()> {
        unimplemented!()
    }

    pub fn state(&self) -> &ProgramState<G> {
        &self.state
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
        Interpreter {
            source: self.source,
            reader: self.reader,
            writer: self.writer,
            ..Default::default()
        }
    }
}
