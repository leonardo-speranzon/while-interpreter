mod sign_domain;
pub mod my_analyzer;

use std::{ops::RangeFull, cmp::max, os::unix::raw::off_t, collections::HashMap};

use crate::{interpreter::types::State, types::ast::{Statement, Aexpr, Bexpr, Operator, Var}};

#[derive(PartialEq)]
struct AbstractState<D>(Option<State<D>>);

impl<D: AbstractDomain> AbstractState<D> {
    fn bottom() -> Self{
        AbstractState(None)
    }
    fn top() -> Self{
        AbstractState(Some(State::new()))
    }    
    fn lub(&self, other: &Self) -> Self { todo!() } 
    fn glb(self, other: &Self) -> Self { 
        match (self, other) {
            (AbstractState(Some(mut s1)),AbstractState(Some(s2))) => {
                for (k,v) in s2.into_iter() {
                    let new_v = match s1.get(k) {
                        Some(d) => v.glb(d),
                        None => v.clone(),
                    };
                    if new_v == D::bottom(){
                        return AbstractState(None)
                    }
                    s1.insert(k.to_string(), new_v);
                }
                AbstractState(Some(s1))
            },
            (_,_) => AbstractState(None)
        }
     }
}

impl<D: AbstractDomain> PartialOrd for AbstractState<D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

trait AbstractDomain : PartialOrd + Clone + Sized {
    fn bottom() -> Self;
    fn top() -> Self;
    fn lub(&self, other: &Self) -> Self;
    fn glb(&self, other: &Self) -> Self;

    fn abstract_operator(op: &Operator, lhs: &Self, rhs: &Self) -> Self;
    fn backward_abstract_operator(op: &Operator, lhs: &Self, rhs: &Self, res: &Self) -> (Self, Self);
    // fn widening();
    // fn narrowing();
}

trait StaticAnalyzer<D: AbstractDomain> {
    fn eval_aexpr(a: &Aexpr<D>, s: &AbstractState<D>)-> D;
    fn refine_aexpr(a: &Aexpr<D>,s:AbstractState<D>, dom: &D) -> AbstractState<D>;
    fn eval_bexpr(b: &Bexpr<D>, s: AbstractState<D>)-> AbstractState<D>;

    fn init(stm: Statement<D>) -> Program<D> {
        stm_to_program(stm)
    }
    fn analyze(p: Program<D>) -> HashMap<Label, AbstractState<D>>{
        todo!()
    } 
    
}
fn stm_to_program<D:AbstractDomain>(stm: Statement<D>) -> Program<D>{
    match stm {
        Statement::Assign(x, a) => 
            Program::new(vec![(0,Command::Assignment(x,*a),1)]),
        Statement::Skip => Program::new(Vec::new()),
        Statement::Compose(s1, s2) => {
            let mut p1 = stm_to_program(*s1);
            let p2 = stm_to_program(*s2);
            let offset = p1.labels_num - 1;
            let mut arcs2: Vec<Arc<D>> = p2.arcs
                .into_iter()
                .map(|(l1,c,l2)|(l1+offset, c, l2+offset))
                .collect();
            arcs2.append(&mut p1.arcs);
            Program::new(arcs2)
        },
        Statement::IfThenElse(b, s1, s2) => {
            let p1 = stm_to_program(*s1);
            let p2 = stm_to_program(*s2);
            let offset_p1 = 1;
            let offset_p2 = p1.labels_num;
            let exit_label = p1.labels_num + p2.labels_num - 1;
            let mut arcs = vec![
                (0,Command::Test(*b.clone()),offset_p1),
                (0,Command::Test(Bexpr::Not(b)), offset_p2)
            ];
            let mut p1_arcs: Vec<Arc<D>> = p1.arcs
                .into_iter()
                .map(|(l1,c,l2)|{
                    if l2 == p1.exit {
                        (l1+offset_p1, c, exit_label)
                    } else {
                        (l1+offset_p1, c, l2+offset_p1)
                    }
                })
                .collect();
            let mut p2_arcs: Vec<Arc<D>> = p2.arcs
                .into_iter()
                .map(|(l1,c,l2)|{
                    if l2 == p2.exit {
                        (l1+offset_p2, c, exit_label)
                    } else {
                        (l1+offset_p2, c, l2+offset_p2)
                    }
                })
                .collect();
            arcs.append(&mut p1_arcs);
            arcs.append(&mut p2_arcs);
            Program::new(arcs)
        },
        Statement::While(b, s) => {
            let p1 = stm_to_program(*s);
            let offset = 1;
            let exit_label = p1.exit + 1;
            let mut arcs = vec![
                (0,Command::Test(*b.clone()), 1),
                (0,Command::Test(Bexpr::Not(b)), exit_label)
            ];
            let mut p1_arcs: Vec<Arc<D>> = p1.arcs
                .into_iter()
                .map(|(l1,c,l2)|{
                    if l2 == p1.exit {
                        (l1+offset, c, 0)
                    } else {
                        (l1+offset, c, l2+offset)
                    }
                })
                .collect();

            arcs.append(&mut p1_arcs);
            Program::new(arcs)
        },
    }
}

struct Program<D: AbstractDomain> {
    labels_num: Label,
    entry: Label, // always first index
    exit: Label,  // always last index
    arcs: Vec<Arc<D>>
}

type Label = i32;
type Arc<D> = (Label, Command<D>, Label);
enum Command<D: AbstractDomain> {
    Assignment(Var, Aexpr<D>),
    Test(Bexpr<D>),
}

impl<D: AbstractDomain> Program<D>{
    fn new(arcs: Vec<(Label, Command<D>, Label)>) -> Self {
        let max_label = (&arcs)
            .into_iter()
            .map(|(l1,_,l2)| max(l1,l2))
            .max()
            .unwrap_or(&0);
        Program {
            labels_num: max_label + 1,
            entry: 0,
            exit: *max_label,
            arcs,
        }
    }
}