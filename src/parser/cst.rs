use crate::ast::Num;

pub type Var = String;

#[derive(Debug, Clone)]
pub enum Aexpr  {
    Add  (Box<Aexpr>,Box<Term>),
    Sub  (Box<Aexpr>, Box<Term>),
    Term (Box<Term>),

    //syntactic sugars
    Opposite (Box<Factor>),
}

#[derive(Debug, Clone)]
pub enum Term {
    Mul   (Box<Term>, Box<Factor>),
    Factor(Box<Factor>)
}

#[derive(Debug, Clone)]
pub enum Factor {
    Num  (Num),
    Var  (Var),
    Aexpr (Box<Aexpr>)
}



#[derive(Debug, Clone)]
pub enum Bexpr {
    And    (Box<Bexpr>, Box<BexprAtomic>),
    Atomic (Box<BexprAtomic>),

    //syntactic sugars
    Or     (Box<Bexpr>, Box<BexprAtomic>),
}
#[derive(Debug, Clone)]
pub enum BexprAtomic {
    True,
    False,
    Equal     (Box<Aexpr>, Box<Aexpr>),
    LessEq    (Box<Aexpr>, Box<Aexpr>),
    Not    (Box<BexprAtomic>),
    Bexpr  (Box<Bexpr>),

    //syntactic sugars
    Less      (Box<Aexpr>, Box<Aexpr>),
    GreaterEq (Box<Aexpr>, Box<Aexpr>),
    Greater   (Box<Aexpr>, Box<Aexpr>),
    NotEqual (Box<Aexpr>, Box<Aexpr>),
}


#[derive(Debug, Clone)]
pub enum Statement {
    Skip,
    IfThenElse (Box<Bexpr>, Box<Statement>, Box<Statement>),
    While      (Box<Bexpr>, Box<Statement>),
    Block      (Box<Statements>),
    AssignStm (Box<AssignStatements>),

    //syntactic sugars
    RepeatUntil(Box<Statement>, Box<Bexpr>),
    ForLoop (Var, Box<Aexpr>, Box<Bexpr>,Box<AssignStatements>,Box<Statement>),
}

#[derive(Debug, Clone)]
pub enum AssignStatements {
    Assign (Var, Box<Aexpr>),
    AddAssign (Var, Box<Aexpr>),
    SubAssign (Var, Box<Aexpr>),
    MulAssign (Var, Box<Aexpr>),
}

#[derive(Debug, Clone)]
pub enum Statements {
    Composition (Box<Statements>, Box<Statement>),
    Singleton (Box<Statement>)
}