use std::cmp::max;

use crate::types::ast::{Statement, Aexpr, Var, Bexpr};

#[derive(Debug)]
pub struct Program<D> {
    pub labels_num: Label,
    pub entry: Label, // always first index
    pub exit: Label,  // always last index
    pub arcs: Vec<Arc<D>>
}

pub type Label = u32;
pub type Arc<D> = (Label, Command<D>, Label);

#[derive(Debug)]
pub enum Command<D> {
    Assignment(Var, Aexpr<D>),
    Test(Bexpr<D>),
}

impl<D> Program<D>{
    pub fn new(arcs: Vec<(Label, Command<D>, Label)>) -> Self {
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

impl<D: Clone> From<Statement<D>> for Program<D>{
    fn from(value: Statement<D>) -> Self {
        stm_to_program(value)
    }
}

fn stm_to_program<D: Clone>(stm: Statement<D>) -> Program<D>{
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
