use std::collections::BTreeSet;

use crate::types::ast::{Aexpr, Bexpr, Operator, Var};

use super::{analyzers::generic_analyzer::GenericAnalyzer, types::{analyzer::StaticAnalyzer, domain::{AbstractDomain, Interval}, state::AbstractState}};


pub fn eval_bexpr<D: AbstractDomain, B: AbstractState<D>>(b: &Bexpr<D>, state: B) -> B {
    if include_critical_ops(&b) {
        return eval_bexpr_dumb(b, state)
    }
    let mut state = eval_pre_b(b, state);

    let mut new_state = eval_bexpr_h(b, state.clone(), false);
    // let  mut i = 1;
    while &new_state != &state {
        state = state.glb(&new_state);
        new_state = eval_bexpr_h(b, state.clone(), false);
        // i+=1;
    }
    // println!("Computed test in {} iterations", i);

    let state = eval_post_b(b, state);
    state
}

// In the case where advance abstract test cannot be used it fallback to this
fn eval_bexpr_dumb<D: AbstractDomain, B: AbstractState<D>> (b: &Bexpr<D>, state: B) -> B {
    match b {
        Bexpr::True => state,
        Bexpr::False => B::bottom(),
        Bexpr::Equal(a1, a2) => {
            let (d1, state) = GenericAnalyzer::eval_aexpr(a1, state);
            let (d2, state) = GenericAnalyzer::eval_aexpr(a2, state);
            if d1.glb(d2) == D::bottom() {
                B::bottom()
            } else {
                state
            }
        },
        Bexpr::LessEq(_, _) => state,
        Bexpr::Not(_) => state,
        Bexpr::And(b1, b2) => {
            let state = eval_bexpr_dumb(b1, state);
            let state = eval_bexpr_dumb(b2, state);
            state // By construction will be either the original state (+ inc/dec) or Bottom
        },
    }
}


fn eval_bexpr_h<D: AbstractDomain, B: AbstractState<D>>(b: &Bexpr<D>, state: B, negated: bool) -> B{
    match b {
        Bexpr::True => state,
        Bexpr::False => B::bottom(),
        Bexpr::Equal(a1, a2) => {
            let interval = if !negated {
                D::from(0) // == 0
            } else {
                D::from(Interval::OpenLeft(-1)).lub(D::from(Interval::OpenRight(1))) // != 0
            };
            advanced_abstract_tests(a1, a2, state, interval)
        },
        Bexpr::LessEq(a1, a2) => {
            let interval = if !negated {
                D::from(Interval::OpenLeft(0)) // <= 0
            } else {
                D::from(Interval::OpenRight(1)) // > 0
            };
            advanced_abstract_tests(a1, a2, state, interval)
        },
        Bexpr::And(b1, b2) => {
            let state1 = eval_bexpr_h(b1, state.clone(), negated);
            let state2 = eval_bexpr_h(b2, state, negated);
            if !negated {
                state1.glb(&state2) // AND
            } else {
                state1.lub(&state2) // OR
            }
        },
        Bexpr::Not(b) => eval_bexpr_h(b, state, !negated)
    }
}

fn advanced_abstract_tests<D: AbstractDomain, B: AbstractState<D>>(a1: &Aexpr<D>, a2: &Aexpr<D>, state: B, interval: D) -> B {
    let a: Aexpr<D> = match a2 as &Aexpr<D> {
        Aexpr::Lit(n) if n == &D::from(0) => (a1 as &Aexpr<D>).clone(),
        _ => Aexpr::BinOp(Operator::Sub, Box::new(a1.clone()), Box::new(a2.clone()))
    };
    let eval_tree = eval_aexpr_tree(&a, &state);
    let state = refine(&eval_tree, state, interval);
    state
    
}


enum EvalTree<D: AbstractDomain>{
    LeafNum(D),
    LeafVar(String,D),
    BinOp(Operator, D, Box<EvalTree<D>>, Box<EvalTree<D>>)
}
impl<D:AbstractDomain> EvalTree<D> {
    fn get_interval(&self)-> D{
        match self{
            EvalTree::LeafNum(d) => *d,
            EvalTree::LeafVar(_, d) => *d,
            EvalTree::BinOp(_, d, _, _) => *d,
        }
    }
}
fn eval_aexpr_tree<D: AbstractDomain, B: AbstractState<D>>(a: &Aexpr<D>, state: &B) -> EvalTree<D> {
    match a {
        Aexpr::Lit(n) => EvalTree::LeafNum(*n),
        Aexpr::Var(x) => EvalTree::LeafVar(x.clone(), state.get(x)),
        Aexpr::PreOp(_, x) | Aexpr::PostOp(_, x) => EvalTree::LeafVar(x.clone(), state.get(x)),
        Aexpr::BinOp(op, a1, a2 ) => {
            let t1 = eval_aexpr_tree(a1, state);
            let t2 = eval_aexpr_tree(a2, state);
            EvalTree::BinOp(
                op.clone(), 
                D::abstract_operator(op, t1.get_interval(), t2.get_interval()),
                Box::new(t1),
                Box::new(t2)
            )        
        }
    }    
}
fn refine<D: AbstractDomain, B: AbstractState<D>>(tree: &EvalTree<D>, mut state: B, interval: D) -> B {
    match tree {
        EvalTree::LeafNum(_) => state,
        EvalTree::LeafVar(x, _) => {
            state.set(x.clone(), state.get(x).glb(interval));
            state
        }
        EvalTree::BinOp(op, _, lhs, rhs) => {
            let (l_int,r_int) = D::backward_abstract_operator(
                op,
                lhs.get_interval(), 
                rhs.get_interval(), 
                interval
            );
            let state = refine(lhs,state,l_int); 
            let state = refine(rhs,state,r_int);
            state
        },
    }
}


fn eval_pre_b<D: AbstractDomain, B: AbstractState<D>>(b: &Bexpr<D>, state: B) -> B {
    match b{
        Bexpr::True | Bexpr::False => state,
        Bexpr::Equal(a1, a2) | Bexpr::LessEq(a1, a2) => {
            let state = eval_pre_a(a1, state);
            let state = eval_pre_a(a2, state);
            state
        },
        Bexpr::Not(b) => eval_pre_b(b, state),
        Bexpr::And(b1, b2) => {
            let state = eval_pre_b(b1, state);
            let state = eval_pre_b(b2, state);
            state
        },
    }
}
fn eval_pre_a<D: AbstractDomain, B: AbstractState<D>>(a: &Aexpr<D>, state: B) -> B {
    match a{
        Aexpr::PreOp(_, _) => GenericAnalyzer::eval_aexpr(a, state).1,
        Aexpr::BinOp(_, a1, a2) => {
            let state = eval_pre_a(a1, state);
            let state = eval_pre_a(a2, state);
            state
        }
        _ => state
    }
}


fn eval_post_b<D: AbstractDomain, B: AbstractState<D>>(b: &Bexpr<D>, state: B) -> B {
    match b{
        Bexpr::True | Bexpr::False => state,
        Bexpr::Equal(a1, a2) | Bexpr::LessEq(a1, a2) => {
            let state = eval_post_a(a1, state);
            let state = eval_post_a(a2, state);
            state
        },
        Bexpr::Not(b) => eval_post_b(b, state),
        Bexpr::And(b1, b2) => {
            let state = eval_post_b(b1, state);
            let state = eval_post_b(b2, state);
            state
        },
    }
}
fn eval_post_a<D: AbstractDomain, B: AbstractState<D>>(a: &Aexpr<D>, state: B) -> B {
    match a{
        Aexpr::PostOp(_, _) => GenericAnalyzer::eval_aexpr(a, state).1,
        Aexpr::BinOp(_, a1, a2) => {
            let state = eval_post_a(a1, state);
            let state = eval_post_a(a2, state);
            state
        }
        _ => state
    }
}




/**
 * Return true if the arithmetic expression contain some operation that
 * would preclude the advanced abstract test validity (and soundness)
 * CRITICAL OPERATIONS:
 * - Single appearance of vars which have an inc/dec
 * 
 */
fn include_critical_ops<D: AbstractDomain>(b: &Bexpr<D>) -> bool {
    if let Err(_) = check_no_dup_b(b){
        return true
    }

    return false
}

/**
 * Check wether "Single appearance of vars which have an inc/dec" is true
 * 
 * Return type: (v, ops) where:
 *          v = variable that appears
 *          ops = variable affected by post inc dec
 * 
 */
fn check_no_dup_b <D: AbstractDomain>(b: &Bexpr<D>) -> Result<(BTreeSet<Var>, BTreeSet<Var>),Var> {
    match b {
        Bexpr::True => Ok((BTreeSet::new(), BTreeSet::new())),
        Bexpr::False => Ok((BTreeSet::new(), BTreeSet::new())),
        Bexpr::Equal(a1, a2) | Bexpr::LessEq(a1, a2) => {
            let r1 = check_no_dup_a(a1)?;
            let r2 = check_no_dup_a(a2)?;            
            merge(r1,r2)      
        },
        Bexpr::Not(b) => check_no_dup_b(b),
        Bexpr::And(b1, b2) => {
            let r1 = check_no_dup_b(b1)?;
            let r2 = check_no_dup_b(b2)?;            
            merge(r1,r2)      
        },
    }
}

fn check_no_dup_a <D: AbstractDomain>(a: &Aexpr<D>) -> Result<(BTreeSet<Var>, BTreeSet<Var>),Var> {
    match a {
        Aexpr::Lit(_) => Ok((BTreeSet::new(), BTreeSet::new())),
        Aexpr::Var(x) => Ok((BTreeSet::from([x.clone()]), BTreeSet::new())),
        Aexpr::PreOp(_, x) | Aexpr::PostOp(_, x) => Ok((BTreeSet::new(), BTreeSet::from([x.clone()]))),
        Aexpr::BinOp(_, a1, a2) => {
            let r1 = check_no_dup_a(a1)?;
            let r2 = check_no_dup_a(a2)?;            
            merge(r1,r2)            
        },
    }
}

fn merge(
        (mut v1, mut op1): (BTreeSet<Var>, BTreeSet<Var>),
        (mut v2, mut op2): (BTreeSet<Var>, BTreeSet<Var>)
    ) -> Result<(BTreeSet<Var>, BTreeSet<Var>),Var> {

    // Two inc/dec on the same var
    if let Some(dup) = op1.intersection(&op2).next() {
        return Err(dup.clone())
    }
    v1.append(&mut v2);
    op1.append(&mut op2);

    // A inc/dec var appear another time
    if let Some(dup) = v1.intersection(&op1).next() {
        return Err(dup.clone())
    }

    Ok((v1,op1))
}
