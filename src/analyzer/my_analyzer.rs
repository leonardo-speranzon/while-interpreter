use std::{collections::HashMap, marker::PhantomData};

use crate::{types::ast::{Aexpr, Bexpr}, analyzer::printers::map_to_str};

use super::{AbstractDomain, AbstractState, StaticAnalyzer, program::{Label, Command, Program, Arc}};
pub struct MyAnalyzer<D, B> {    
   domain: PhantomData<D>,
   abs_state: PhantomData<B>,
}

impl<D: AbstractDomain, B: AbstractState<D>> StaticAnalyzer<D,B> for MyAnalyzer<D,B>{
    
    fn eval_aexpr(a: &Aexpr<D>, mut s: B)-> (D, B) {
        match a {
            Aexpr::Num(n) => (n.clone(), s),
            Aexpr::Var(x) => (s.get(x), s),
            Aexpr::BinOp(op, a1, a2 ) => {
                let (n1, s1) = Self::eval_aexpr(a1, s);
                let (n2, s2) = Self::eval_aexpr(a2, s1);
                let d = D::abstract_operator(op, &n1, &n2);
                (d, s2)
            }
            Aexpr::PreInc(x) => {
                let d = s.get(x) + D::from(1);
                s.set(x.to_string(), d.clone());
                (d, s)
            }
            Aexpr::PreDec(x) => {
                let d = s.get(x) - D::from(1);
                s.set(x.to_string(), d.clone());
                (d, s)
            },
            Aexpr::PostInc(x) => {
                let d = s.get(x);
                s.set(x.to_string(), d.clone() + D::from(1));
                (d, s)
            },
            Aexpr::PostDec(x) =>{
                let d = s.get(x);
                s.set(x.to_string(), d.clone() - D::from(1));
                (d, s)
            },
        }
    }

    fn eval_bexpr(b: &Bexpr<D>, s: B)-> B {
        match b{
            Bexpr::True => s,
            Bexpr::False => AbstractState::bottom(),
            Bexpr::Equal(a1, a2) => {
                match (*a1.clone(),*a2.clone()) {
                    (Aexpr::Num(c), Aexpr::Var(x)) | (Aexpr::Var(x), Aexpr::Num(c)) =>
                        test_eq_case_1(s, x, c),
                    (Aexpr::Var(x), Aexpr::Var(y)) => 
                        test_eq_case_2(s, x, y, D::from(0)),
                    (a1,a2) => {
                        let (n1, s1) = Self::eval_aexpr(&a1, s);
                        let (n2, s2) = Self::eval_aexpr(&a2, s1);
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
                match (*a1.clone(),*a2.clone()) {
                    (Aexpr::Num(c), Aexpr::Var(x)) | (Aexpr::Var(x), Aexpr::Num(c)) => 
                        test_lte_case_1(s, x, c),
                    (Aexpr::Var(x), Aexpr::Var(y)) =>
                        test_lte_case_2(s, x, y),
                    (_, _) => s
                }                
                
            },
            Bexpr::And(_, _) => todo!(),

            
            Bexpr::Not(b) => {
                match *b.clone() { 
                    Bexpr::True => AbstractState::bottom(),
                    Bexpr::False => s,
                    Bexpr::Equal(a1, a2) => {
                        let (n1, s1) = Self::eval_aexpr(&a1, s);
                        let (n2, s2) = Self::eval_aexpr(&a2, s1);
                        // let dom = n1.glb(&n2);
                        if n1 == n2 {
                            AbstractState::bottom()
                        }else{
                            s2   
                        }
                    },
                    Bexpr::LessEq(a1, a2) => {
                        match (*a1.clone(),*a2.clone()) {
                            (Aexpr::Num(c), Aexpr::Var(x)) | (Aexpr::Var(x), Aexpr::Num(c)) => 
                                test_gt_case_1(s, x, c),
                            (Aexpr::Var(x), Aexpr::Var(y)) => 
                                test_gt_case_2(s, x, y),
                            (_, _) => s
                        } 
                    },
                    Bexpr::Not(b) => Self::eval_bexpr(&b, s),
                    Bexpr::And(_, _) => todo!(),
                }
            },
        }
    }

    fn analyze(prog: Program<D>, init_state: B) -> HashMap<Label, B> {
        let mut all_state: HashMap<Label, B> = HashMap::new();


        for i in 0..=(prog.labels_num-1) {
            all_state.insert(i, AbstractState::bottom());
        }
        all_state.insert(prog.entry, init_state);


        let mut iteration_num= 1;

        println!("\nINITIAL STATES:\n{}\n", map_to_str(&all_state));
        let mut new_all_state = Self::make_iteration(&prog, all_state.clone(), StepType::WideningStep);
        while new_all_state != all_state {
            all_state = new_all_state;
            println!("ITERATION (∇) {}:\n{:?}\n",iteration_num, map_to_str(&all_state)); iteration_num+=1;
            new_all_state = Self::make_iteration(&prog, all_state.clone(), StepType::WideningStep); 
        }

        let mut new_all_state = Self::make_iteration(&prog, all_state.clone(), StepType::NarrowingStep);
        while new_all_state != all_state {
            all_state = new_all_state;
            println!("ITERATION (Δ) {}:\n{:?}\n",iteration_num, map_to_str(&all_state)); iteration_num+=1;
            new_all_state = Self::make_iteration(&prog, all_state.clone(), StepType::NarrowingStep); 
        }


        println!("\nFINAL STATES:\n{}\n", map_to_str(&all_state));


        all_state
    }
}

enum StepType {
    NormalStep,
    WideningStep,
    NarrowingStep
}

impl<D: AbstractDomain, B: AbstractState<D>> MyAnalyzer<D,B>{
    fn make_iteration(prog: &Program<D>, states: HashMap<Label, B>, step_type: StepType) -> HashMap<Label, B>{
        let mut all_states: HashMap<Label, B> = HashMap::new();
        for i in 0..=(prog.labels_num-1) {
            if i == prog.entry { all_states.insert(i, states.get(&i).unwrap().clone()); continue; }
            let arcs = Self::get_entering_arcs(prog, i);
            let mut new_state = B::bottom();
            // println!("label {i} entering arcs:{:?}", &arcs);
            for (l,cmd,_) in arcs {
                match  states.get(l) {
                    Some(s) => new_state = new_state.lub(Self::apply_cmd(cmd, s)),
                    None => panic!("Missing AbsState for label {l}"),
                };
            }

            if prog.widening_points.contains(&i) {
                // print!("{} ∇ {} = ", states.get(&i).unwrap(), new_state);
                let old_state = states
                    .get(&i)
                    .expect(&format!("Missing AbsState for label {i}"))
                    .clone();
                new_state = match step_type {
                    StepType::NormalStep => new_state,
                    StepType::WideningStep => old_state.widening(new_state),
                    StepType::NarrowingStep => old_state.narrowing(new_state),
                }
                // println!("{}", new_state);
            }

            all_states.insert(i, new_state);
        };
        all_states
    }


    fn apply_cmd(cmd: &Command<D>, old_state: &B) -> B{
        let mut state = old_state.clone();
        match cmd {
            Command::Assignment(x, a) => {
                let (aexpr_dom, mut s2) = Self::eval_aexpr(a, state);
                // println!("aexpr_dom: {:?}", aexpr_dom);
                s2.set(x.to_string(), aexpr_dom);
                state = s2
            },
            Command::Test(b) => {
                state = Self::eval_bexpr(b, state);
                // println!("Apply test: {b} -> {old_state} -> {state}");
            },
        }
        state
    }

    fn get_entering_arcs(prog: &Program<D>, label: Label) -> Vec<&Arc<D>>{
        prog.arcs.iter().filter(|(_,_,l)|l==&label).collect()
    }
}


/**
 * X - c = 0
 */
fn test_eq_case_1<D: AbstractDomain, B: AbstractState<D>>(mut state: B, x: String, c: D)-> B{
    state.set(x.clone(), state.get(&x).glb(&c));
    state
}

/**
 * X - Y - c = 0
 */
fn test_eq_case_2<D: AbstractDomain, B: AbstractState<D>>(state: B, x: String, y: String, c: D)-> B{
    let s1 =match state.get(&x) {
        d if d==D::bottom() || d==D::top()  => state.clone(),
        _ => test_eq_case_1(state.clone(), x.clone(), state.get(&y) + c.clone())
    };
    let s2 =match state.get(&y) {
        d if d==D::bottom() || d==D::top()  => state,
        _ => test_eq_case_1(state.clone(), y, state.get(&x) - c)
    };
    s1.glb(&s2)
}


/**
 * X - c <= 0
 */
fn test_lte_case_1<D: AbstractDomain, B: AbstractState<D>>(mut state: B, x: String, c: D)-> B{
    state.set(x.clone(), state.get(&x).glb(&D::lte(&c)));
    state
}

/**
 * X - Y <= 0
 */
fn test_lte_case_2<D: AbstractDomain, B: AbstractState<D>>(mut state: B, x: String, y: String)-> B{
    let x_val = state.get(&x);
    let y_val = state.get(&y);
    state.set(x, x_val.glb(&D::lte(&y_val)));
    state.set(y, y_val.glb(&D::gte(&x_val)));
    state
}

/**
 * X - c > 0
 */
fn test_gt_case_1<D: AbstractDomain, B: AbstractState<D>>(mut state: B, x: String, c: D)-> B{
    state.set(x.clone(), state.get(&x).glb(&D::gte(&(c+D::from(1)))));
    state
}
/**
 * X - Y > 0 // => Y - X < 0 => Y - X <= 1
 */
fn test_gt_case_2<D: AbstractDomain, B: AbstractState<D>>(mut state: B, x: String, y: String)-> B{
    let x_val = state.get(&x);
    let y_val = state.get(&y);
    state.set(x, x_val.glb(&D::gte(&(y_val.clone() + D::from(1)))));
    state.set(y, y_val.glb(&D::gte(&(x_val - D::from(1)))));
    state
}
