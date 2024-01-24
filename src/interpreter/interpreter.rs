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
            // cond(
            //     b,
            //     &boh(stm1),
            //     &boh(stm2)
            // )(state),
        },
        Statement::While(b, stm) => {
            // let id = &|s: State| Ok(Some(s));
            // let bottom = &|_| Ok(None);
            // let guard = |s: &State| eval_bexpr(b, s);
            // let f_bottom = cond(
            //     &guard,
            //     bottom,
            //     id
            // );
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


// fn boh(statement: &Statement)->impl Fn(State)->State{
//     let stm_clone=statement.clone();
//     move |state:State|eval_statement(&stm_clone, state)
// }
// fn cond<'a, T>(b: &'a dyn Fn(&State)->Result<bool,RuntimeError>, g1: &'a dyn Fn(State)->Result<T,RuntimeError>, g2: &'a dyn Fn(State)->Result<T,RuntimeError>) -> impl Fn(State) -> Result<T,RuntimeError> + 'a  {
//     move |state: State| {
//         if b(&state)? {
//             g1(state)
//         } else {
//             g2(state)
//         }
//     }   
// }



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
        Aexpr::Num(n) => n.to_owned(),
        Aexpr::Var(x) => 
            match state.get(x) {
                Some(n) => n.to_owned(),
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
    };
    Ok(num)
}



// #[allow(dead_code)]
// pub fn config_transition(config: Configuration) -> Configuration {
//     if let Configuration::NonTerminal(stm, mut state) = config {
//         let new_conf = match *stm {
//             Statement::Assign(x, aexp) => {
//                 let num = eval_aexpr(&aexp, &state);
//                 state.insert(x.to_string(), num);
//                 Configuration::Terminal(state)
//             },
//             Statement::Skip => Configuration::Terminal(state),
//             Statement::Compose(s1, s2) => {
//                 match config_transition(Configuration::NonTerminal(s1, state)){
//                     Configuration::Terminal(state) =>
//                         Configuration::NonTerminal(s2, state),
//                     Configuration::NonTerminal(s1, state) =>
//                         Configuration::NonTerminal(Statement::Compose(s1, s2).into(), state),
//                 }
//             },
//             Statement::IfThenElse(b, s1, s2) =>
//                 if eval_bexpr(&b, &state) {
//                     Configuration::NonTerminal(s1, state)
//                 } else {
//                     Configuration::NonTerminal(s2, state)
//                 },
//             Statement::While(b, s) => {
//                 Configuration::NonTerminal(Statement::IfThenElse(
//                     b.clone(),
//                     Statement::Compose(s.clone(), Statement::While(b, s).into()).into(),
//                     Statement::Skip.into()
//                 ).into(), state)
//             }
//         };
//         return new_conf;
//     }else {
//         panic!()
//     }
// }
