pub type Num = i128;
pub type Var = String;



#[derive(Debug, Clone)]
pub enum Operator{
    Add,Sub,Mul//,Div
}

#[derive(Debug, Clone)]
pub enum Statement<D> {
    Assign (Var, Box<Aexpr<D>>),
    Skip,
    Compose    (Box<Statement<D>>, Box<Statement<D>>),
    IfThenElse (Box<Bexpr<D>>, Box<Statement<D>>, Box<Statement<D>>),
    While      (Box<Bexpr<D>>, Box<Statement<D>>),
}

#[derive(Debug, Clone)]
pub enum Aexpr<D>  {
    Num  (D),
    Var  (Var),
    BinOp (Operator, Box<Aexpr<D>>, Box<Aexpr<D>>),
}

#[derive(Debug, Clone)]
pub enum Bexpr<D> {
    True,
    False,
    Equal  (Box<Aexpr<D>>, Box<Aexpr<D>>),
    LessEq (Box<Aexpr<D>>, Box<Aexpr<D>>),
    Not    (Box<Bexpr<D>>),
    And    (Box<Bexpr<D>>, Box<Bexpr<D>>),
}


