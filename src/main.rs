use std::{collections::HashMap, fmt::Display, fs::File};
use analyzer::{analyzers::generic_analyzer::GenericAnalyzer, domains::{bounded_interval_domain::BoundedIntervalDomain, congruence_domain::CongruenceDomain, extended_sign_domain::ExtendedSignDomain, sign_domain::SignDomain}, states::hashmap_state::HashMapState, types::{analyzer::StaticAnalyzer, domain::AbstractDomain, program::{Label, Program, ProgramInterface}, state::AbstractState}};
use config::{AnalyzerConfiguration, Config};
use interpreter::{types::State, interpreter::eval_statement};
use parser::parse_file;
use types::{ast::{Num, Statement}, errors::RuntimeError};

use crate::{analyzer::printers::print_stm_with_inv, types::lit_interval::LitInterval};


mod types;
mod interpreter;
mod parser;
mod examples;
mod analyzer;
mod config;


fn main() {
    let config = Config::new();

    let parser_config = config.get_parser_conf();
    std::env::set_var("print-token", parser_config.print_token.to_string());
    std::env::set_var("print-cst", parser_config.print_cst.to_string());
    std::env::set_var("print-ast", parser_config.print_ast.to_string());
    std::env::set_var("print-pretty-cst", parser_config.print_pretty_cst.to_string());
    std::env::set_var("print-pretty-ast", parser_config.print_pretty_ast.to_string());


    
    let file = match File::open(&parser_config.filename){
        Ok(f) => f,
        Err(e) => panic!("Can't read from file: {}, err {}", parser_config.filename, e),
    };
    match config {
        Config::InterpreterConfiguration { config, .. } => {
            let ast: Statement<Num> = match parse_file(file) {
                Ok(ast) => ast,
                Err(err) => panic!("{err}")
            };

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
            std::env::set_var("print-iterations",config.print_iterations.to_string());
            // println!("{:?}",config);

            let ast: Statement<LitInterval> = match parse_file(file) {
                Ok(ast) => ast,
                Err(err) => panic!("{err}")
            };

            let (prog_int, result) = match config.domain {
                config::Domain::Sign => analyze::<SignDomain>(ast.clone(), config),                
                config::Domain::ExtendedSign => analyze::<ExtendedSignDomain>(ast.clone(), config),
                config::Domain::BoundedInterval => analyze::<BoundedIntervalDomain>(ast.clone(), config),
                config::Domain::Congruence => analyze::<CongruenceDomain>(ast.clone(), config),
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

fn analyze<D: AbstractDomain + 'static>(ast: Statement<LitInterval>, config: AnalyzerConfiguration) -> (Box<dyn ProgramInterface>, HashMap<Label, Box<dyn Display>>){
    if let Err(e) = D::set_config(config.domain_config) {
        panic!("Failed configuration :{e}")
    }

    let prog: Program<D> = GenericAnalyzer::<_, HashMapState<_>>::init(ast);
    let prog_int= Box::new(prog.clone()) as Box<dyn ProgramInterface>;
    let result = to_boxed_state(GenericAnalyzer::analyze(
        prog,
        config.init_state
            .map(|s|s.parse().unwrap())
            .unwrap_or(HashMapState::top()),
        config.iteration_strategy
    ));
    (prog_int, result)
}