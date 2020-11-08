use crate::*;

pub fn parse_program(source: &str) -> Program {
    let mut instructions = vec![];
    let mut is_commenting = false;

    for c in source.chars() {
        match c {
            '#' => {
                is_commenting = true;
            }
            '\n' => {
                is_commenting = false;
            }
            _ => {
                if let Some(instruction) = parse_instruction(&c) {
                    if !is_commenting {
                        instructions.push(instruction);
                    }
                }
            }
        }
    }

    Program::new(instructions)
}

pub fn parse_instruction(c: &char) -> Option<Instruction> {
    use Instruction::*;

    Some(match c {
        '<' => SetDirection(Direction::Left),
        '>' => SetDirection(Direction::Right),
        '^' => SetDirection(Direction::Up),
        'v' => SetDirection(Direction::Down),
        ',' => Move,
        '.' => ReadValue,
        ';' => WriteValue,
        '~' => SwapValue,
        '[' => StartLoop,
        ']' => EndLoop,
        '(' => PushPosition,
        ')' => PopPosition,
        '$' => SwapPosition,
        'q' => SavePosition,
        'g' => LoadPosition,
        'x' => GoToColumn,
        'y' => GoToRow,
        '\'' => ToggleStringMode { kind: StringModeKind::Single },
        '"' => ToggleStringMode { kind: StringModeKind::Double },
        'e' => Evaluate,
        '@' => Kill,
        '0'..='9' => Value(c.to_digit(10).unwrap() as u8),
        'A'..='F' => Value(*c as u8 - 'A' as u8 + 10),
        '+' => Add,
        '-' => Subtract,
        '*' => Multiply,
        '/' => Divide,
        '%' => Modulo,
        '=' => Equals,
        '`' => GreaterThan,
        '!' => LogicalNot,
        'i' => ReadInput,
        'p' | 'P' => PrintCharacter { new_line: c.is_uppercase() },
        'n' | 'N' => PrintNumber { new_line: c.is_uppercase() },
        _ => return None,
    })
}
