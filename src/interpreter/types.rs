use std::collections::HashMap;

use crate::ast::Num;


pub type State = HashMap<String, Num>;

#[derive(Debug)]
pub enum RuntimeError {
    VariableNotInitialized(String)
}

// #[derive(Debug)]
// pub enum Configuration {
//     Terminal(State),
//     NonTerminal(Box<Statement>,State)
// }