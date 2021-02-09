use std::io;
use std::io::{BufRead, Write};

use rand::{rngs::ThreadRng, thread_rng};

use crate::*;

pub struct Interpreter<'a, G: 'a + Grid> {
    reader: Box<dyn BufRead>,
    writer: Box<dyn Write>,
    state: ProgramState<'a, G, ThreadRng>,
}

impl<'a, G: 'a + Grid> Interpreter<'a, G> {
    pub fn state(&self) -> &ProgramState<'a, G, ThreadRng> {
        &self.state
    }

    pub fn run(&mut self) -> io::Result<()> {
        while self.step()? {}

        Ok(())
    }

    pub fn step(&mut self) -> io::Result<bool> {
        self.state.step(&mut self.reader, &mut self.writer)
    }
}

pub struct InterpreterBuilder<'a> {
    source: &'a str,
    reader: Option<Box<dyn BufRead>>,
    writer: Option<Box<dyn Write>>,
}

impl<'a> InterpreterBuilder<'a> {
    pub fn from_source(source: &'a str) -> Self {
        Self {
            source,
            reader: None,
            writer: None,
        }
    }

    pub fn reader(mut self, reader: Box<dyn BufRead>) -> Self {
        self.reader = Some(Box::new(reader));
        self
    }

    pub fn writer(mut self, writer: Box<dyn Write>) -> Self {
        self.writer = Some(Box::new(writer));
        self
    }

    pub fn build<G: Grid + 'a>(self) -> Interpreter<'a, G> {
        let tape = SourceTape::from(self.source);
        let program_state = ProgramState::new(Box::new(tape), thread_rng());

        Interpreter {
            reader: self.reader.unwrap_or(Box::new(io::empty())),
            writer: self.writer.unwrap_or(Box::new(io::sink())),
            state: program_state,
        }
    }
}

pub struct SourceTape {
    source: Vec<u8>,
    index: usize,
}

impl EvalTape for SourceTape {
    fn peek_next(&self) -> Option<u8> {
        if self.index < self.source.len() {
            let value = self.source[self.index];
            Some(value)
        } else {
            None
        }
    }

    fn next(&mut self) -> Option<u8> {
        let value = self.peek_next();

        if value.is_some() {
            // advance
            self.index += 1;
        }

        value
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
