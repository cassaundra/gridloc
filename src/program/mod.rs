use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ops::{Add, Sub, Mul};
use std::rc::Rc;

mod grid;
pub use grid::*;

pub struct ProgramState<G: Grid> {
    eval_tapes: Vec<Box<dyn EvalTape>>,
    grid: Rc<G>,
    pointers: Vec<Rc<RefCell<Pointer>>>,
    saved_positions: BTreeMap<u8, Position>,
    string_mode: Option<StringModeKind>,
}

impl<G: Grid> ProgramState<G> {
    pub fn new(program: Box<dyn EvalTape>) -> Self {
        Self {
            eval_tapes: vec![program],
            grid: Rc::new(G::default()),
            pointers: vec![Rc::new(RefCell::new(Pointer::default()))],
            saved_positions: BTreeMap::new(),
            string_mode: None,
        }
    }

    pub fn step(&mut self) {
        
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        use Instruction::*;

        match instruction {
            SetDirection(direction) => {

            }
            Move => {
            }
            ReadValue => {
            }
            WriteValue => {
            }
            SwapValue => {
            }
            StartLoop => {
            }
            EndLoop => {
            }
            PushPosition => {
            }
            PopPosition => {
            }
            SwapPosition => {
            }
            SavePosition => {
            }
            LoadPosition => {
            }
            GoToColumn => {
            }
            GoToRow => {
            }
            ToggleStringMode { kind: StringModeKind } => {
            }
            Evaluate => {
            }
            Kill => {
            }
            Value(value) => {
            }
            Add => {
            }
            Subtract => {
            }
            Multiply => {
            }
            Divide => {
            }
            Modulo => {
            }
            Equals => {
            }
            GreaterThan => {
            }
            LogicalNot => {
            }
            ReadInput => {
            }
            PrintCharacter { new_line: bool } => {
            }
            PrintNumber { new_line: bool } => {
            }
        }
    }
}

pub trait EvalTape {
    fn next(&mut self) -> Option<u8>;
    fn prev(&mut self) -> Option<u8>;
}

struct GridTape<G: Grid> {
    pointer: Rc<RefCell<Pointer>>,
    grid: Rc<G>,
}

impl<G: Grid> EvalTape for GridTape<G> {
    fn next(&mut self) -> Option<u8> {
        let pointer = &mut self.pointer.borrow_mut();
        let value = self.grid.get(&pointer.position);

        if value != 0 {
            // move the pointer forward
            let delta = pointer.direction.unit_vector();
            pointer.position = pointer.position + delta;

            Some(value)
        } else {
            None
        }
    }

    fn prev(&mut self) -> Option<u8> {
        let pointer = &mut self.pointer.borrow_mut();

        // move the pointer backwards
        let delta = pointer.direction
            .opposite()
            .unit_vector();
        pointer.position = pointer.position + delta;

        let value = self.grid.get(&pointer.position);
        Some(value)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Pointer {
    pub position: Position,
    pub direction: Direction,
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

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x.wrapping_sub(other.x),
            y: self.y.wrapping_sub(other.y),
        }
    }
}

impl Mul<isize> for Position {
    type Output = Position;

    fn mul(self, rhs: isize) -> Self::Output {
        unimplemented!()
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
    Right,
    Left,
    Up,
    Down,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        use Direction::*;

        match self {
            Right => Left,
            Left => Right,
            Up => Down,
            Down => Up,
        }
    }

    pub fn unit_vector(&self) -> Position {
        use Direction::*;

        match self {
            Right => Position::new(1, 0),
            Left => Position::new(-1, 0),
            Up => Position::new(0, 1),
            Down => Position::new(0, -1),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StringModeKind {
    Single,
    Double,
}
