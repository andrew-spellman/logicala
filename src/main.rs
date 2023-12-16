mod lexer;
mod propositional;
mod test_helpers;

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
    println!("{}", source);
}
