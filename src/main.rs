use std::fs::File;
use analyzer::{domains::{bounded_interval_domain::BoundedInterval, extended_sign_domain::ExtendedSign, sign_domain::Sign}, analyzers::generic_analyzer::GenericAnalyzer, states::hashmap_state::HashMapState, types::{analyzer::StaticAnalyzer, program::Program, state::AbstractState}};
use config::Config;
use interpreter::{types::State, interpreter::eval_statement};
use parser::{ parse_string, parse_file};
use types::{ast::Statement, errors::{ParserError, RuntimeError}};

mod types;
mod interpreter;
mod parser;
mod examples;
mod analyzer;
mod config;


fn main() {


    let code = examples::TEST_REPEAT_UNTIL;

    let config = Config::new();

    let parser_config = config.get_parser_conf();
    std::env::set_var("print-token", parser_config.print_token.to_string());
    std::env::set_var("print-cst", parser_config.print_cst.to_string());
    std::env::set_var("print-ast", parser_config.print_ast.to_string());
    std::env::set_var("print-pretty-cst", parser_config.print_pretty_cst.to_string());
    std::env::set_var("print-pretty-ast", parser_config.print_pretty_ast.to_string());


    let ast: Result<Statement<i128>, ParserError> = match &parser_config.filename {
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

    match config {
        Config::InterpreterConfiguration { config, .. } => {
            let final_state = eval_statement(
                &ast,
                config.init_state.unwrap_or(State::new())
            );
            match final_state {
                Ok(state) => println!("EVAL STM: {:?}", state),
                Err(RuntimeError::VariableNotInitialized(x)) =>
                    println!("Runtime error: variable '{}' used before initialization", x),
                Err(RuntimeError::NotImplemented(str)) =>
                    println!("{str}")
            }
        },
        Config::AnalyzerConfiguration { config, .. } => {            
            match config.domain {
                config::AbstractDomain::Sign => {
                    let prog: Program<Sign> = GenericAnalyzer::<_, HashMapState<_>>::init(ast.clone());
                    GenericAnalyzer::analyze(prog, HashMapState::top());
                }
                config::AbstractDomain::ExtendedSign => {
                    let prog: Program<ExtendedSign> = GenericAnalyzer::<_, HashMapState<_>>::init(ast.clone());
                    GenericAnalyzer::analyze(prog, HashMapState::top());
                }
                config::AbstractDomain::BoundedInterval => {
                    let prog: Program<BoundedInterval> = GenericAnalyzer::<_, HashMapState<_>>::init(ast.clone());
                    GenericAnalyzer::analyze(prog, HashMapState::top());
                }
            }
        },
    }

    

}
