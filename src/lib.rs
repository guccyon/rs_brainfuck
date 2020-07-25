use std::fmt;

pub mod interpreter;
pub mod parser;

#[derive(Debug)]
pub enum Instruction {
    UNDEFINED,
    NEXT,
    PREV,
    INC,
    DEC,
    IN,
    OUT,
    BLOCK(Vec<Instruction>),
}

impl From<char> for Instruction {
    fn from(c: char) -> Instruction {
        use Instruction::*;
        match c {
            '>' => NEXT,
            '<' => PREV,
            '+' => INC,
            '-' => DEC,
            '.' => OUT,
            ',' => IN,
            _ => UNDEFINED,
        }
    }
}

impl Instruction {
    fn as_str(&self) -> &str {
        use Instruction::*;
        match self {
            &NEXT => ">",
            &PREV => "<",
            &INC => "+",
            &DEC => "-",
            &OUT => ".",
            &IN => ",",
            &BLOCK(_) => "[...]",
            &UNDEFINED => "*",
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
