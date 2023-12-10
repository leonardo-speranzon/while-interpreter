pub type Num = i128;
pub type Var = String;

#[derive(Debug, Clone)]
pub enum Aexpr  {
    Num  (Num),
    Var  (Var),
    Add  (Box<Aexpr>, Box<Aexpr>),
    Mul (Box<Aexpr>, Box<Aexpr>),
    Sub  (Box<Aexpr>, Box<Aexpr>),
}
#[derive(Debug, Clone)]
pub enum Bexpr {
    True,
    False,
    Equal  (Box<Aexpr>, Box<Aexpr>),
    LessEq (Box<Aexpr>, Box<Aexpr>),
    Not    (Box<Bexpr>),
    And    (Box<Bexpr>, Box<Bexpr>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assign (Var, Box<Aexpr>),
    Skip,
    Compose    (Box<Statement>, Box<Statement>),
    IfThenElse (Box<Bexpr>, Box<Statement>, Box<Statement>),
    While      (Box<Bexpr>, Box<Statement>),
    // Abort,
    // Or         (Box<Statement>, Box<Statement>),
    // Par        (Box<Statement>, Box<Statement>),
}


