use crate::ast::{Environment, Node, NodeType, build_ast};
use std::any::Any;
use crate::parser::parse_line;


mod lexer;
mod parser;
mod ast;


fn main() {
    let content = lexer::read_file("source.txt"); // Read the Source Code
    let lines: Vec<&str> = content.split("\r\n").collect(); //Split the source code by EOL

    let mut environment = Environment::new();

    for line in lines { // Iterate through the Source Code
        let token_list = lexer::tokenise(line);
        let statement = build_ast(&token_list);
        if !statement.is_none() {
            statement.unwrap().eval(&mut environment);
        }
    }

    // let six = Node::integer(6);
    // let seven  = Node::integer(7);
    //
    // let test = Node::sum(Some(Box::new(six)), Some(Box::new(seven)));
    // test.eval(&mut environment);
}