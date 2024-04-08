>For building all the pdf file containing proofs and definitions, run `./build-pdf.sh`.
# Interpreter for While<sup>+</sup> language 

A rust implementation of an interpreter for while<sup>+</sup> language for the Software Verification course at UniPD

The while<sup>+</sup> language add some syntactic sugar over while, namely:
- For loop
- Repeat until loop
- Op-assignment statements: `+=`, `-=`, `*=`
- Boolean ops: `<`, `>`, `>=`, `!=`
- Opposite arithmetic operation: `-`
## How to use

To use the interpreter the command is simply: `cargo run run <filename>`.

And using `--state` is possible to add initial states to the interpreter, for example:
```
cargo run examples/gcd --state "a:222;b:3553"
```

All the option can be seen using: `cargo run run --hep`.

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
                  | Var "*=" Aexpr

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

# Static Analyzer for While language 
Using abstract interpretation to analyze program written in While language

## Addition to the while language 
The analyzer support a couple of operation more than the original while language like:
- `--` pre-dec and post-dec expression
- `++` pre-inc and post-inc expression
- `/` integer division
The semantic for increment and decrement can be seen [here](inc-dec-semantic.pdf)

## Not supported thing
Since the syntactics sugars were implemented before the inc and dec they was not supposed to work with them, so combining the two can cause problem.

For example 
``` 
while(x++ < 0){ 
    // ...
}
```
would be desugar into:
```
while((x++ <= 0) && !(x++ == 0)){ 
    // ...
}
```
and that is clearly wrong.

For this reason is recommended to never use increment or decrement in combination of syntactic sugars.

### How to use
You can simply run the analysis on a program with
`cargo run analyze <filename>`

All the other settings are explained in `cargo run analyze --help`, like the abstract domain,
its configuration is needed, wether to use widening/narrowing, initial states, .... 