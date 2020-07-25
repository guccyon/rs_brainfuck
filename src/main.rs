use rs_brainfuck::{interpreter, parser};

fn main() {
    let src = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-------------.<<+++++++++++++++.>.+++.------.--------.>+..";
    let chars: Vec<char> = src.chars().collect();
    let instructions = parser::Parser::new().parse(chars);

    let mut interpreter = interpreter::Interpreter::new();
    interpreter.eval(&instructions);
}
