# Interpreter for While<sup>+</sup> language 

A rust implementation of an interpreter for while<sup>+</sup> language for the Software Verification course at UniPD

The while<sup>+</sup> language add some syntactic sugar over while, namely:
- For loops
- Repeat until loop
- Increase/Decrease statement
- Boolean ops: `<`,`>`,`>=`, `!=`
## How to use

To use the interpreter the command is simply: `cargo run <filename>`.

And using `--state` is possible to add initial states to the interpreter, for example:
```
cargo run examples/gcd --state a:222,b:3553
```


## Grammar of While<sup>+</sup> 
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
        | "-" Factor
        | Term

Term  ::= Term "*" Factor
        | Factor

Factor ::= Num | Var | "(" Aexpr ")"


Bexpr ::= Bexpr "and" BexprAtomic
        | Bexpr "or" BexprAtomic
        | BexprAtomic
    
BexprAtomic ::= "true" | "false"
              | Aexpr "==" Aexpr
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

## Implementation of while loop
By the assignment the semantics of the while loop (the only one) must rely on Kleene-Knaster-Tarski fixpoint iteration sequence.

But since Rust language is not so functional-like the naive implementation is not the best. So I implemented it in a not naive method that is equivalent to the naive one by [this demonstration](demonstration.md)