use super::Instruction;
use std::iter::Peekable;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, tokens: Vec<char>) -> Vec<Instruction> {
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
