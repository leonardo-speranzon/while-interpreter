mod sign_domain;

use crate::{interpreter::types::State, types::ast::{Statement, Aexpr, Bexpr, Operator}};

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
    fn glb(&self, other: &Self) -> Self { todo!() }
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
    fn eval_stm(stm: &Statement<D>, s: AbstractState<D>)-> AbstractState<D>;
    fn eval_aexpr(a: &Aexpr<D>, s: &AbstractState<D>)-> D;
    fn refine_aexpr(a: &Aexpr<D>,s:AbstractState<D>, dom: &D) -> AbstractState<D>;
    fn eval_bexpr(b: &Bexpr<D>, s: AbstractState<D>)-> AbstractState<D>;
}

struct MyAnalyzer{}


impl<D: AbstractDomain> StaticAnalyzer<D> for MyAnalyzer{
    fn eval_stm(stm: &Statement<D>, s: AbstractState<D>)-> AbstractState<D> {
        todo!()
    }

    fn eval_aexpr(a: &Aexpr<D>, s: &AbstractState<D>)-> D {
        match a {
            Aexpr::Num(n) => n.clone(),
            Aexpr::Var(x) => match s {
                AbstractState(Some(s)) => match s.get(x) {
                    Some(n) => n.clone(),
                    None => D::bottom(),
                },
                AbstractState(None) => D::bottom(),
            },
            Aexpr::BinOp(op, a1, a2 ) => {
                let n1 = Self::eval_aexpr(a1, s);
                let n2 = Self::eval_aexpr(a2, s);
                D::abstract_operator(op, &n1, &n2)
            }
        }
    }

    fn eval_bexpr(b: &Bexpr<D>, s: AbstractState<D>)-> AbstractState<D> {
        match b{
            Bexpr::True => s,
            Bexpr::False => AbstractState::bottom(),
            Bexpr::Equal(a1, a2) => {
                let n1 = Self::eval_aexpr(a1, &s);
                let n2 = Self::eval_aexpr(a2, &s);
                let dom = n1.glb(&n2);
                let s = Self::refine_aexpr(a1, s, &dom);
                let s = Self::refine_aexpr(a2, s, &dom);
                s
            },
            Bexpr::LessEq(_, _) => todo!(),
            Bexpr::Not(_) => todo!(),
            Bexpr::And(_, _) => todo!(),
        }
    }

    fn refine_aexpr(a: &Aexpr<D>, s: AbstractState<D>, dom: &D) -> AbstractState<D> {
        match a {
            Aexpr::Num(n) => {
                if n.glb(dom) == D::bottom() {
                    AbstractState::bottom()
                } else {
                    s
                }
            },
            Aexpr::Var(x) => {
                match s {
                    AbstractState(Some(mut s)) => {
                        let glb = s.get(x)
                            .unwrap_or(&AbstractDomain::top())
                            .glb(dom);
                        if glb == D::bottom() {
                            AbstractState(None)
                        } else {
                            s.insert(x.clone(), glb);
                            AbstractState(Some(s))
                        }
                    }
                    AbstractState(None) => s
                }
            },
            Aexpr::BinOp(op, a1, a2)=>{
                let n1 = Self::eval_aexpr(a1, &s);
                let n2 = Self::eval_aexpr(a2, &s);

                let (d1,d2) = D::backward_abstract_operator(
                    op,
                    &n1,
                    &n2,
                    dom
                );

                let s = Self::refine_aexpr(a1, s,&d1);
                let s = Self::refine_aexpr(a2, s,&d2);                
                s
            }
        }
    }
}

