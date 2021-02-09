use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io;
use std::io::{BufRead, Write};
use std::ops::{Add, Sub, Mul};
use std::rc::Rc;

mod grid;
pub use grid::*;

mod parser;
pub use parser::*;

pub struct ProgramState<'a, G: Grid, Rng: rand::Rng> {
    pub eval_tapes: Vec<Box<dyn EvalTape + 'a>>,
    pub grid: Rc<RefCell<G>>,
    pub pointers: Vec<Rc<RefCell<Pointer>>>,
    pub saved_positions: BTreeMap<u8, Position>,
    pub string_mode: Option<StringModeKind>,
    rng: Rng,
}

impl<'a, G: 'a + Grid, Rng: rand::Rng> ProgramState<'a, G, Rng> {
    pub fn new(program: Box<dyn EvalTape>, rng: Rng) -> Self {
        Self {
            eval_tapes: vec![program],
            grid: Rc::new(RefCell::new(G::default())),
            pointers: vec![Rc::new(RefCell::new(Pointer::default()))],
            saved_positions: BTreeMap::new(),
            string_mode: None,
            rng,
        }
    }

    pub fn step<R: BufRead, W: Write>(&mut self, reader: &mut R, writer: &mut W) -> io::Result<bool>{
        if let Some(value) = self.eval_tapes.last_mut().and_then(|tape| tape.next()) {
            let character = value as char;
            let instruction = parse_instruction(character);

            if let Some(current_kind) = self.string_mode {
                if instruction == Some(Instruction::ToggleStringMode { kind: current_kind }) {
                    self.string_mode = None;
                } else {
                    let mut pointer = self.pointers
                        .last()
                        .unwrap()
                        .borrow_mut();

                    self.grid.borrow_mut().set(&pointer.position, value);
                    pointer.move_pointer(1);
                }
            } else if let Some(instruction) = instruction {
                self.execute_instruction(instruction, reader, writer)?;
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn execute_instruction<R: BufRead, W: Write>(&mut self, instruction: Instruction, reader: &mut R, writer: &mut W) -> io::Result<()> {
        use Instruction::*;

        let (p_value, g_value) = {
            let pointer = self.pointers.last()
                .unwrap()
                .borrow() ;

            let p_value = pointer.value;
            let g_value = self.grid.borrow().get(&pointer.position);

            (p_value, g_value)
        };

        match instruction {
            // instructions which operate on the pointer stack
            Evaluate => {
                // TODO
                let top_pointer = self.pointers.last().unwrap();
                let new_pointer = Rc::new(RefCell::new(Pointer::from_other(&top_pointer.borrow())));

                let tape = GridTape {
                    pointer: top_pointer.clone(),
                    grid: self.grid.clone(),
                };

                self.pointers.push(new_pointer);
                self.eval_tapes.push(Box::new(tape));
            }
            StartLoop => {
                if p_value == 0 {
                    self.jump_loop_forwards();
                }
            }
            EndLoop => {
                if p_value != 0 {
                    self.jump_loop_backwards();
                }
            }
            Kill => {
                self.pointers.pop();
                self.eval_tapes.pop();
            }
            // instructions which operate on the pointer
            _ => {
                let mut pointer = self.pointers
                    .last()
                    .unwrap()
                    .borrow_mut();

                match instruction {
                    SetDirection(direction) => {
                        pointer.direction = direction;
                    }
                    MoveOne => {
                        pointer.move_pointer(1);
                    }
                    MoveMultiple => {
                        pointer.move_pointer(p_value);
                    }
                    ReadValue => {
                        // read grid value to pointer
                        pointer.value = self.grid.borrow().get(&pointer.position);
                    }
                    WriteValue => {
                        // write pointer value to grid
                        self.grid.borrow_mut().set(&pointer.position, pointer.value);
                    }
                    WriteValueMove => {
                        // write pointer value to grid
                        self.grid.borrow_mut().set(&pointer.position, pointer.value);

                        // move pointer forward
                        pointer.move_pointer(1);
                    }
                    SwapValue => {
                        let grid = &mut self.grid.borrow_mut();

                        // temporary
                        let grid_value = grid.get(&pointer.position);

                        // set grid value
                        grid.set(&pointer.position, pointer.value);

                        // set pointer value
                        pointer.value = grid_value;
                    }
                    PushPosition => {
                        let pos = pointer.position;
                        pointer.position_stack.push(pos);
                    }
                    PopPosition => {
                        // only perform if the stack is not empty
                        if let Some(pos) = pointer.position_stack.pop() {
                            pointer.position = pos;
                        }
                    }
                    SwapPosition => {
                        // only perform if the stack is not empty
                        if let Some(new_pos) = pointer.position_stack.pop() {
                            // push the current position
                            let current_pos = pointer.position;
                            pointer.position_stack.push(current_pos);

                            // go to the position that was at the top
                            pointer.position = new_pos;
                        }
                    }
                    SavePosition => {
                        self.saved_positions.insert(pointer.value, pointer.position);
                    }
                    LoadPosition => {
                        if let Some(pos) = self.saved_positions.get(&pointer.value) {
                            pointer.position = *pos;
                        }
                    }
                    ToggleStringMode { kind } => {
                        // this will only be called when
                        // string mode is not active
                        self.string_mode = Some(kind);
                    }
                    Value(value) => {
                        pointer.value &= 0b1111; // keep last four bits
                        pointer.value <<= 4; // move those four to the left
                        pointer.value += value & 0b1111; // put in the new value
                    }
                    Add => {
                        pointer.value = p_value.wrapping_add(g_value);
                    }
                    Subtract => {
                        pointer.value = p_value.wrapping_sub(g_value);
                    }
                    Multiply => {
                        pointer.value = p_value.wrapping_mul(g_value);
                    }
                    Divide => {
                        if g_value != 0 {
                            pointer.value = p_value.wrapping_div(g_value);
                        } else {
                            pointer.value = 0;
                        }
                    }
                    Modulo => {
                        pointer.value = p_value.wrapping_rem(g_value);
                    }
                    Equals => {
                        pointer.value = (p_value == g_value) as u8;
                    }
                    GreaterThan => {
                        pointer.value = (p_value > g_value) as u8;
                    }
                    LogicalAnd => {
                        pointer.value = (p_value != 0 && g_value != 0) as u8;
                    }
                    LogicalOr => {
                        pointer.value = (p_value != 0 || g_value != 0) as u8;
                    }
                    LogicalNot => {
                        pointer.value = (p_value == 0) as u8;
                    }
                    Random => {

                    }
                    Write { kind } => {
                        let value = match kind {
                            IOKind::Character => (g_value as char).to_string(),
                            IOKind::Number => format!("{}", g_value),
                            IOKind::String => {
                                let mut s = String::new();
                                let mut current_value = g_value;

                                while current_value != 0 {
                                    // push the character value to the string
                                    s.push(current_value as char);

                                    // move pointer
                                    pointer.move_pointer(1);
                                    current_value = self.grid.borrow().get(&pointer.position);
                                }

                                s
                            },
                        };

                        write!(writer, "{}", value)?;
                        writer.flush()?;
                    }
                    Read { kind } => {
                        let mut buffer = String::new();
                        reader.read_line(&mut buffer)?;

                        buffer = buffer.trim().to_string();

                        let values = match kind {
                            IOKind::Character => {
                                let c = buffer.chars().next().unwrap_or(0 as char);
                                vec![c as u8]
                            }
                            IOKind::Number => {
                                let num = buffer.parse::<usize>().unwrap_or(0) as u8;
                                vec![num]
                            }
                            IOKind::String => {
                                buffer.chars().map(|c| c as u8).collect()
                            }
                        };

                        for value in values {
                            self.grid.borrow_mut().set(&pointer.position, value);
                            pointer.move_pointer(1);
                        }
                    }
                    _ => unreachable!()
                }
            }
        }

        Ok(())
    }

    fn jump_loop(&mut self, forwards: bool) {
        use Instruction::*;

        let tape = self.eval_tapes.last_mut().unwrap();

        let mut depth = 0;

        while let Some(value) = if forwards { tape.next() } else { tape.prev() } {
            if let Some(instr @ (StartLoop | EndLoop)) = parse_instruction(value as char) {
                let is_matching = instr == if forwards { EndLoop } else { StartLoop };

                if is_matching {
                    if depth > 0 {
                        depth -= 1;
                    } else {
                        break;
                    }
                } else {
                    depth += 1;
                }
            }
        }
    }

    fn jump_loop_forwards(&mut self) {
        self.jump_loop(true)
    }

    fn jump_loop_backwards(&mut self) {
        self.jump_loop(false)
    }
}

pub trait EvalTape {
    fn next(&mut self) -> Option<u8>;
    fn prev(&mut self) -> Option<u8>;
}

struct GridTape<G: Grid> {
    pointer: Rc<RefCell<Pointer>>,
    grid: Rc<RefCell<G>>,
}

impl<G: Grid> EvalTape for GridTape<G> {
    fn next(&mut self) -> Option<u8> {
        let pointer = &mut self.pointer.borrow_mut();
        let value = self.grid.borrow().get(&pointer.position);

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

        let value = self.grid.borrow().get(&pointer.position);
        Some(value)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Pointer {
    pub position: Position,
    pub direction: Direction,
    pub position_stack: Vec<Position>,
    pub value: u8,
}

impl Pointer {
    pub fn new(position: Position, direction: Direction) -> Self {
        Pointer {
            position,
            direction,
            ..Default::default()
        }
    }

    pub fn from_other(other: &Pointer) -> Self {
        Pointer::new(other.position, other.direction)
    }

    pub fn move_pointer(&mut self, amount: u8) {
        self.position = self.position + self.direction.unit_vector() * amount.into();
    }
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

    fn mul(self, scalar: isize) -> Self::Output {
        Position::new(scalar * self.x, scalar * self.y)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    SetDirection(Direction),
    MoveOne,
    MoveMultiple,
    ReadValue,
    WriteValue,
    WriteValueMove,
    SwapValue,
    StartLoop,
    EndLoop,
    PushPosition,
    PopPosition,
    SwapPosition,
    SavePosition,
    LoadPosition,
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
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    Random,
    Write { kind: IOKind },
    Read { kind: IOKind },
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IOKind {
    Character,
    String,
    Number,
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
