use std::collections::BTreeMap;

use std::io::{BufReader, BufWriter, Read, Write};

use crate::*;

mod grid;
pub use grid::*;

#[derive(Default)]
pub struct Interpreter<'a, G: Grid> {
    source: &'a str,
    reader: Option<BufReader<&'a mut dyn Read>>,
    writer: Option<BufWriter<&'a mut dyn Write>>,
    pointers: Vec<Pointer>,
    grid: G,
    saved_positions: BTreeMap<u8, Position>,
    string_mode: Option<StringModeKind>,
    is_commenting: bool,
}

impl<'a, G: Grid> Interpreter<'a, G> {
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

struct Pointer {
    position: Position,
    stack: Vec<Position>,
    value: u8,
}
