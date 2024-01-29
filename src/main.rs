use std::fs::File;
use clap::{Command, ArgAction, Arg};
use interpreter::{types::State, interpreter::eval_statement};
use parser::{ parse_string, parse_file};
use types::{ast::{Num, Statement}, errors::{ParserError, RuntimeError}};
use analyzer::{my_analyzer::MyAnalyzer, StaticAnalyzer, domains::{bounded_interval_domain::BoundedInterval, extended_sign_domain::ExtendedSign, interval_domain::Interval, sign_domain::Sign}, program::Program, AbstractState, HashMapState};

mod types;
mod interpreter;
mod parser;
mod examples;
mod analyzer;

type MyType = i128;

fn main() {


    let code = examples::TEST_REPEAT_UNTIL;

    let config = Config::new();
    std::env::set_var("print-token", config.print_token.to_string());
    std::env::set_var("print-cst", config.print_cst.to_string());
    std::env::set_var("print-ast", config.print_ast.to_string());
    std::env::set_var("print-pretty-cst", config.print_pretty_cst.to_string());
    std::env::set_var("print-pretty-ast", config.print_pretty_ast.to_string());


    let ast: Result<Statement<i128>, ParserError> = match config.filename {
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
    let prog: Program<ExtendedSign> = MyAnalyzer::<_, HashMapState<_>>::init(ast.clone());
    MyAnalyzer::analyze(prog, HashMapState::top());

    
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
}

#[derive(Debug)]
struct Config {
    filename: Option<String>,
    init_state: Option<State<MyType>>,
    print_token: bool,
    print_cst: bool,
    print_pretty_cst: bool,
    print_ast: bool,
    print_pretty_ast: bool,
    lower: Num
}

impl Config {
    pub fn new() -> Config{
        let matches = Command::new("While Interprer")
            .arg(Arg::new("filename"))
            .arg(Arg::new("state")     
                .long("state")
                // .help("BOH provaaaa")
                .help("Set initial state, must be in format <var-name>:<value>,<var-name>:<value>,...")
                // .long_help("Set initial state, must be in format <var-name>:<value>,<var-name>:<value>,...")
                .value_parser(parse_state))
            .arg(Arg::new("token")     .long("token")     .short('t').help("Print token list").action(ArgAction::SetTrue))
            .arg(Arg::new("ast")       .long("ast")       .short('a').help("Print raw ast")   .action(ArgAction::SetTrue))
            .arg(Arg::new("pretty-ast").long("pretty-ast").short('A').help("Print pretty ast").action(ArgAction::SetTrue))
            .arg(Arg::new("cst")       .long("cst")       .short('c').help("Print raw ast")   .action(ArgAction::SetTrue))
            .arg(Arg::new("pretty-cst").long("pretty-cst").short('C').help("Print pretty ast").action(ArgAction::SetTrue))
            .arg(Arg::new("lower").long("lower-bound").short('l').help("Lower bound").value_parser(clap::value_parser!(Num)).action(ArgAction::Set).required(true))
            .get_matches();
        
        Config{
            filename: matches.get_one::<String>("filename").cloned(),
            init_state: matches.get_one::<State<MyType>>("state").cloned(),
            print_token: matches.get_flag("token"),
            print_cst: matches.get_flag("cst"),
            print_pretty_cst:matches.get_flag("pretty-cst"),
            print_ast: matches.get_flag("ast"),
            print_pretty_ast:matches.get_flag("pretty-ast"),
            lower: *matches.get_one("lower").unwrap()
        }
    }
}


fn parse_state(str_state: &str) -> Result<State<MyType>, String> {
    str_state
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
        }).collect::<Result<_,_>>()
}    