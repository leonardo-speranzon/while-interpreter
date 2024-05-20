use std::fs::File;

use crate::{parser::{ast_parser::abstract_parse, cst_parser::ConcreteParser, lexer::Lexer}, types::{ast::{NumLiteral, Statement}, errors::ParserError}};
use self::lexer::MyLexer;

mod cst_parser;
mod ast_parser;
mod lexer;

#[allow(dead_code)]
pub fn parse_string<N: NumLiteral>(str: String) -> Result<Statement<N>, ParserError<N>> {
    let lexer = MyLexer::from(str.as_str());
    return parse(lexer)
}
pub fn parse_file<N: NumLiteral>(file: File) -> Result<Statement<N>,ParserError<N>> {
    let lexer = MyLexer::from(file);
    return parse(lexer)
}

fn parse<N: NumLiteral>(lexer: impl Lexer<N>)-> Result<Statement<N>,ParserError<N>>{   

    if std::env::var("print-token").is_ok_and(|s|s=="true") {
        println!("╔════════╗");
        println!("║ Tokens ║");
        println!("╚════════╝"); 
    }
    let cst_parser = ConcreteParser::new(lexer);
    let cst = cst_parser.parse()?;
    
    if std::env::var("print-token").is_ok_and(|s|s=="true") {
        print!("\n\n");
    }

    if std::env::var("print-cst").is_ok_and(|s|s=="true") {
        println!("╔═════════╗");
        println!("║ Raw CST ║");
        println!("╚═════════╝");
        println!("{:?}", &cst);
        println!();
    }
    if std::env::var("print-pretty-cst").is_ok_and(|s|s=="true") {
        println!("╔════════════╗");
        println!("║ Pretty CST ║");
        println!("╚════════════╝");
        println!("{}", &cst);
        println!();
    }


    let ast = abstract_parse(&cst);


    if std::env::var("print-ast").is_ok_and(|s|s=="true") {
        println!("╔═════════╗");
        println!("║ Raw AST ║");
        println!("╚═════════╝");
        println!("{:?}", &ast);
        println!();
    }
    if std::env::var("print-pretty-ast").is_ok_and(|s|s=="true") {
        println!("╔════════════╗");
        println!("║ Pretty AST ║");
        println!("╚════════════╝");
        println!("{}", &ast);
        println!();
    }

    return Ok(ast);
}