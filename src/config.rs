use std::{collections::binary_heap::Iter, str::FromStr};

use clap::{builder::PossibleValue, Arg, ArgAction, ArgMatches, Command, ValueEnum};

use crate::{analyzer::types::analyzer::IterationStrategy, interpreter::types::State, types::ast::Num};


#[derive(Debug)]
pub struct ParserConfig {
    pub filename: Option<String>,
    pub print_token: bool,
    pub print_cst: bool,
    pub print_pretty_cst: bool,
    pub print_ast: bool,
    pub print_pretty_ast: bool,
}

#[derive(Debug)]
pub struct InterpreterConfiguration {
    pub init_state: Option<State<Num>>,
}

#[derive(Debug, Clone)]
pub enum Domain{
    Sign,
    ExtendedSign,
    BoundedInterval
}
impl ValueEnum for Domain {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Sign, Self::ExtendedSign, Self::BoundedInterval]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Domain::Sign => Some(PossibleValue::new("sign")),
            Domain::ExtendedSign => Some(PossibleValue::new("extended-sign").alias("sign+")),
            Domain::BoundedInterval => Some(PossibleValue::new("bounded-interval")),
        }
    }
}

#[derive(Debug)]
pub struct AnalyzerConfiguration{
    pub domain: Domain,
    pub iteration_strategy: IterationStrategy,
    // pub init_state: Option<Box<HashMapState<>>>,
}

#[derive(Debug)]
pub enum Config {
    InterpreterConfiguration{parser_configuration: ParserConfig, config: InterpreterConfiguration},
    AnalyzerConfiguration{parser_configuration: ParserConfig, config: AnalyzerConfiguration}
}


impl Config {
    pub fn new() -> Config{

        let parser_args = [
            Arg::new("filename"),
            Arg::new("token")     .long("token")     .short('t').help("Print token list").action(ArgAction::SetTrue),
            Arg::new("ast")       .long("ast")       .short('a').help("Print raw ast")   .action(ArgAction::SetTrue),
            Arg::new("pretty-ast").long("pretty-ast").short('A').help("Print pretty ast").action(ArgAction::SetTrue),
            Arg::new("cst")       .long("cst")       .short('c').help("Print raw ast")   .action(ArgAction::SetTrue),
            Arg::new("pretty-cst").long("pretty-cst").short('C').help("Print pretty ast").action(ArgAction::SetTrue),
        ] ;


        let interpreter_cmd = Command::new("run")
            .arg(Arg::new("state")     
                .long("state")
                .help("Set initial state, must be in format <var-name>:<value>,<var-name>:<value>,...")
                // .long_help("Set initial state, must be in format <var-name>:<value>,<var-name>:<value>,...")
                .value_parser(parse_state::<Num>))
            .args(parser_args.clone());

        let analyzer_cmd = Command::new("analyze")
            .arg(Arg::new("domain").long("domain").short('d').value_parser(clap::builder::EnumValueParser::<Domain>::new()))
            .arg(Arg::new("widening") .short('W').help("Use widening") .action(ArgAction::SetTrue))
            .arg(Arg::new("narrowing").short('N').help("Use narrowing").action(ArgAction::SetTrue).requires("widening"))
            .args(parser_args);
            // .arg(Arg::new("lower").long("lower-bound").short('l').help("Lower bound").value_parser(clap::value_parser!(Num)).action(ArgAction::Set).required(true))

            

        let matches = Command::new("While Interprer")
            .subcommand(interpreter_cmd)
            .subcommand(analyzer_cmd)
            .subcommand_required(true)
            .get_matches();
    
        // println!("{:#?}", matches.);
        // todo!();
        match matches.subcommand() {
            Some(("run", sub_m)) => Config::InterpreterConfiguration { 
                parser_configuration: ParserConfig::from(sub_m),
                config: InterpreterConfiguration{
                    init_state: sub_m.get_one::<State<Num>>("state").cloned(),
                }
            },
            Some(("analyze", sub_m)) => Config::AnalyzerConfiguration{ 
                parser_configuration: ParserConfig::from(sub_m),
                config: AnalyzerConfiguration{
                    domain: sub_m.get_one::<Domain>("domain").cloned().unwrap_or(Domain::BoundedInterval),
                    iteration_strategy: match (sub_m.get_flag("widening"), sub_m.get_flag("narrowing")) {
                        (false, _) => IterationStrategy::Simple,
                        (true, false) => IterationStrategy::Widening,
                        (true, true) => IterationStrategy::WideningAndNarrowing,
                    }
                }
            },
            _ => todo!(),
        }
    }

    pub fn get_parser_conf(&self) -> &ParserConfig{
        match self {
            Config::InterpreterConfiguration { parser_configuration,.. } => parser_configuration,
            Config::AnalyzerConfiguration { parser_configuration, .. } => parser_configuration,
        }
    }
}

impl From<&ArgMatches> for ParserConfig {
    fn from(value: &ArgMatches) -> Self {
        ParserConfig{
            filename: value.get_one::<String>("filename").cloned(),
            print_token: value.get_flag("token"),
            print_cst: value.get_flag("cst"),
            print_pretty_cst:value.get_flag("pretty-cst"),
            print_ast: value.get_flag("ast"),
            print_pretty_ast:value.get_flag("pretty-ast"),
        }
    }
}

fn parse_state<T : FromStr>(str_state: &str) -> Result<State<T>, String> {
    str_state
        .split(',')
        .map(|pair_str|{
            match pair_str.split_once(':'){
                Some((var,val)) => {
                    match val.parse::<T>() {
                        Ok(n) => Ok((var.to_string(), n)),
                        Err(_) => Err(format!("Wrong format of state pair, expected '<var-name>:<value>' but found '{pair_str}'")),
                    }                                    
                }
                None => Err(format!("Wrong format of state pair, expected '<var-name>:<value>' but found '{pair_str}'")),    
            }
        }).collect::<Result<_,_>>()
}    