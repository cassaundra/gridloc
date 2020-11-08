use std::ops::{Add, Sub};

#[derive(Clone, Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Program {
        Program {
            instructions,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
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
pub enum StringModeKind {
    Single,
    Double,
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
