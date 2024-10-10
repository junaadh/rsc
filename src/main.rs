use std::{env, fs, io::Read};

use rsc::{ast::Ast, codegen::CodeGen, parser::Parser};

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
    let ast_ = Ast { program: prog };

    // prog.declarations.iter().for_each(|x| println!("{:#?}", x))
    let mut codegen = CodeGen;
    let program_code = codegen.generate(&ast_).unwrap();

    println!("{program_code}");
}
