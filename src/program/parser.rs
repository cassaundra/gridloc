use crate::*;

pub const COMMENT_CHAR: char = '#';

pub fn parse_instruction(c: char) -> Option<Instruction> {
    use Instruction::*;

    Some(match c {
        '<' => SetDirection(Direction::Left),
        '>' => SetDirection(Direction::Right),
        '^' => SetDirection(Direction::Up),
        'v' => SetDirection(Direction::Down),
        '.' => MoveOne,
        ':' => MoveMultiple,
        '_' => ReadValue,
        ',' => WriteValue,
        ';' => WriteValueMove,
        '~' => SwapValue,
        '[' => StartLoop,
        ']' => EndLoop,
        '(' => PushPosition,
        ')' => PopPosition,
        '$' => SwapPosition,
        'q' => SavePosition,
        'g' => LoadPosition,
        '\'' => ToggleStringMode { kind: StringModeKind::Single },
        '"' => ToggleStringMode { kind: StringModeKind::Double },
        'e' => Evaluate,
        '@' => Kill,
        '0'..='9' => Value(c.to_digit(10).unwrap() as u8),
        'A'..='F' => Value(c as u8 - 'A' as u8 + 10),
        '+' => Add,
        '-' => Subtract,
        '*' => Multiply,
        '/' => Divide,
        '%' => Modulo,
        '=' => Equals,
        '`' => GreaterThan,
        '&' => LogicalAnd,
        '|' => LogicalOr,
        '!' => LogicalNot,
        'x' => Write { kind: IOKind::Character },
        'n' => Write { kind: IOKind::Number },
        's' => Write { kind: IOKind::String },
        'X' => Read { kind: IOKind::Character },
        'N' => Read { kind: IOKind::Number },
        'S' => Read { kind: IOKind::String },
        _ => return None,
    })
}

pub fn clean_source(source: &str) -> String {
    let mut buffer = String::with_capacity(source.len());

    for line in source.lines() {
        let line = line.trim();

        if let Some(idx) = line.find(COMMENT_CHAR) {
            buffer.push_str(&line[..idx]);
        } else {
            buffer.push_str(&line);
        };
    }

    buffer
}
