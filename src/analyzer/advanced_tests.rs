use crate::types::ast::{Aexpr, Bexpr};

use super::{analyzers::generic_analyzer::GenericAnalyzer, tests, types::{analyzer::StaticAnalyzer, domain::AbstractDomain, state::AbstractState}};

pub fn eval_bexpr<D: AbstractDomain, B: AbstractState<D>>(b: &Bexpr<D>, state: B) -> B{
    match b {
        Bexpr::True => state,
        Bexpr::False => B::bottom(),
        Bexpr::Equal(a1, a2) => {
            match (a1 as &Aexpr<D>, a2 as &Aexpr<D>) {
                (Aexpr::Num(c), ref a@Aexpr::Var(ref x)) | (ref a@Aexpr::Var(ref x), Aexpr::Num(c)) | 
                (Aexpr::Num(c), ref a@Aexpr::PostInc(ref x)) | (ref a@Aexpr::PostInc(ref x), Aexpr::Num(c)) |
                (Aexpr::Num(c), ref a@Aexpr::PostDec(ref x)) | (ref a@Aexpr::PostDec(ref x), Aexpr::Num(c)) |
                (Aexpr::Num(c), ref a@Aexpr::PreInc(ref x)) | (ref a@Aexpr::PreInc(ref x), Aexpr::Num(c)) |
                (Aexpr::Num(c), ref a@Aexpr::PreDec(ref x)) | (ref a@Aexpr::PreDec(ref x), Aexpr::Num(c)) => {
                    let state = eval_pre(a, state);
                    let state = tests::test_eq_case_1(state, x.clone(), c.clone());
                    let state = eval_post(a, state);
                    return state;
                }
                (Aexpr::Var(x), Aexpr::Var(y)) => 
                    tests::test_eq_case_2(state, x.clone(), y.clone(), D::from(0)),
                (a1,a2) => {
                    let (n1, s1) = GenericAnalyzer::eval_aexpr(&a1, state);
                    let (n2, s2) = GenericAnalyzer::eval_aexpr(&a2, s1);
                    let dom = n1.glb(&n2);
                    if dom == D::bottom() {
                        AbstractState::bottom()
                    }else{
                        s2   
                    }
                }
            }
        },
        Bexpr::LessEq(a1, a2) => {
            match (a1 as &Aexpr<D>, a2 as &Aexpr<D>) {
                (ref a@Aexpr::Var(ref x), Aexpr::Num(c)) | 
                (ref a@Aexpr::PostInc(ref x), Aexpr::Num(c)) |(ref a@Aexpr::PostDec(ref x), Aexpr::Num(c)) |
                (ref a@Aexpr::PreInc(ref x), Aexpr::Num(c)) | (ref a@Aexpr::PreDec(ref x), Aexpr::Num(c)) => {
                    let state = eval_pre(a, state);
                    let state = tests::test_lte_case_1(state, x.clone(), c.clone());
                    let state = eval_post(a, state) ;
                    return state;
                }

                (Aexpr::Num(c), ref a@Aexpr::Var(ref x)) |
                (Aexpr::Num(c), ref a@Aexpr::PostDec(ref x)) | (Aexpr::Num(c), ref a@Aexpr::PostInc(ref x)) |
                (Aexpr::Num(c), ref a@Aexpr::PreDec(ref x)) | (Aexpr::Num(c), ref a@Aexpr::PreInc(ref x)) =>{
                    let state = eval_pre(a, state);
                    let state = tests::test_gte_case_1(state, x.clone(), c.clone());
                    let state = eval_post(a, state) ;
                    return state;
                },

                (Aexpr::Var(x), Aexpr::Var(y)) =>
                    tests::test_lte_case_2(state, x.clone(), y.clone()),
                (_, _) => {
                    let (_, state) = GenericAnalyzer::eval_aexpr(&a1, state);
                    let (_, state) = GenericAnalyzer::eval_aexpr(&a2, state);
                    state // Always sound
                }
            }    
        },
        Bexpr::And(b1, b2) => {
            let s1 = eval_bexpr(&b1, state);
            let s2 = eval_bexpr(&b2, s1.clone());
            s1.glb(&s2)
        },
        Bexpr::Not(b) => {
            match b as &Bexpr<D> { 
                Bexpr::True => AbstractState::bottom(),
                Bexpr::False => state,
                Bexpr::Equal(a1, a2) => {
                    match (a1 as &Aexpr<D>,a2 as &Aexpr<D>) {
                        (Aexpr::Num(c), Aexpr::Var(x)) | (Aexpr::Var(x), Aexpr::Num(c)) => 
                            tests::test_neq_case_1(state, x.clone(), c.clone()),
                        (Aexpr::Num(c), ref a@Aexpr::PostInc(ref x)) | (ref a@Aexpr::PostInc(ref x), Aexpr::Num(c)) |
                        (Aexpr::Num(c), ref a@Aexpr::PostDec(ref x)) | (ref a@Aexpr::PostDec(ref x), Aexpr::Num(c)) |
                        (Aexpr::Num(c), ref a@Aexpr::PreInc(ref x)) | (ref a@Aexpr::PreInc(ref x), Aexpr::Num(c)) |
                        (Aexpr::Num(c), ref a@Aexpr::PreDec(ref x)) | (ref a@Aexpr::PreDec(ref x), Aexpr::Num(c))=>{
                            let state = eval_pre(a, state);
                            let state = tests::test_neq_case_1(state, x.clone(), c.clone());
                            let state = eval_post(a, state);
                            return state;
                        },
                        // (Aexpr::Var(x), Aexpr::Var(y)) => 
                        //     tests::test_neq_case_2(s, x, y),
                        (_, _) => {
                            // Since n1 and n2 are over approximations we can't know 
                            let (_, state) = GenericAnalyzer::eval_aexpr(&a1, state);
                            let (_, state) = GenericAnalyzer::eval_aexpr(&a2, state);
                            state // Always sound
                        }
                    } 
                },
                Bexpr::LessEq(a1, a2) => {
                    match (a1 as &Aexpr<D>,a2 as &Aexpr<D>) {
                        (ref a@Aexpr::Var(ref x), Aexpr::Num(c)) |
                        (ref a@Aexpr::PostInc(ref x), Aexpr::Num(c)) |(ref a@Aexpr::PostDec(ref x), Aexpr::Num(c)) |
                        (ref a@Aexpr::PreInc(ref x), Aexpr::Num(c)) | (ref a@Aexpr::PreDec(ref x), Aexpr::Num(c)) => {
                            let state = eval_pre(a, state);
                            let state = tests::test_gt_case_1(state, x.clone(), c.clone());
                            let state = eval_post(a, state) ;
                            return state;
                        }
                        (Aexpr::Num(c), ref a@Aexpr::Var(ref x)) |
                        (Aexpr::Num(c), ref a@Aexpr::PostDec(ref x)) | (Aexpr::Num(c), ref a@Aexpr::PostInc(ref x)) |
                        (Aexpr::Num(c), ref a@Aexpr::PreDec(ref x)) | (Aexpr::Num(c), ref a@Aexpr::PreInc(ref x))=>{
                            let state = eval_pre(a, state);
                            let state = tests::test_lt_case_1(state, x.clone(), c.clone());
                            let state = eval_post(a, state) ;
                            return state;
                        },
                        (Aexpr::Var(x), Aexpr::Var(y)) => 
                            tests::test_gt_case_2(state, x.clone(), y.clone()),
                        (_, _) => {
                            let (_, state) = GenericAnalyzer::eval_aexpr(&a1, state);
                            let (_, state) = GenericAnalyzer::eval_aexpr(&a2, state);
                            return state; //Always sound 
                        }
                    } 
                },
                Bexpr::Not(b) =>  eval_bexpr(&b, state), 
                Bexpr::And(b1, b2) => {
                    let s1 = eval_bexpr(&b1, state.clone());
                    let state = eval_aexpr_in_bexpr(b, state);
                    let s2 = eval_bexpr(&b2, state);
                    s1.lub(&s2)
                },
            }
        }
    }
}

fn eval_aexpr_in_bexpr<D: AbstractDomain, B: AbstractState<D>>(b: &Bexpr<D>, state: B) -> B{
    match b {
        Bexpr::Equal(a1, a2) => {
            let (_, state) = GenericAnalyzer::<D,B>::eval_aexpr(a1, state);
            let (_, state) = GenericAnalyzer::<D,B>::eval_aexpr(a2, state);
            state
        },
        Bexpr::LessEq(a1, a2) => {
            let (_, state) = GenericAnalyzer::eval_aexpr(a1, state);
            let (_, state) = GenericAnalyzer::eval_aexpr(a2, state);
            state
        },
        Bexpr::And(a1, a2) => {            
            let state = eval_aexpr_in_bexpr(a1, state);
            let state = eval_aexpr_in_bexpr(a2, state);
            state
        },
        Bexpr::Not(b) => eval_aexpr_in_bexpr(b, state),
        Bexpr::True | Bexpr::False => state,
    }
}

fn eval_pre<D: AbstractDomain, B: AbstractState<D>>(a: &Aexpr<D>, state: B) -> B {
    match a{
        Aexpr::PreInc(_) | Aexpr::PreDec(_) => GenericAnalyzer::eval_aexpr(a, state).1,
        Aexpr::BinOp(_, a1, a2) => {
            let state = eval_pre(a1, state);
            let state = eval_pre(a2, state);
            state
        }
        _ => state
    }
}
fn eval_post<D: AbstractDomain, B: AbstractState<D>>(a: &Aexpr<D>, state: B) -> B {
    match a{
        Aexpr::PostInc(_) | Aexpr::PostDec(_) => GenericAnalyzer::eval_aexpr(a, state).1,
        Aexpr::BinOp(_, a1, a2) => {
            let state = eval_post(a1, state);
            let state = eval_post(a2, state);
            state
        }
        _ => state
    }
}