use crate::types::{ast::{Statement, Aexpr, Bexpr, Num, Operator}, errors::RuntimeError};

use super::types::State;

pub fn eval_statement(statement: &Statement<Num>, mut state: State<Num>) -> Result<State<Num>, RuntimeError> {
    match statement {
        Statement::Assign(x, aexpr) => {
            state.insert(x.to_string(), eval_aexpr(aexpr, &state)?);
            Ok(state)
        },
        Statement::Skip => Ok(state),
        Statement::Compose(stm1, stm2) =>{
            state = eval_statement(&stm1, state)?;
            state = eval_statement(&stm2, state)?;
            Ok(state)
        },
        Statement::IfThenElse(b, stm1, stm2) => {
            if eval_bexpr(b, &state)? {
                eval_statement(&stm1, state)
            } else {
                eval_statement(&stm2, state)
            }
        },
        Statement::While(b, stm) => {
            let f_bottom = |s: State<Num>|{
                if eval_bexpr(b, &s)? {
                    Ok(None)
                } else {
                    Ok(Some(s))
                }
            };
            let mut last_state: State<Num> = state.clone();
            
            // let mut i=0;
            
            // check if (F^k ⊥) s = undef = (F⊥ o (stm^k-1)) s
            while f_bottom(last_state.clone())?.is_none() {
                // println!("ITER: {i}, state: {:?}",last_state); i+=1;
                
                // S[stm]^(k+1) s = S[stm](S[stm]^k s)
                last_state = eval_statement(stm, last_state)?;
            }
            f_bottom(last_state).map(|s|s.unwrap())
        },
    
    }
}


fn eval_bexpr(bexpr: &Bexpr<Num>, state: &State<Num>) -> Result<bool,RuntimeError> {
    let b = match bexpr {
        Bexpr::True => true,
        Bexpr::False => false,
        Bexpr::Equal(a1, a2) => 
            eval_aexpr(a1, state)? == eval_aexpr(a2, state)?,
        Bexpr::LessEq(a1, a2) => 
            eval_aexpr(a1, state)? <= eval_aexpr(a2, state)?,
        Bexpr::Not(b) => 
            !eval_bexpr(b, state)?,
        Bexpr::And(b1, b2) => 
            eval_bexpr(b1, state)? && eval_bexpr(b2, state)?,
    };
    Ok(b)
}

fn eval_aexpr(aexpr: &Aexpr<Num>, state: &State<Num>) -> Result<Num,RuntimeError> {
    let num = match aexpr {
        Aexpr::Lit(n) => *n,
        Aexpr::Var(x) => 
            match state.get(x) {
                Some(n) => *n,
                None => return Err(RuntimeError::VariableNotInitialized(x.clone())),
            }
        Aexpr::BinOp(op, a1, a2) =>{
            let n1 = eval_aexpr(a1, state)?;
            let n2 = eval_aexpr(a2, state)?;            
            match op {
                Operator::Add => n1 + n2,
                Operator::Sub => n1 - n2,
                Operator::Mul => n1 * n2,
                Operator::Div => return Err(RuntimeError::NotImplemented("Division not implemented".to_string())),
            }
        }
        Aexpr::PreOp(_,_) => return Err(RuntimeError::NotImplemented("Prefix operators not implemented".to_string())),
        Aexpr::PostOp(_,_) => return Err(RuntimeError::NotImplemented("Postfix operators not implemented".to_string())),
    };
    Ok(num)
}
