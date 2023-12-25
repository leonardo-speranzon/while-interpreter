use std::fs::File;

use crate::{types::{ast::{Statement, Num}, errors::ParserError}, parser::{lexer::Lexer, cst_parser::ConcreteParser, ast_parser::abstract_parse}};
use self::lexer::MyLexer;

mod cst_parser;
mod ast_parser;
mod lexer;

pub fn parse_string(str: String) -> Result<Statement<Num>,ParserError> {
    let lexer = MyLexer::from(str.as_str());
    return parse(lexer)
}
pub fn parse_file(file: File) -> Result<Statement<Num>,ParserError> {
    let lexer = MyLexer::from(file);
    return parse(lexer)
}

fn parse(lexer: impl Lexer)-> Result<Statement<Num>,ParserError>{   

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