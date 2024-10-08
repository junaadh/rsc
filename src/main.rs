use std::{env, fs, io::Read};

use rsc::parser::Parser;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let mut file = fs::File::open(args[1].as_str()).expect("Fuck");
    let mut buf = String::new();

    file.read_to_string(&mut buf).unwrap();

    let name = buf.as_str();

    // let lexer = Lexer::new(name);
    // // let mut symbols = Vec::new();

    // for token in lexer.tokenize() {
    //     println!("{token:?}");
    // }

    let mut parser = Parser::new(name);
    let prog = parser.program().unwrap();

    prog.declarations.iter().for_each(|x| println!("{:#?}", x))
}
