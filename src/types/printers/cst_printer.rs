use std::fmt::Display;

use crate::types::cst::{Aexpr, Term, Factor, BexprAtomic, Bexpr, Statement, AssignStatements, Statements};



impl<N: Display> Display for Aexpr<N> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Aexpr::Add(a,t) => write!(fmt, "({a} + {t})"),
            Aexpr::Sub(a, t) => write!(fmt, "({a} - {t})"),
            Aexpr::Term( t) => write!(fmt, "{t}"),
            Aexpr::Opposite(f) => write!(fmt, "-{f}"),        }
    }
}
impl<N: Display> Display for Term<N> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Mul(t, f) => write!(fmt, "({t} * {f})"),
            Term::Div(t, f) => write!(fmt, "({t} / {f})"),
            Term::Factor(f) => write!(fmt, "{f}"),
        }
    }
}
impl<N: Display> Display for Factor<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Factor::Lit(n) => write!(f, "{n}"),
            Factor::Var(x) => write!(f, "{x}"),
            Factor::PreInc(x) => write!(f, "++{x}"), 
            Factor::PostInc(x) => write!(f, "{x}++"),
            Factor::PreDec(x) => write!(f, "--{x}"),
            Factor::PostDec(x) => write!(f, "{x}--"),            
            Factor::Aexpr(a) => write!(f, "{a}"),
        }
    }
}

impl<N: Display> Display for Bexpr<N> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Bexpr::And(b, ba) => write!(fmt, "({b} and {ba})"),
            Bexpr::Or(b, ba) => write!(fmt, "({b} or {ba})"),
            Bexpr::Atomic(ba) => write!(fmt, "{ba}"),
        }
    }
}
impl<N: Display> Display for BexprAtomic<N> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BexprAtomic::True => write!(fmt, "true"),
            BexprAtomic::False => write!(fmt, "false"),
            BexprAtomic::Equal(a1, a2) => write!(fmt, "({a1} == {a2})"),
            BexprAtomic::NotEqual(a1, a2) => write!(fmt, "({a1} != {a2})"),
            BexprAtomic::Less(a1, a2) => write!(fmt, "({a1} < {a2})"),
            BexprAtomic::LessEq(a1, a2) => write!(fmt, "({a1} <= {a2})"),
            BexprAtomic::Greater(a1, a2) => write!(fmt, "({a1} > {a2})"),
            BexprAtomic::GreaterEq(a1, a2) => write!(fmt, "({a1} => {a2})"),
            BexprAtomic::Not(b) => write!(fmt, "(not {})", b),
            BexprAtomic::Bexpr(b) => write!(fmt, "({b})"),
        }
    }
}

impl<N: Display> Display for Statement<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Statement::Skip => write!(f, "skip;"),
            Statement::AssignStm(s) =>  write!(f, "{s};"),
            Statement::Block(stms) => write!(f, "{stms}"),
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
                write!(f, "}}")
            }
            Statement::RepeatUntil(s, b) => {
                writeln!(f, "repeat {{")?;
                writeln!(f, "{}", add_tab(&s.to_string()))?;
                write!(f, "}} until {b};")
            }
            Statement::ForLoop(x, a, b, a_stm, s) => { 
                writeln!(f, "for ({x}:= {a}; {b}; {a_stm}) {{")?;
                writeln!(f, "{}", add_tab(&s.to_string()))?;
                write!(f, "}}")
            }
        }
    }
}
impl<N: Display> Display for AssignStatements<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignStatements::Assign(x, a) => write!(f, "{x}:= {a}"),
            AssignStatements::AddAssign(x, a) => write!(f, "{x}+= {a}"),
            AssignStatements::SubAssign(x, a) => write!(f, "{x}-= {a}"),
            AssignStatements::MulAssign(x, a) => write!(f, "{x}*= {a}"),
        }
    }
}
impl<N: Display> Display for Statements<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Statements::Composition(stms, stm) => write!(f,"{stms}\n{stm}"),
            Statements::Singleton(stm) => write!(f,"{stm}"),
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
