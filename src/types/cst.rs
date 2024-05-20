pub type Var = String;

#[derive(Debug, Clone)]
pub enum Aexpr<N>  {
    Add  (Box<Aexpr<N>>,Box<Term<N>>),
    Sub  (Box<Aexpr<N>>, Box<Term<N>>),
    Term (Box<Term<N>>),

    //syntactic sugars
    Opposite (Box<Factor<N>>),
}

#[derive(Debug, Clone)]
pub enum Term<N> {
    Mul   (Box<Term<N>>, Box<Factor<N>>),
    Div   (Box<Term<N>>, Box<Factor<N>>),
    Factor(Box<Factor<N>>)
}

#[derive(Debug, Clone)]
pub enum Factor<N> {
    Lit  (N),
    Var  (Var),
    PreInc (Var),
    PostInc (Var),
    PreDec (Var),
    PostDec (Var),
    Aexpr (Box<Aexpr<N>>)
}



#[derive(Debug, Clone)]
pub enum Bexpr<N> {
    And    (Box<Bexpr<N>>, Box<BexprAtomic<N>>),
    Atomic (Box<BexprAtomic<N>>),

    //syntactic sugars
    Or     (Box<Bexpr<N>>, Box<BexprAtomic<N>>),
}
#[derive(Debug, Clone)]
pub enum BexprAtomic<N> {
    True,
    False,
    Equal     (Box<Aexpr<N>>, Box<Aexpr<N>>),
    LessEq    (Box<Aexpr<N>>, Box<Aexpr<N>>),
    Not    (Box<BexprAtomic<N>>),
    Bexpr  (Box<Bexpr<N>>),

    //syntactic sugars
    Less      (Box<Aexpr<N>>, Box<Aexpr<N>>),
    GreaterEq (Box<Aexpr<N>>, Box<Aexpr<N>>),
    Greater   (Box<Aexpr<N>>, Box<Aexpr<N>>),
    NotEqual (Box<Aexpr<N>>, Box<Aexpr<N>>),
}


#[derive(Debug, Clone)]
pub enum Statement<N> {
    Skip,
    IfThenElse (Box<Bexpr<N>>, Box<Statement<N>>, Box<Statement<N>>),
    While      (Box<Bexpr<N>>, Box<Statement<N>>),
    Block      (Box<Statements<N>>),
    AssignStm (Box<AssignStatements<N>>),

    //syntactic sugars
    RepeatUntil(Box<Statement<N>>, Box<Bexpr<N>>),
    ForLoop (Var, Box<Aexpr<N>>, Box<Bexpr<N>>,Box<AssignStatements<N>>,Box<Statement<N>>),
}

#[derive(Debug, Clone)]
pub enum AssignStatements<N> {
    Assign (Var, Box<Aexpr<N>>),
    AddAssign (Var, Box<Aexpr<N>>),
    SubAssign (Var, Box<Aexpr<N>>),
    MulAssign (Var, Box<Aexpr<N>>),
}

#[derive(Debug, Clone)]
pub enum Statements<N> {
    Composition (Box<Statements<N>>, Box<Statement<N>>),
    Singleton (Box<Statement<N>>)
}