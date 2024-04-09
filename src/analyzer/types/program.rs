use std::cmp::max;
use iter_tools::Itertools;
use crate::types::ast::{Statement, Aexpr, Var, Bexpr};

#[derive(Debug, Clone)]
pub struct Program<D: Clone> {
    pub labels_num: Label,
    pub entry: Label, // always first index
    pub widening_points: Vec<Label>,
    pub arcs: Vec<Arc<D>>
}

pub type Label = u32;
pub type Arc<D> = (Label, Command<D>, Label);

#[derive(Debug, Clone)]
pub enum Command<D> {
    Assignment(Var, Aexpr<D>),
    Test(Bexpr<D>),
}

impl<D: Clone> Program<D>{
    pub fn new(arcs: Vec<(Label, Command<D>, Label)>, widening_points: Vec<Label>) -> Self {
        let max_label = (&arcs)
            .into_iter()
            .map(|(l1,_,l2)| max(l1,l2))
            .max()
            .unwrap_or(&0);
        Program {
            labels_num: max_label + 1,
            entry: 0,
            widening_points,
            arcs,
        }
    }
    

    pub fn get_entering_arcs(self: &Self, label: Label) -> Vec<&Arc<D>>{
        self.arcs.iter().filter(|(_,_,l)|l==&label).collect()
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
            Program::new(vec![(0,Command::Assignment(x,*a),1)], vec![]),
        Statement::Skip => Program::new(Vec::new(), vec![]),
        Statement::Compose(s1, s2) => {
            let mut p1 = stm_to_program(*s1);
            let p2 = stm_to_program(*s2);
            let offset = p1.labels_num - 1;
            let mut arcs2: Vec<Arc<D>> = shift_arcs(p2.arcs.clone(), offset,p2.labels_num-1,p2.labels_num-1+offset);
            arcs2.append(&mut p1.arcs);
            let widening_points = [p1.widening_points, p2.widening_points.iter().map(|x|x+offset).collect_vec()].concat();
            Program::new(arcs2, widening_points)
        },
        Statement::IfThenElse(b, s1, s2) => {
            let p1 = stm_to_program(*s1);
            let p2 = stm_to_program(*s2);

            let offset_p1 = if p1.labels_num > 1 { 1 } else { 0 };
            let offset_p2 = offset_p1 + 1;
            let exit_label = offset_p2 + p2.labels_num - 1 ;
            
            let mut arcs = vec![
                (0,Command::Test(*b.clone()),if p1.labels_num > 1 { 1 } else { exit_label }),
                (0,Command::Test(Bexpr::Not(b)), offset_p2)
            ];
            let mut p1_arcs: Vec<Arc<D>> = shift_arcs(p1.arcs.clone(), offset_p1,p1.labels_num-1,exit_label);
            let mut p2_arcs: Vec<Arc<D>> = shift_arcs(p2.arcs.clone(), offset_p2,p2.labels_num-1,exit_label);
            arcs.append(&mut p1_arcs);
            arcs.append(&mut p2_arcs);

            let widening_points = [
                p1.widening_points.iter().map(|x|x+offset_p1).collect_vec(),
                p2.widening_points.iter().map(|x|x+offset_p2).collect_vec()
            ].concat();
            Program::new(arcs, widening_points)
        },
        Statement::While(b, s) => {
            let p1 = stm_to_program(*s);
            let offset = 1;
            let exit_label = p1.labels_num;
            let mut arcs = vec![
                (0,Command::Test(*b.clone()), if p1.labels_num == 1 { 0 } else { 1 }),
                (0,Command::Test(Bexpr::Not(b)), exit_label)
            ];
            let mut p1_arcs: Vec<Arc<D>> = shift_arcs(p1.arcs.clone(), offset,p1.labels_num-1,0);
            arcs.append(&mut p1_arcs);

            let widening_points = [
                vec![0],
                p1.widening_points.iter().map(|x|x+offset).collect_vec()
            ].concat();
            Program::new(arcs, widening_points)
        },
    }
}

fn shift_arcs<D>(arcs: Vec<Arc<D>>, offset: Label, old_exit: Label, new_exit: Label) -> Vec<Arc<D>>{
    arcs.into_iter()
    .map(|(l1,c,l2)|{
        if l2 == old_exit {
            (l1+offset, c, new_exit)
        } else {
            (l1+offset, c, l2+offset)
        }
    })
    .collect()
}

pub trait ProgramInterface{
    fn get_end_label(&self)-> Label;
    fn get_loop_label(&self) -> &Vec<Label>;
}
impl<D: Clone> ProgramInterface for Program<D>  {
    fn get_end_label(&self)-> Label {
        self.labels_num -1 
    }

    fn get_loop_label(&self) -> &Vec<Label> {
        &self.widening_points
    }
}