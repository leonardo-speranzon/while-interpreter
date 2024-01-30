use crate::{analyzer::{domains::interval_domain::Interval, states::hashmap_state::HashMapState, types::{analyzer::StaticAnalyzer, program::{Label, Program}}}, types::ast::{Aexpr, Bexpr}};



struct IntervalAnalyzer;

impl StaticAnalyzer<Interval, HashMapState<Interval>> for IntervalAnalyzer{
    fn eval_aexpr(a: &Aexpr<Interval>, s: HashMapState<Interval>)-> (Interval, HashMapState<Interval>) {
        todo!()
    }

    fn eval_bexpr(b: &Bexpr<Interval>, s: HashMapState<Interval>)-> HashMapState<Interval> {
        todo!()
    }

    fn analyze(p: Program<Interval>, init_state: HashMapState<Interval>) -> std::collections::HashMap<Label, HashMapState<Interval>> {
        todo!()
    }
}