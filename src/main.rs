use std::{fs::File, env::Args};
use interpreter::{types::{RuntimeError, State}, interpreter::eval_statement};
use parser::{types::ParserError, parse_string, parse_file};
use ast::Num;

mod ast;
mod ast_printer;
mod interpreter;
mod parser;
mod examples;
fn main() {
    
    let code = examples::TEST_REPEAT_UNTIL; 

    let config = match Config::new(std::env::args())  {
        Ok(c) => c,
        Err(err) => panic!("{}", err),
    };
    std::env::set_var("print-cst", config.print_cst.to_string());
    std::env::set_var("print-ast", config.print_ast.to_string());
    std::env::set_var("print-pretty-cst", config.print_pretty_cst.to_string());
    std::env::set_var("print-pretty-ast", config.print_pretty_ast.to_string());
    
    
    let ast = match config.filename {
        Some(filename) => {
                let f = match File::open(&filename){
                    Ok(f) => f,
                    // Err(ref e) if e.kind() == std::io::ErrorKind:: => break,
                    Err(e) => panic!("Can't read from file: {}, err {}", filename, e),
                };
                parse_file(f)
            },
        None => parse_string(code.to_owned()),
    };
    
    
    let ast = match ast {
        Ok(ast) => ast,
        Err(err) => {
            match err {
                ParserError::UnexpectedEOF => 
                    println!("Unexpected EOF encountered"),
                ParserError::UnknownSymbol { pos:(l,c), symbol } => 
                    println!("Unknown symbol encountered: '{symbol}' at location {l}:{c}"),
                ParserError::UnexpectedToken { pos:(l,c), expected: None, found } =>
                    println!("Unexpected token encountered: {:?} at location {l}:{c}", found),
                ParserError::UnexpectedToken { pos:(l,c), expected: Some(expected), found } => 
                    println!("Expected token {:?} but found {:?} at location {l}:{c}", expected, found),
            }
            return;
        },
    };


    let final_state = eval_statement(
        &ast,
        config.init_state.unwrap_or(State::new())
    );
    match final_state {
        Ok(state) => println!("EVAL STM: {:?}", state),
        Err(RuntimeError::VariableNotInitialized(x)) => 
            println!("Runtime error: variable '{}' used before initialization", x),
    }
    

}

#[derive(Debug)]
struct Config{
    filename: Option<String>,
    init_state: Option<State>,
    print_cst: bool,
    print_pretty_cst: bool,
    print_ast: bool,
    print_pretty_ast: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, String>{
        
        args.next(); 

        let mut conf = Config{
            filename:None,
            init_state: None,
            print_cst: false,
            print_pretty_cst:false,
            print_ast: false,
            print_pretty_ast:false,
        };
        while let Some(s) = args.next() {
            match s.as_str() {
                "--state" => {
                    if conf.init_state.is_some() {
                        return Err("Unexpected --state: states already set".to_string())
                    }
                    let str_state = match args.next() {
                        Some(s) => s,
                        None => return Err("Empty --state argument".to_string()),
                    };
                    let state = str_state
                        .split(',')
                        .map(|pair_str|{
                            match pair_str.split_once(':'){
                                Some((var,val)) => {
                                    match val.parse::<Num>() {
                                        Ok(n) => Ok((var.to_string(), n)),
                                        Err(_) => Err(format!("Wrong format of state pair, expected '<var-name>:<value>' but found '{pair_str}'")),
                                    }                                    
                                }
                                None => Err(format!("Wrong format of state pair, expected '<var-name>:<value>' but found '{pair_str}'")),    
                            }
                        }).collect::<Result<_,_>>();
                    match state {
                        Ok(s) => conf.init_state = Some(s),
                        Err(e) => return Err(e)
                    }
                    
                },
                "-a" | "--print-ast" => conf.print_ast = true,
                "-c" | "--print-cst" => conf.print_cst = true,
                "-A" => conf.print_pretty_ast = true,
                "-C" => conf.print_pretty_cst = true,
                _ if !s.starts_with('-') && conf.filename.is_none() => conf.filename = Some(s),
                _ => return Err(format!("Unexpected arg: '{s}'"))
            }
        }
        // println!("{:?}",conf);
        Ok(conf)


    }
} 