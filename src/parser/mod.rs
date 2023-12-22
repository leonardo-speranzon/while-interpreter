use std::fs::File;

use crate::{ast::Statement, parser::{lexer::Lexer, cst_parser::ConcreteParser, ast_parser::abstract_parse}};

use self::{types::ParserError, lexer::MyLexer};

mod cst_parser;
mod cst_printer;
mod ast_parser;
mod lexer;
mod cst;
pub mod types;

const DEBUG:bool = true;

pub fn parse_string(str: String) -> Result<Statement,ParserError> {
    let mut lexer = MyLexer::from(str.as_str());
    if DEBUG {
        println!("TOKENS: ");
        loop{
            match lexer.next() {
                Ok(Some(t)) => print!("{:?} ",t),
                Ok(None) => break,
                Err(err) => {println!("{:?}",err);break;},
            }
        }
        println!("\n==============================");
        lexer = MyLexer::from(str.as_str());
    }
    return parse(lexer)
}
pub fn parse_file(file: File) -> Result<Statement,ParserError> {
    // if DEBUG {
    //     let mut lexer = MyLexer::from(file);
    //     println!("TOKENS: ");
    //     loop{
    //         match lexer.next() {
    //             Ok(Some(t)) => print!("{:?} ",t),
    //             Ok(None) => break,
    //             Err(err) => {println!("{:?}",err);break;},
    //         }
    //     }
    //     println!("\n==============================");
    //     // lexer = MyLexer::from(file);
    // }

    let lexer = MyLexer::from(file);
    return parse(lexer)
}

fn parse(lexer: impl Lexer)-> Result<Statement,ParserError>{    

    let cst_parser = ConcreteParser::new(lexer);
    let cst = cst_parser.parse()?;

    if std::env::var("print-cst").is_ok_and(|s|s=="true") {
        println!("Raw CST:");
        println!("{:?}", &cst);
        println!("==============================");
    }
    if std::env::var("print-pretty-cst").is_ok_and(|s|s=="true") {
        println!("Pretty CST:");
        println!("{}", &cst);
        println!("==============================");
    }


    let ast = abstract_parse(&cst);


    if std::env::var("print-ast").is_ok_and(|s|s=="true") {
        println!("Raw AST:");
        println!("{:?}", &ast);
        println!("==============================");
    }
    if std::env::var("print-pretty-ast").is_ok_and(|s|s=="true") {
        println!("Pretty AST:");
        println!("{}", &ast);
        println!("==============================");
    }

    return Ok(ast);
}