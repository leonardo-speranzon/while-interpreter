use crate::types::ast::{self, NumLiteral, PostOp, PreOp};
use crate::types::cst;



pub fn abstract_parse<N: NumLiteral>(cst: &cst::Statements<N>) -> ast::Statement<N> {
    parse_statements(cst)
}

fn parse_statements<N: NumLiteral>(cst: &cst::Statements<N>) -> ast::Statement<N> {
    match cst{
        cst::Statements::Singleton( s) => parse_statement(s),
        cst::Statements::Composition(s1, s2) =>
            ast::Statement::Compose(
                Box::new(parse_statements(s1)),
                Box::new(parse_statement(s2))
            )
    }
}

fn parse_statement<N: NumLiteral>(cst: &cst::Statement<N>) -> ast::Statement<N> {
    match cst{
        cst::Statement::<N>::Skip => 
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

fn parse_assign_statement<N: NumLiteral>(cst: &cst::AssignStatements<N>) -> ast::Statement<N> {
    match cst {
        cst::AssignStatements::Assign(x, a) => 
            ast::Statement::Assign(x.clone(), Box::new(parse_aexpr(a))),
        cst::AssignStatements::AddAssign(x, a) => 
            ast::Statement::Assign(
                x.clone(),
                Box::new(ast::Aexpr::BinOp(
                    ast::Operator::Add,
                    Box::new(ast::Aexpr::Var(x.clone())), 
                    Box::new(parse_aexpr(a))
                ))
            ),
        cst::AssignStatements::SubAssign(x, a) => 
            ast::Statement::Assign(
                x.clone(),
                Box::new(ast::Aexpr::BinOp(
                    ast::Operator::Sub,
                    Box::new(ast::Aexpr::Var(x.clone())), 
                    Box::new(parse_aexpr(a))
                ))
            ),
        cst::AssignStatements::MulAssign(x, a) => 
            ast::Statement::Assign(
                x.clone(),
                Box::new(ast::Aexpr::BinOp(
                    ast::Operator::Mul,
                    Box::new(ast::Aexpr::Var(x.clone())), 
                    Box::new(parse_aexpr(a))
                ))
            ),
    }
}
fn parse_aexpr<N: NumLiteral>(cst: &cst::Aexpr<N>) -> ast::Aexpr<N> {
    match cst {
        cst::Aexpr::Add(a, t) => 
            ast::Aexpr::BinOp(
                ast::Operator::Add,
                Box::new(parse_aexpr(a)),
                Box::new(parse_term(t))
            ),
        cst::Aexpr::Sub(a, t) => 
            ast::Aexpr::BinOp(
                ast::Operator::Sub,
                Box::new(parse_aexpr(a)),
                Box::new(parse_term(t))
            ),
        cst::Aexpr::Term(t) => parse_term(t),
        cst::Aexpr::Opposite(f) => ast::Aexpr::BinOp(
            ast::Operator::Sub,
            Box::new(ast::Aexpr::Lit(0.into())),
            Box::new(parse_factor(&f))
        ),
        
    }
}
fn parse_term<N: NumLiteral>(cst: &cst::Term<N>) -> ast::Aexpr<N> {
    match cst {
        cst::Term::Mul(t, f) => 
            ast::Aexpr::BinOp(
                ast::Operator::Mul,
                Box::new(parse_term(t)),
                Box::new(parse_factor(f))
            ),
        cst::Term::Div(t, f) => 
            ast::Aexpr::BinOp(
                ast::Operator::Div,
                Box::new(parse_term(t)),
                Box::new(parse_factor(f))
            ),
        cst::Term::Factor(f) => parse_factor(f),
    }
}
fn parse_factor<N: NumLiteral>(cst: &cst::Factor<N>) -> ast::Aexpr<N> {
    match cst {
        cst::Factor::Lit(n) => ast::Aexpr::Lit((*n).into()),
        cst::Factor::Var(x) => ast::Aexpr::Var(x.clone()),
        cst::Factor::PreInc(x) => ast::Aexpr::PreOp(PreOp::Inc, x.clone()),
        cst::Factor::PostInc(x) => ast::Aexpr::PostOp(PostOp::Inc, x.clone()),
        cst::Factor::PreDec(x) => ast::Aexpr::PreOp(PreOp::Dec,x.clone()),
        cst::Factor::PostDec(x) => ast::Aexpr::PostOp(PreOp::Dec, x.clone()),
        cst::Factor::Aexpr(a) => parse_aexpr(a),
    }
}



fn parse_bexpr<N: NumLiteral>(cst: &cst::Bexpr<N>) -> ast::Bexpr<N> {
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
fn parse_bexpr_atomic<N: NumLiteral>(cst: &cst::BexprAtomic<N>) -> ast::Bexpr<N> {
    match cst {
        cst::BexprAtomic::<N>::True => ast::Bexpr::<N>::True,
        cst::BexprAtomic::<N>::False => ast::Bexpr::False,
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
            // a1<a2 <=> not(a2 <= a1)
            ast::Bexpr::Not(
                Box::new(ast::Bexpr::LessEq(
                    Box::new(parse_aexpr(a2)),
                    Box::new(parse_aexpr(a1)),
                ))
            ),
        cst::BexprAtomic::GreaterEq(a1, a2) => 
            // a1>=a2 <=> a2<=a1
            ast::Bexpr::LessEq(
                Box::new(parse_aexpr(a2)),
                Box::new(parse_aexpr(a1)),
            ),
        cst::BexprAtomic::Greater(a1, a2) => 
            // a1>a2 <=> not(a1<=a2)
            ast::Bexpr::Not(
                Box::new(ast::Bexpr::LessEq(
                    Box::new(parse_aexpr(a1)),
                    Box::new(parse_aexpr(a2))
                )),
            ),
    }
}