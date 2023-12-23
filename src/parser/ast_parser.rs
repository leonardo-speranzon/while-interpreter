use crate::types::ast::{self, ConcreteType};
use crate::types::cst;



pub fn abstract_parse<N: ConcreteType>(cst: &cst::Statements) -> ast::Statement<N> {
    parse_statements(cst)
}

fn parse_statements<N: ConcreteType>(cst: &cst::Statements) -> ast::Statement<N> {
    match cst{
        cst::Statements::Singleton( s) => parse_statement(s),
        cst::Statements::Composition(s1, s2) =>
            ast::Statement::Compose(
                Box::new(parse_statements(s1)),
                Box::new(parse_statement(s2))
            )
    }
}

fn parse_statement<N: ConcreteType>(cst: &cst::Statement) -> ast::Statement<N> {
    match cst{
        cst::Statement::Skip => 
            ast::Statement::Skip,
        cst::Statement::IfThenElse(b, s1, s2) => 
            ast::Statement::IfThenElse(
                Box::new(parse_bexpr(b)),
                Box::new(parse_statement(s1)),
                Box::new(parse_statement(s2))
            ),
        cst::Statement::While(b, s) =>
            ast::Statement::While(
                Box::new(parse_bexpr(b)),
                Box::new(parse_statement(s))
            ),
        cst::Statement::Block(stms) =>
            parse_statements(stms),
            
        cst::Statement::AssignStm(ass_stm) => 
            parse_assign_statement(ass_stm),
        
        cst::Statement::RepeatUntil(s, b) => {
            //repeat S until b <=> S;while(!b) do S
            let stm = parse_statement(s);
            ast::Statement::Compose(
                Box::new(stm.clone()),
                Box::new(ast::Statement::While(
                    Box::new(ast::Bexpr::Not(
                        Box::new(parse_bexpr(b))
                    )),
                    Box::new(stm)
                ))
            )
            
        }
        cst::Statement::ForLoop(x, a1, b,upd_stm, s )=>{
            //for(x:=a1; b; y:= a2) stm => x:=a1; while(b){stm; y:=a2;}
            ast::Statement::Compose(
                Box::new(ast::Statement::Assign(
                    x.clone(),
                    Box::new(parse_aexpr(a1))
                )),
                Box::new(ast::Statement::While(
                    Box::new(parse_bexpr(b)), 
                    Box::new(ast::Statement::Compose(
                        Box::new(parse_statement(s)),
                        Box::new(parse_assign_statement(upd_stm))
                    ))
                ))
            )
        }
        
    }
}

fn parse_assign_statement<N: ConcreteType>(cst: &cst::AssignStatements) -> ast::Statement<N> {
    match cst {
        cst::AssignStatements::Assign(x, a) => 
            ast::Statement::Assign(x.clone(), Box::new(parse_aexpr(a))),
        cst::AssignStatements::AddAssign(x, a) => 
            ast::Statement::Assign(
                x.clone(),
                Box::new(ast::Aexpr::Add(
                    Box::new(ast::Aexpr::Var(x.clone())), 
                    Box::new(parse_aexpr(a))
                ))
            ),
        cst::AssignStatements::SubAssign(x, a) => 
            ast::Statement::Assign(
                x.clone(),
                Box::new(ast::Aexpr::Sub(
                    Box::new(ast::Aexpr::Var(x.clone())), 
                    Box::new(parse_aexpr(a))
                ))
            ),
        cst::AssignStatements::MulAssign(x, a) => 
            ast::Statement::Assign(
                x.clone(),
                Box::new(ast::Aexpr::Mul(
                    Box::new(ast::Aexpr::Var(x.clone())), 
                    Box::new(parse_aexpr(a))
                ))
            ),
    }
}
fn parse_aexpr<N: ConcreteType>(cst: &cst::Aexpr) -> ast::Aexpr<N> {
    match cst {
        cst::Aexpr::Add(a, t) => 
            ast::Aexpr::Add(
                Box::new(parse_aexpr(a)),
                Box::new(parse_term(t))
            ),
        cst::Aexpr::Sub(a, t) => 
            ast::Aexpr::Sub(
                Box::new(parse_aexpr(a)),
                Box::new(parse_term(t))
            ),
        cst::Aexpr::Term(t) => parse_term(t),
        cst::Aexpr::Opposite(f) => ast::Aexpr::Sub(
            Box::new(ast::Aexpr::Num(0.into())),
            Box::new(parse_factor(&f))
        ),
        
    }
}
fn parse_term<N: ConcreteType>(cst: &cst::Term) -> ast::Aexpr<N> {
    match cst {
        cst::Term::Mul(t, f) => 
            ast::Aexpr::Mul(
                Box::new(parse_term(t)),
                Box::new(parse_factor(f))
            ),
        cst::Term::Factor(f) => parse_factor(f),
    }
}
fn parse_factor<N: ConcreteType>(cst: &cst::Factor) -> ast::Aexpr<N> {
    match cst {
        cst::Factor::Num(n) => ast::Aexpr::Num((*n).into()),
        cst::Factor::Var(x) => ast::Aexpr::Var(x.clone()),
        cst::Factor::Aexpr(a) => parse_aexpr(a),
    }
}



fn parse_bexpr<N: ConcreteType>(cst: &cst::Bexpr) -> ast::Bexpr<N> {
    match cst {
        cst::Bexpr::And(b, ba) =>
            ast::Bexpr::And(
                Box::new(parse_bexpr(b)),
                Box::new(parse_bexpr_atomic(ba))
            ),
        cst::Bexpr::Or(b, ba) =>
            ast::Bexpr::Not(
                Box::new(ast::Bexpr::And(
                    Box::new(ast::Bexpr::Not(Box::new(parse_bexpr(b)))),
                    Box::new(ast::Bexpr::Not(Box::new(parse_bexpr_atomic(ba))))
                ))
            ),            
        cst::Bexpr::Atomic(ba) =>
            parse_bexpr_atomic(ba),
    }
}
fn parse_bexpr_atomic<N: ConcreteType>(cst: &cst::BexprAtomic) -> ast::Bexpr<N> {
    match cst {
        cst::BexprAtomic::True => ast::Bexpr::True,
        cst::BexprAtomic::False => ast::Bexpr::False,
        cst::BexprAtomic::Equal(a1, a2) => 
            ast::Bexpr::Equal(
                Box::new(parse_aexpr(a1)),
                Box::new(parse_aexpr(a2))
            ),
        cst::BexprAtomic::LessEq(a1, a2) => 
            ast::Bexpr::LessEq(
                Box::new(parse_aexpr(a1)),
                Box::new(parse_aexpr(a2))
            ),
        cst::BexprAtomic::Not(ba) => 
            ast::Bexpr::Not(Box::new(parse_bexpr_atomic(ba))),
        cst::BexprAtomic::Bexpr(b) => parse_bexpr(b),

        //Desugar
        cst::BexprAtomic::NotEqual(a1, a2) => {
            ast::Bexpr::Not(
                Box::new(ast::Bexpr::Equal(
                    Box::new(parse_aexpr(a1)),
                    Box::new(parse_aexpr(a2))
                ))
            )
        }
        cst::BexprAtomic::Less(a1, a2) => 
            //a1<a2 <=> a1<=a2 and not(a1 = a2)
            ast::Bexpr::And(
                Box::new(ast::Bexpr::LessEq(
                    Box::new(parse_aexpr(a1)),
                    Box::new(parse_aexpr(a2))
                )),
                Box::new(ast::Bexpr::Not(
                    Box::new(ast::Bexpr::Equal(                    
                        Box::new(parse_aexpr(a1)),
                        Box::new(parse_aexpr(a2))
                    ))                    
                ))
            ),
        cst::BexprAtomic::GreaterEq(a1, a2) => 
            //a1>=a2 <=> not(a1<a2) <=> not(a1<=a2 and not(a1 = a2))
            ast::Bexpr::Not(
                Box::new(ast::Bexpr::And(
                    Box::new(ast::Bexpr::LessEq(
                        Box::new(parse_aexpr(a1)),
                        Box::new(parse_aexpr(a2))
                    )),
                    Box::new(ast::Bexpr::Not(
                        Box::new(ast::Bexpr::Equal(                    
                            Box::new(parse_aexpr(a1)),
                            Box::new(parse_aexpr(a2))
                        ))                    
                    ))
                ))
            ),
        cst::BexprAtomic::Greater(a1, a2) => 
            //a1>a2 <=> not(a1<=a2)
            ast::Bexpr::Not(
                Box::new(ast::Bexpr::LessEq(
                    Box::new(parse_aexpr(a1)),
                    Box::new(parse_aexpr(a2))
                )),
            ),
    }
}