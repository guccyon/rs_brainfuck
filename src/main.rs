use std::fmt;

#[derive(Debug)]
enum Instruction {
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

use std::iter::Peekable;
struct Parser;

impl Parser {
    fn new() -> Self {
        Self
    }

    fn parse(&self, tokens: Vec<char>) -> Vec<Instruction> {
        let mut iter = tokens.into_iter().peekable();
        self.inst(&mut iter)
    }

    fn inst<T>(&self, tokens: &mut Peekable<T>) -> Vec<Instruction>
    where
        T: Iterator<Item = char>,
    {
        let mut instructions = vec![];
        while let Some(c) = tokens.next() {
            let i: Instruction = match c {
                '>' | '<' | '+' | '-' | '.' | ',' => c.into(),
                '[' => {
                    let inner = self.inst(tokens);
                    Instruction::BLOCK(inner)
                }
                ']' => break,
                _ => break,
            };
            instructions.push(i);
        }
        instructions
    }
}

struct Interpreter {
    memory: Vec<i32>,
    pointer: usize,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            memory: vec![0; 10],
            pointer: 0,
        }
    }

    fn display_instructions(&self, instructions: &Vec<Instruction>, current: usize) {
        // print!("{}[2J", 27 as char);
        let mut inst = String::new();
        let mut carret = String::new();
        for (i, v) in instructions.iter().enumerate() {
            inst.push_str(&format!("{}", v.as_str()));
            if i == current {
                carret.push_str("^")
            } else {
                carret.push_str(" ")
            }
        }
        println!("{}", inst);
        println!("{}", carret);
        self.dump();
    }

    fn dump(&self) {
        for i in self.memory.iter() {
            print!("|{:02}", i);
        }
        println!("");
        for (i, _) in self.memory.iter().enumerate() {
            if i == self.pointer {
                print!(" ↑ ");
            } else {
                print!("   ");
            }
        }
        println!("");
    }

    fn eval(&mut self, instructions: &Vec<Instruction>) {
        use Instruction::*;
        let mut current = 0;

        loop {
            match instructions.get(current) {
                Some(INC) => self.increment(),
                Some(DEC) => self.decrement(),
                Some(NEXT) => self.next(),
                Some(PREV) => self.prev(),
                Some(OUT) => self.print(),
                Some(BLOCK(i)) => self.run_loop(i),
                _ => break,
            }
            current += 1;
        }
    }

    fn increment(&mut self) {
        self.memory[self.pointer] += 1;
    }

    fn decrement(&mut self) {
        self.memory[self.pointer] -= 1;
    }

    fn next(&mut self) {
        self.pointer += 1;
    }

    fn prev(&mut self) {
        self.pointer -= 1;
    }

    fn print(&self) {
        let value = self.memory[self.pointer];
        let c: char = std::char::from_u32(value as u32).unwrap();
        print!("{}", c);
    }

    fn run_loop(&mut self, instructions: &Vec<Instruction>) {
        loop {
            let value = self.memory[self.pointer];
            if value == 0 {
                break;
            } else {
                self.eval(instructions)
            }
        }
    }
}

fn main() {
    let src = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-------------.<<+++++++++++++++.>.+++.------.--------.>+..";
    let chars: Vec<char> = src.chars().collect();
    let instructions = Parser::new().parse(chars);

    let mut interpreter = Interpreter::new();
    interpreter.eval(&instructions);
}
