use crate::types::{cst::{Statement, Statements,Aexpr, Term, Factor, BexprAtomic, Bexpr, AssignStatements}, errors::ParserError, tokens::Token};
use super::lexer::Lexer;



pub struct ConcreteParser<L: Lexer>{
    lexer: L,
}

impl<L: Lexer> ConcreteParser<L> {
    pub fn new(lexer: L) -> Self { Self { lexer } }


    pub fn parse(mut self)->Result<Statements,ParserError>{
        let ast = self.parse_statements()?;
        if let Some(_) = self.lexer.peek(){
            Err(self.lexer.unexpected_error())
        }else{
            Ok(ast)
        }
    }
    
    
    fn parse_statements(&mut self) -> Result<Statements,ParserError> {
        let s = self.parse_statement()?;
        let mut stms = Statements::Singleton(Box::new(s));

        loop {
            match self.lexer.peek() {
                None => break,
                Some(Token::CurlyClose) => break,
                Some(_) => {
                    let s = self.parse_statement()?;
                    stms = Statements::Composition(Box::new(stms), Box::new(s));
                }
            }
        }
        Ok(stms)
    }



    fn parse_statement(&mut self) -> Result<Statement,ParserError> {
        match self.lexer.peek() {
            Some(Token::Skip) => {
                self.lexer.match_next(Token::Skip)?;
                self.lexer.match_next(Token::Semicolon)?;
                Ok(Statement::Skip)
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
    fn parse_assign_statement(&mut self) -> Result<AssignStatements,ParserError> {
        let x = self.parse_id()?;
        let stm_constructor = match self.lexer.peek() {
            Some(Token::Assign) => {
                self.lexer.match_next(Token::Assign)?;
                AssignStatements::Assign
            },
            Some(Token::AddAssign) => {
                self.lexer.match_next(Token::AddAssign)?;
                AssignStatements::AddAssign
            },
            Some(Token::SubAssign) => {
                self.lexer.match_next(Token::SubAssign)?;
                AssignStatements::SubAssign
            },
            Some(Token::MulAssign) => {
                self.lexer.match_next(Token::MulAssign)?;
                AssignStatements::MulAssign
            },
            _ => return Err(self.lexer.unexpected_error()),
        };
        let a = self.parse_aexpr()?;
        Ok(stm_constructor(x, Box::new(a)))
    }

    fn parse_id(&mut self) -> Result<String, ParserError>{
        let x =  match self.lexer.peek(){
            Some(Token::Id(x)) => x,
            _ => return Err(self.lexer.unexpected_error())
        };
        self.lexer.match_next(Token::Id(x.clone()))?;
        return Ok(x)
    }
    
    fn parse_aexpr(&mut self) -> Result<Aexpr,ParserError> {
        if let Some(Token::Minus) = self.lexer.peek() {
            self.lexer.match_next(Token::Minus)?;    
            return Ok(Aexpr::Opposite(Box::new(
                self.parse_factor()?
            )));   
        }

        let t = self.parse_term()?;
        let mut aexpr = Aexpr::Term(Box::new(t));
        loop {
            match self.lexer.peek() {
                Some(Token::Plus) => {
                    self.lexer.match_next(Token::Plus)?;    
                    let t = self.parse_term()?;
                    aexpr = Aexpr::Add(Box::new(aexpr), Box::new(t));
                },
                Some(Token::Minus) => {
                    self.lexer.match_next(Token::Minus)?;    
                    let t = self.parse_term()?;
                    aexpr = Aexpr::Sub(Box::new(aexpr), Box::new(t));
                }
                _ => break,
            }
        }
        return Ok(aexpr);
    }
    fn parse_term(&mut self) -> Result<Term,ParserError> {
        let f = self.parse_factor()?;
        let mut term = Term::Factor(Box::new(f));
        loop {
            match self.lexer.peek() {
                Some(Token::Mul) => {
                    self.lexer.match_next(Token::Mul)?;     
                    let f = self.parse_factor()?;
                    term = Term::Mul(Box::new(term), Box::new(f));
                }
                _ => break,
            }
        }
        return Ok(term);
    }
    fn parse_factor(&mut self) -> Result<Factor,ParserError> {
        match self.lexer.peek() {
            Some(Token::Num(n)) => {
                self.lexer.match_next(Token::Num(n))?;
                Ok(Factor::Num(n))
            },
            Some(Token::Id(x)) => {
                self.lexer.match_next(Token::Id(x.clone()))?;
                Ok(Factor::Var(x))
            },
            Some(Token::BracketOpen) => {
                self.lexer.match_next(Token::BracketOpen)?;
                let a = self.parse_aexpr()?;
                self.lexer.match_next(Token::BracketClose)?;
                Ok(Factor::Aexpr(Box::new(a)))
            }
            _ => Err(self.lexer.unexpected_error())
        }    
    }
    
    
    
    fn parse_bexpr(&mut self) -> Result<Bexpr,ParserError> {
        let ba = self.parse_bexpr_atom()?;
        let mut bexpr = Bexpr::Atomic(Box::new(ba));
        loop {
            match self.lexer.peek() {
                Some(Token::And) => {
                    self.lexer.match_next(Token::And)?;     
                    let ba = self.parse_bexpr_atom()?;
                    bexpr = Bexpr::And(Box::new(bexpr), Box::new(ba));
                }
                Some(Token::Or) => {
                    self.lexer.match_next(Token::Or)?;     
                    let ba: BexprAtomic = self.parse_bexpr_atom()?;
                    bexpr = Bexpr::Or(Box::new(bexpr), Box::new(ba));
                }
                _ => break,
            }
        }
        return Ok(bexpr);
    }
    fn parse_bexpr_atom(&mut self) -> Result<BexprAtomic,ParserError> {
        match self.lexer.peek() {
            Some(Token::True) => {
                self.lexer.match_next(Token::True)?;
                Ok(BexprAtomic::True)
            },
            Some(Token::False) => {
                self.lexer.match_next(Token::False)?;
                Ok(BexprAtomic::False)
            },
            Some(Token::Not) => {
                self.lexer.match_next(Token::Not)?;
                let b =self.parse_bexpr_atom()?;
                Ok(BexprAtomic::Not(Box::new(b)))
            },
            Some(Token::BracketOpen) => {
                self.lexer.match_next(Token::BracketOpen)?;
                let b = self.parse_bexpr()?;
                self.lexer.match_next(Token::BracketClose)?;
                Ok(BexprAtomic::Bexpr(Box::new(b)))
            }
            Some(_) => {
                let a1 = self.parse_aexpr()?;
                match self.lexer.peek() {
                    Some(Token::Eq) => {
                        self.lexer.match_next(Token::Eq)?;
                        let a2 = self.parse_aexpr()?;
                        Ok(BexprAtomic::Equal(Box::new(a1), Box::new(a2)))
                    }
                    Some(Token::Neq) => {
                        self.lexer.match_next(Token::Neq)?;
                        let a2 = self.parse_aexpr()?;
                        Ok(BexprAtomic::NotEqual(Box::new(a1), Box::new(a2)))
                    }
                    Some (Token::Lte) => {
                        self.lexer.match_next(Token::Lte)?;
                        let a2 = self.parse_aexpr()?;
                        Ok(BexprAtomic::LessEq(Box::new(a1), Box::new(a2)))
                    }
                    Some (Token::Lt) => {
                        self.lexer.match_next(Token::Lt)?;
                        let a2 = self.parse_aexpr()?;
                        Ok(BexprAtomic::Less(Box::new(a1), Box::new(a2)))
                    }
                    Some (Token::Gte) => {
                        self.lexer.match_next(Token::Gte)?;
                        let a2 = self.parse_aexpr()?;
                        Ok(BexprAtomic::GreaterEq(Box::new(a1), Box::new(a2)))
                    }
                    Some (Token::Gt) => {
                        self.lexer.match_next(Token::Gt)?;
                        let a2 = self.parse_aexpr()?;
                        Ok(BexprAtomic::Greater(Box::new(a1), Box::new(a2)))
                    }
                    _ => Err(self.lexer.unexpected_error())
                }
            },
            _ => Err(self.lexer.unexpected_error())
        }  
    }
    





}
