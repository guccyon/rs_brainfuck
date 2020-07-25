use super::Instruction;

pub struct Interpreter {
    memory: Vec<i32>,
    pointer: usize,
}

#[allow(dead_code)]
impl Interpreter {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 10],
            pointer: 0,
        }
    }

    pub fn eval(&mut self, instructions: &Vec<Instruction>) {
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
}
