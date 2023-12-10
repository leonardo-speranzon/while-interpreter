Syntactic sugars:
- For loops
- Repeat until loop
- Increase statement
- Decrease statement
- Boolean ops: <,>,>=, !=

# Grammar

Only `Statements` is terminal

```
Statements ::= Statements Statement
             | Statement

Statement ::= "skip;"
            | AssignStatement ";"
            | "if" Bexpr "then" Statement "else" Statement
            | "while" Bexpr "do" Statement
            | "{" Statements "}"
            | "repeat" Statement "until" Bexpr ";"
            | "for(" Var  ":=" Aexpr ";" Bexpr ";" AssignStatement ")" Statement 

AssignStatement ::= Var ":=" Aexpr
                  | Var "+=" Aexpr
                  | Var "-=" Aexpr

Aexpr ::= Aexpr "+" Term 
        | Aexpr "-" Term
        | Term

Term  ::= Term "*" Factor
        | Factor

Factor ::= Num | Var | "(" Aexpr ")" 


Bexpr ::= Bexpr "and" BexprAtomic
        | Bexpr "or" BexprAtomic
        | BexprAtomic
    
BexprAtomic ::= "true" | "false"
              | Aexpr "=" Aexpr
              | Aexpr "!=" Aexpr
              | Aexpr "<" Aexpr
              | Aexpr "<=" Aexpr
              | Aexpr ">" Aexpr
              | Aexpr ">=" Aexpr
              | "not" BexprAtomic
              | "(" Bexpr ")"

Num ::= [0-9]+
Var ::= (a-z | A-Z)[a-z | A-Z | 0-9]*
```