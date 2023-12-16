mod lexer;
mod parser;
mod propositional;
mod syntax;
mod test_helpers;

use parser::Parser;

use std::env::args;
use std::fs;

fn main() {
    let Some(source_path) = args().nth(1) else {
        println!("{}", "USAGE: logicala [FILE]");
        return;
    };
    let Ok(source) = fs::read_to_string(&source_path) else {
        println!("logicala: {}: No such file or directory", source_path);
        return;
    };
    let parse = Parser::new(&source).parse();
    println!("{}", parse.debug_tree());
}
