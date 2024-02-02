use std::{collections::HashMap, fmt::Display, fs::File};
use analyzer::{domains::{bounded_interval_domain::BoundedInterval, extended_sign_domain::ExtendedSign, sign_domain::Sign}, analyzers::generic_analyzer::GenericAnalyzer, states::hashmap_state::HashMapState, types::{analyzer::StaticAnalyzer, program::{Label, Program, ProgramInterface}, state::AbstractState}};
use config::Config;
use interpreter::{types::State, interpreter::eval_statement};
use parser::{ parse_string, parse_file};
use types::{ast::Statement, errors::{ParserError, RuntimeError}};

use crate::analyzer::printers::print_stm_with_inv;

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
            let prog_int: Box<dyn ProgramInterface>; //: Program<dyn AbstractDomain> =   
            let result: HashMap<Label, Box<dyn Display>>;
            match config.domain {
                config::Domain::Sign => {
                    let prog: Program<Sign> = GenericAnalyzer::<_, HashMapState<_>>::init(ast.clone());
                    prog_int= Box::new(prog.clone()) as Box<dyn ProgramInterface>;
                    result = to_boxed_state(GenericAnalyzer::analyze(prog, HashMapState::top(), config.iteration_strategy));
                }
                config::Domain::ExtendedSign => {
                    let prog: Program<ExtendedSign> = GenericAnalyzer::<_, HashMapState<_>>::init(ast.clone());
                    prog_int= Box::new(prog.clone()) as Box<dyn ProgramInterface>;
                    result = to_boxed_state(GenericAnalyzer::analyze(prog, HashMapState::top(), config.iteration_strategy));
                }
                config::Domain::BoundedInterval => {
                    let prog: Program<BoundedInterval> = GenericAnalyzer::<_, HashMapState<_>>::init(ast.clone());
                    prog_int= Box::new(prog.clone()) as Box<dyn ProgramInterface>;
                    result = to_boxed_state(GenericAnalyzer::analyze(prog, HashMapState::top(), config.iteration_strategy));
                }
            };
            println!("╔═════════════════╗");
            println!("║ Analyzer Result ║");
            println!("╚═════════════════╝");
            println!("{}", print_stm_with_inv(ast));
            println!();
            println!("LOOP INVARIANTS:");
            let mut loop_labels = prog_int.get_loop_label().clone();
            loop_labels.sort();
            
            for (i, l) in loop_labels.iter().enumerate() {
                println!("(i{}) {}", i+1, result.get(l).unwrap())
            }

            // println!("{}", map_to_str(&result));

            println!();
            println!("FINAL INVARIANT: {}", result.get(&prog_int.get_end_label()).unwrap());
        },
    }

    

}

fn to_boxed_state<D:Display + 'static>(r: HashMap<Label,D>)->HashMap<Label, Box<dyn Display>>{
    r.into_iter()
    .map(|(l,s)|(l, Box::new(s) as Box<dyn Display>))
    .collect::<HashMap<_,_>>()
}