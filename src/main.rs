#[derive(Debug)]
enum Token {
    NEXT,
    PREV,
    INC,
    DEC,
    IN,
    OUT,
    FWD,
    BWD,
}

struct Parser;

impl Parser {
    fn new() -> Self {
        Self
    }

    fn parse(&self, tokens: Vec<char>) -> Vec<Token> {
        tokens.iter().map(|c| self.parse_char(c).unwrap()).collect()
    }

    fn parse_char(&self, c: &char) -> Option<Token> {
        use Token::*;
        match c {
            '>' => Some(NEXT),
            '<' => Some(PREV),
            '+' => Some(INC),
            '-' => Some(DEC),
            '.' => Some(OUT),
            ',' => Some(IN),
            '[' => Some(FWD),
            ']' => Some(BWD),
            _ => None,
        }
    }
}

struct Interpreter {
    memory: Vec<i32>,
    pointer: usize,
    stack: Vec<usize>,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            memory: vec![0; 100],
            pointer: 0,
            stack: vec![],
        }
    }

    fn eval(&mut self, tokens: Vec<Token>) {
        use Token::*;
        let mut current = 0;
        loop {
            match tokens.get(current) {
                Some(INC) => self.increment(),
                Some(DEC) => self.decrement(),
                Some(NEXT) => self.next(),
                Some(PREV) => self.prev(),
                Some(OUT) => self.print(),
                Some(FWD) => current += self.foward(&tokens[current..]),
                Some(BWD) => current -= self.backward(&tokens[..current]),
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

    fn foward(&mut self, slices: &[Token]) -> usize {
        let value = self.memory[self.pointer];
        if value != 0 {
            self.stack.push(self.pointer);
            return 0;
        }

        let mut counter = 0;
        loop {
            let token = &slices[counter];
            if let &Token::BWD = token {
                break;
            }
            counter += 1;
        }
        return counter;
    }

    fn backward(&mut self, slices: &[Token]) -> usize {
        let value = self.memory[self.pointer];
        if value == 0 {
            self.stack.pop();
            return 0;
        }

        let mut counter = 1;
        loop {
            let token = &slices[slices.len() - counter];
            if let &Token::FWD = token {
                break;
            }
            counter += 1;
        }
        return counter;
    }
}

fn main() {
    let src = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-------------.<<+++++++++++++++.>.+++.------.--------.>+..";
    let tokens: Vec<char> = src.chars().collect();
    let tokens = Parser::new().parse(tokens);

    let mut interpreter = Interpreter::new();
    interpreter.eval(tokens);
}
