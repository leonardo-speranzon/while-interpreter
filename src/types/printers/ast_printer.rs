use std::fmt::Display;
use crate::types::ast::{Aexpr, Bexpr, Statement};


impl<N: Display> Display for Aexpr<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Aexpr::Num(n) => 
                write!(f, "{}", n),
            Aexpr::Var(x) => 
                write!(f, "{}", x),
            Aexpr::Add(a1, a2) => 
                write!(f, "({} + {})", a1, a2),
            Aexpr::Mul(a1, a2) => 
                write!(f, "({} * {})", a1, a2),            
            Aexpr::Sub(a1, a2) => 
               write!(f, "({} - {})", a1, a2),
        }
    }
}

impl<N: Display> Display for Bexpr<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bexpr::True => 
                write!(f, "true"),
            Bexpr::False => 
                write!(f, "false"),
            Bexpr::Equal(a1, a2) => 
                write!(f, "({} = {})", a1, a2),
            Bexpr::LessEq(a1, a2) => 
                write!(f, "({} <= {})", a1, a2),
            Bexpr::Not(b) =>
                write!(f, "(not {})", b),
            Bexpr::And(b1, b2) => 
                write!(f, "({} and {})", b1, b2),
        }
    }
}

impl<N: Display> Display for Statement<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Assign(x, a) => 
                write!(f, "{x}:= {a};"),
            Statement::Skip => 
                write!(f, "skip;"),
            Statement::Compose(s1, s2) => {
                write!(f, "{s1}\n{s2}")
            }
            Statement::IfThenElse(b, s1, s2) => {
                writeln!(f, "if {b} then {{")?;
                writeln!(f, "{}", add_tab(&s1.to_string()))?;
                writeln!(f,"}} else {{")?;
                writeln!(f, "{}", add_tab(&s2.to_string()))?;
                write!(f,"}}")
            }
            Statement::While(b, s) => {
                writeln!(f, "while {b} do {{")?;
                writeln!(f, "{}", add_tab(&s.to_string()))?;
                write!(f,"}}")
            },
        }
    }
}
fn add_tab(str: &str)-> String{
    let mut tabbed_string = str
        .lines()
        .fold(String::new(), |a,s| a+"    "+s+"\n");

    match str.chars().last() {
        Some('\n') => (),
        Some(_) => {tabbed_string.pop();},
        None => (),
    }

    return tabbed_string;
}
