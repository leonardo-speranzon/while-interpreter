use std::str::FromStr;

use clap::{builder::{EnumValueParser, PossibleValue}, Arg, ArgAction, ArgMatches, Command, ValueEnum};

use crate::{analyzer::types::analyzer::IterationStrategy, interpreter::types::State, types::ast::Num};


#[derive(Debug)]
pub struct ParserConfig {
    pub filename: String,
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
    BoundedInterval,
    Congruence
}
impl ValueEnum for Domain {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Sign, Self::ExtendedSign, Self::BoundedInterval, Self::Congruence]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Domain::Sign => Some(PossibleValue::new("sign")),
            Domain::ExtendedSign => Some(PossibleValue::new("extended-sign").alias("sign+")),
            Domain::BoundedInterval => Some(PossibleValue::new("bounded-interval")),
            Domain::Congruence => Some(PossibleValue::new("cong")),
        }
    }
}

#[derive(Debug)]
pub struct AnalyzerConfiguration{
    pub domain: Domain,
    pub domain_config: Option<String>,
    pub iteration_strategy: IterationStrategy,
    pub init_state: Option<String>,
    pub print_iterations: bool,
}

#[derive(Debug)]
pub enum Config {
    InterpreterConfiguration{parser_configuration: ParserConfig, config: InterpreterConfiguration},
    AnalyzerConfiguration{parser_configuration: ParserConfig, config: AnalyzerConfiguration}
}


impl Config {
    pub fn new() -> Config{

        let parser_args = [
            Arg::new("filename").required(true),
            Arg::new("token")     .long("token")     .short('t').help("Print token list").action(ArgAction::SetTrue),
            Arg::new("ast")       .long("ast")       .short('a').help("Print raw ast")   .action(ArgAction::SetTrue),
            Arg::new("pretty-ast").long("pretty-ast").short('A').help("Print pretty ast").action(ArgAction::SetTrue),
            Arg::new("cst")       .long("cst")       .short('c').help("Print raw ast")   .action(ArgAction::SetTrue),
            Arg::new("pretty-cst").long("pretty-cst").short('C').help("Print pretty ast").action(ArgAction::SetTrue),
        ];


        let interpreter_cmd = Command::new("run")
            .arg(Arg::new("state")     
                .long("state")
                .help("Set initial state, must be in format <var-name>:<value>;<var-name>:<value>;...")
                // .long_help("Set initial state, must be in format <var-name>:<value>;<var-name>:<value>;...")
                .value_parser(parse_state::<Num>))
            .args(parser_args.clone())
            .arg_required_else_help(true);

        let analyzer_cmd = Command::new("analyze")
            .arg(Arg::new("domain").long("domain").short('d').value_parser(EnumValueParser::<Domain>::new()).default_value("bounded-interval"))
            .arg(Arg::new("widening") .short('W').help("Use widening") .action(ArgAction::SetTrue))
            .arg(Arg::new("narrowing").short('N').help("Use narrowing").action(ArgAction::SetTrue).requires("widening"))
            .arg(Arg::new("state")     
                .long("state")
                .help("Set initial state, must be in format <var-name>:<value>;<var-name>:<value>;...")
                // .long_help("Set initial state, must be in format <var-name>:<value>;<var-name>:<value>;...")
                // .if
                // .value_parser(parse_abs_state::<BoundedInterval>)
            )
            .arg(Arg::new("config").long("conf").help("Set the configuration for the domain"))
            .arg(Arg::new("iter").long("iter").short('i').help("Print analyzer iterations").action(ArgAction::SetTrue))
            .args(parser_args)
            .arg_required_else_help(true);
            // .arg(Arg::new("lower").long("lower-bound").short('l').help("Lower bound").value_parser(clap::value_parser!(Num)).action(ArgAction::Set).required(true))

            
        let matches = Command::new("While Interpreter")
            .subcommand(interpreter_cmd)
            .subcommand(analyzer_cmd)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .get_matches();
    
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
                    domain_config: sub_m.get_one::<String>("config").cloned(),
                    iteration_strategy: match (sub_m.get_flag("widening"), sub_m.get_flag("narrowing")) {
                        (false, _) => IterationStrategy::Simple,
                        (true, false) => IterationStrategy::Widening,
                        (true, true) => IterationStrategy::WideningAndNarrowing,
                    },
                    init_state: sub_m.get_one::<String>("state").cloned(), //sub_m.get_one::<HashMapState<BoundedInterval>>("state").cloned(),
                    print_iterations: sub_m.get_flag("iter"),
                }
            },
            _ => unreachable!(),
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
            filename: match value.get_one::<String>("filename"){
                Some(f) => f.clone(),
                None => unreachable!(),
            },
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
        .split(';')
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