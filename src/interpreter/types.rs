use std::collections::HashMap;


pub type State<N> = HashMap<String, N>;

#[derive(Debug)]
pub enum RuntimeError {
    VariableNotInitialized(String)
}

// #[derive(Debug)]
// pub enum Configuration {
//     Terminal(State),
//     NonTerminal(Box<Statement>,State)
// }