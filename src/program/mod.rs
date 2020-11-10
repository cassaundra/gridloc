use std::collections::BTreeMap;

use std::ops::{Add, Sub};

mod grid;
pub use grid::*;

#[derive(Clone, Debug)]
pub struct ProgramState<G: Grid> {
    pub grid: G,
    pub pointers: Vec<Pointer>,
    pub saved_positions: BTreeMap<u8, Position>,
    pub string_mode: Option<StringModeKind>,
}

impl<G: Grid> Default for ProgramState<G> {
    fn default() -> Self {
        Self {
            pointers: vec![Pointer::default()],
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Pointer {
    pub position: Position,
    pub stack: Vec<Position>,
    pub value: u8,
}

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x, y,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x.wrapping_add(other.x),
            y: self.y.wrapping_add(other.y),
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x.wrapping_sub(other.x),
            y: self.y.wrapping_sub(other.y),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    SetDirection(Direction),
    Move,
    ReadValue,
    WriteValue,
    SwapValue,
    StartLoop,
    EndLoop,
    PushPosition,
    PopPosition,
    SwapPosition,
    SavePosition,
    LoadPosition,
    GoToColumn,
    GoToRow,
    ToggleStringMode { kind: StringModeKind },
    Evaluate,
    Kill,
    Value(u8),
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equals,
    GreaterThan,
    LogicalNot,
    ReadInput,
    PrintCharacter { new_line: bool },
    PrintNumber { new_line: bool },
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StringModeKind {
    Single,
    Double,
}
