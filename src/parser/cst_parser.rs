use std::marker::PhantomData;

use crate::types::{ast::NumLiteral, cst::{Aexpr, AssignStatements, Bexpr, BexprAtomic, Factor, Statement, Statements, Term}, errors::ParserError, tokens::Token};
use super::lexer::Lexer;



pub struct ConcreteParser<N, L: Lexer<N>>{
    lexer: L,
    num_lit: PhantomData<N>
}

impl<N: NumLiteral, L: Lexer<N>> ConcreteParser<N, L> {
    pub fn new(lexer: L) -> Self { Self { lexer, num_lit: Default::default() } }


    pub fn parse(mut self)->Result<Statements<N>,ParserError<N>>{
        let ast = self.parse_statements()?;
        if let Some(_) = self.lexer.peek(){
            Err(self.lexer.unexpected_error())
        }else{
            Ok(ast)
        }
    }
    
    
    fn parse_statements(&mut self) -> Result<Statements<N>,ParserError<N>> {
        let s = self.parse_statement()?;
        let mut stms = Statements::<N>::Singleton(Box::new(s));

        loop {
            match self.lexer.peek() {
                None => break,
                Some(Token::CurlyClose) => break,
                Some(_) => {
                    let s = self.parse_statement()?;
                    stms = Statements::<N>::Composition(Box::new(stms), Box::new(s));
                }
            }
        }
        Ok(stms)
    }



    fn parse_statement(&mut self) -> Result<Statement<N>,ParserError<N>> {
        match self.lexer.peek() {
            Some(Token::Skip) => {
                self.lexer.match_next(Token::Skip)?;
                self.lexer.match_next(Token::Semicolon)?;
                Ok(Statement::<N>::Skip)
            }
            Some(Token::Id(_)) => {
                let ass_stm = self.parse_assign_statement()?;
                self.lexer.match_next(Token::Semicolon)?;
                
                Ok(Statement::AssignStm(Box::new(ass_stm)))
            }
            Some(Token::If) => {
                self.lexer.match_next(Token::If)?;
                let b = self.parse_bexpr()?;
                self.lexer.match_next(Token::Then)?;
                let s1 = self.parse_statement()?;
                self.lexer.match_next(Token::Else)?;
                let s2 = self.parse_statement()?;
                Ok(Statement::IfThenElse(Box::new(b), Box::new(s1), Box::new(s2)))
            }
            Some(Token::While) => {
                self.lexer.match_next(Token::While)?;
                let b = self.parse_bexpr()?;
                self.lexer.match_next(Token::Do)?;
                let s = self.parse_statement()?;
                Ok(Statement::While(Box::new(b), Box::new(s)))
            }
            Some(Token::Repeat) => {
                self.lexer.match_next(Token::Repeat)?;
                let s = self.parse_statement()?;
                self.lexer.match_next(Token::Until)?;
                let b = self.parse_bexpr()?;
                self.lexer.match_next(Token::Semicolon)?;
                Ok(Statement::RepeatUntil(Box::new(s), Box::new(b)))
            }
            Some(Token::For) => {
                self.lexer.match_next(Token::For)?;
                self.lexer.match_next(Token::BracketOpen)?;
                let x = self.parse_id()?;
                self.lexer.match_next(Token::Assign)?;
                let a1 = self.parse_aexpr()?;
                self.lexer.match_next(Token::Semicolon)?;
                let b = self.parse_bexpr()?;
                self.lexer.match_next(Token::Semicolon)?;
                let upd_stm = self.parse_assign_statement()?;
                self.lexer.match_next(Token::BracketClose)?;
                let s = self.parse_statement()?;

                Ok(Statement::ForLoop(x, Box::new(a1), Box::new(b), Box::new(upd_stm), Box::new(s)))
            }
            Some(Token::CurlyOpen) => {
                self.lexer.match_next(Token::CurlyOpen)?;
                let stms = self.parse_statements()?;
                self.lexer.match_next(Token::CurlyClose)?;
                Ok(Statement::Block(Box::new(stms)))
            },
            _ => Err(self.lexer.unexpected_error())
        }
    }
    fn parse_assign_statement(&mut self) -> Result<AssignStatements<N>,ParserError<N>> {
        let x = self.parse_id()?;
        let stm_constructor = match self.lexer.peek() {
            Some(Token::Assign) => {
                self.lexer.match_next(Token::Assign)?;
                AssignStatements::<N>::Assign
            },
            Some(Token::AddAssign) => {
                self.lexer.match_next(Token::AddAssign)?;
                AssignStatements::<N>::AddAssign
            },
            Some(Token::SubAssign) => {
                self.lexer.match_next(Token::SubAssign)?;
                AssignStatements::<N>::SubAssign
            },
            Some(Token::MulAssign) => {
                self.lexer.match_next(Token::MulAssign)?;
                AssignStatements::<N>::MulAssign
            },
            _ => return Err(self.lexer.unexpected_error()),
        };
        let a = self.parse_aexpr()?;
        Ok(stm_constructor(x, Box::new(a)))
    }

    fn parse_id(&mut self) -> Result<String, ParserError<N>>{
        let x =  match self.lexer.peek(){
            Some(Token::Id(x)) => x,
            _ => return Err(self.lexer.unexpected_error())
        };
        self.lexer.match_next(Token::Id(x.clone()))?;
        return Ok(x)
    }
    
    fn parse_aexpr(&mut self) -> Result<Aexpr<N>,ParserError<N>> {
        if let Some(Token::Minus) = self.lexer.peek() {
            self.lexer.match_next(Token::Minus)?;    
            return Ok(Aexpr::<N>::Opposite(Box::new(
                self.parse_factor()?
            )));   
        }

        let t = self.parse_term()?;
        let mut aexpr = Aexpr::<N>::Term(Box::new(t));
        loop {
            match self.lexer.peek() {
                Some(Token::Plus) => {
                    self.lexer.match_next(Token::Plus)?;    
                    let t = self.parse_term()?;
                    aexpr = Aexpr::<N>::Add(Box::new(aexpr), Box::new(t));
                },
                Some(Token::Minus) => {
                    self.lexer.match_next(Token::Minus)?;    
                    let t = self.parse_term()?;
                    aexpr = Aexpr::<N>::Sub(Box::new(aexpr), Box::new(t));
                }
                _ => break,
            }
        }
        return Ok(aexpr);
    }
    fn parse_term(&mut self) -> Result<Term<N>,ParserError<N>> {
        let f = self.parse_factor()?;
        let mut term = Term::<N>::Factor(Box::new(f));
        loop {
            match self.lexer.peek() {
                Some(Token::Mul) => {
                    self.lexer.match_next(Token::Mul)?;     
                    let f = self.parse_factor()?;
                    term = Term::<N>::Mul(Box::new(term), Box::new(f));
                }
                Some(Token::Div) => {
                    self.lexer.match_next(Token::Div)?;     
                    let f = self.parse_factor()?;
                    term = Term::<N>::Div(Box::new(term), Box::new(f));
                }
                _ => break,
            }
        }
        return Ok(term);
    }
    fn parse_factor(&mut self) -> Result<Factor<N>,ParserError<N>> {
        match self.lexer.peek() {
            Some(Token::Lit(n)) => {
                self.lexer.match_next(Token::Lit(n))?;
                Ok(Factor::Lit(n))
            },
            Some(Token::Id(x)) => {
                self.lexer.match_next(Token::Id(x.clone()))?;
                match self.lexer.peek() {
                    Some(Token::Inc) => {
                        self.lexer.match_next(Token::Inc)?;
                        Ok(Factor::PostInc(x))
                    }
                    Some(Token::Dec) => {
                        self.lexer.match_next(Token::Dec)?;
                        Ok(Factor::PostDec(x))
                    }
                    _ => Ok(Factor::Var(x))
                }
            },
            Some(Token::BracketOpen) => {
                self.lexer.match_next(Token::BracketOpen)?;
                let a = self.parse_aexpr()?;
                self.lexer.match_next(Token::BracketClose)?;
                Ok(Factor::Aexpr(Box::new(a)))
            }
            Some(Token::Inc) => {
                self.lexer.match_next(Token::Inc)?;
                Ok(Factor::PreInc(self.parse_id()?))
            }
            Some(Token::Dec) => {
                self.lexer.match_next(Token::Dec)?;       
                Ok(Factor::PreDec(self.parse_id()?))
            }
            _ => Err(self.lexer.unexpected_error())
        }    
    }
    
    
    
    fn parse_bexpr(&mut self) -> Result<Bexpr<N>,ParserError<N>> {
        let ba = self.parse_bexpr_atom()?;
        let mut bexpr = Bexpr::<N>::Atomic(Box::new(ba));
        loop {
            match self.lexer.peek() {
                Some(Token::And) => {
                    self.lexer.match_next(Token::And)?;     
                    let ba = self.parse_bexpr_atom()?;
                    bexpr = Bexpr::<N>::And(Box::new(bexpr), Box::new(ba));
                }
                Some(Token::Or) => {
                    self.lexer.match_next(Token::Or)?;     
                    let ba = self.parse_bexpr_atom()?;
                    bexpr = Bexpr::<N>::Or(Box::new(bexpr), Box::new(ba));
                }
                _ => break,
            }
        }
        return Ok(bexpr);
    }
    fn parse_bexpr_atom(&mut self) -> Result<BexprAtomic<N>,ParserError<N>> {
        match self.lexer.peek() {
            Some(Token::True) => {
                self.lexer.match_next(Token::True)?;
                Ok(BexprAtomic::<N>::True)
            },
            Some(Token::False) => {
                self.lexer.match_next(Token::False)?;
                Ok(BexprAtomic::<N>::False)
            },
            Some(Token::Not) => {
                self.lexer.match_next(Token::Not)?;
                let b =self.parse_bexpr_atom()?;
                Ok(BexprAtomic::<N>::Not(Box::new(b)))
            },
            Some(Token::BracketOpen) => {
                self.lexer.match_next(Token::BracketOpen)?;
                let b = self.parse_bexpr()?;
                self.lexer.match_next(Token::BracketClose)?;
                Ok(BexprAtomic::<N>::Bexpr(Box::new(b)))
            }
            Some(_) => {
                let a1 = self.parse_aexpr()?;
                match self.lexer.peek() {
                    Some(Token::Eq) => {
                        self.lexer.match_next(Token::Eq)?;
                        let a2 = self.parse_aexpr()?;
                        Ok(BexprAtomic::<N>::Equal(Box::new(a1), Box::new(a2)))
                    }
                    Some(Token::Neq) => {
                        self.lexer.match_next(Token::Neq)?;
                        let a2 = self.parse_aexpr()?;
                        Ok(BexprAtomic::<N>::NotEqual(Box::new(a1), Box::new(a2)))
                    }
                    Some (Token::Lte) => {
                        self.lexer.match_next(Token::Lte)?;
                        let a2 = self.parse_aexpr()?;
                        Ok(BexprAtomic::<N>::LessEq(Box::new(a1), Box::new(a2)))
                    }
                    Some (Token::Lt) => {
                        self.lexer.match_next(Token::Lt)?;
                        let a2 = self.parse_aexpr()?;
                        Ok(BexprAtomic::<N>::Less(Box::new(a1), Box::new(a2)))
                    }
                    Some (Token::Gte) => {
                        self.lexer.match_next(Token::Gte)?;
                        let a2 = self.parse_aexpr()?;
                        Ok(BexprAtomic::<N>::GreaterEq(Box::new(a1), Box::new(a2)))
                    }
                    Some (Token::Gt) => {
                        self.lexer.match_next(Token::Gt)?;
                        let a2 = self.parse_aexpr()?;
                        Ok(BexprAtomic::<N>::Greater(Box::new(a1), Box::new(a2)))
                    }
                    _ => Err(self.lexer.unexpected_error())
                }
            },
            _ => Err(self.lexer.unexpected_error())
        }  
    }
    





}
