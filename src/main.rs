

use std::{fs::File, env::Args};

use interpreter::types::{RuntimeError, State};

use crate::{parser::{types::ParserError, parse_string, parse_file}, interpreter::interpreter::eval_statement, ast::Num};


mod ast;
mod ast_printer;
mod interpreter;
mod parser;
#[allow(unused_variables)]
fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    //TODO -> add curly braces
    // let code = "x:= 55; y:= 5; z:= x + 2 + y";
    // let code = "x:= 55; while (5 + 4) = 5 - 3 do {x:=x+5;skip};skip;skip";
    // let code = "x:= 55; while (5 + 4) = 5 - 3 do {x:=x+5;skip};if(x<=22)and(55=3)then skip else skip;skip";
    // let code = "if true then if false then x:=1 else x:=2;y:=22 else x:=3";
    // let code = "if (not false and true) then skip else skip";
    let factorial = "y:=25;x:=1;while(not(y=0)) do {x:=x*y;y:=y-1;}";
    let long_loop = "y:=1000000;x:=1;while(not(y=0)) do {x:=x+y;y:=y-1;}";
    let infinite_loop = "while true do skip;"; 
    let while_false = "x:=1;while false do x:=2;";
    let inner_loop = r#"
        x:=0; y:=1;
        while x<=1000 do {
            x:=x+10;
            while (y<=10) do
                y:=y+1;
            y:=y*2;
        }"#;
    let gcd = r#"
        n1 := 814324; n2 := 1532;
        while (not n1 = n2) do 
            if n1 <= n2 then 
                n2:= n2 - n1;
            else
                n1:= n1 - n2;
        gcd:=n1;
    "#;
    let test_repeat_until = r#"
        x:=1023;
        repeat x-=110; until x<333;
    "#;
    let test_for_loop = r#"
        y:=1;
        for(x:=1; x<=500000; x:=x+1){
            y:=y+x;
        }
    "#;
    let code = test_repeat_until; 

    let config = match Config::new(std::env::args())  {
        Ok(c) => c,
        Err(err) => panic!("{}", err),
    };
    
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
    init_state: Option<State>
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, String>{
        
        args.next(); 

        let mut conf = Config{filename:None, init_state: None};
        while let Some(s) = args.next() {
            match s.as_str() {
                "--state" => {
                    if conf.init_state.is_some() {
                        return Err("Unexpected --state: states already set".to_string())
                    }
                    let str_state = args.next().unwrap();
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
                _ if !s.starts_with('-') && conf.filename.is_none() => conf.filename = Some(s),
                _ => return Err(format!("Unexpected arg: '{s}'"))
            }
        }
        println!("{:?}",conf);
        Ok(conf)


    }
} 