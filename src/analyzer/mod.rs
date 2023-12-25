// type AbstractState<D: AbstractDomain> = State<D>;
trait AbstractDomain {
    //partial-ord
    fn bottom() -> Self;
    fn top() -> Self;
    fn lub(&self, rhs: &Self) -> Self;
    fn glb(&self, rhs: &Self) -> Self;
    // fn widening();
    // fn narrowing();
    //arithmetic-ops
}

// trait StaticAnalyzer<D: AbstractDomain> {
//     fn eval_stm(stm: Statement<D>, s: AbstractState<D>)-> AbstractState<D>;
//     fn eval_aexpr(a: Aexpr<D>, s: AbstractState<D>)-> D;
//     fn eval_bexpr(b: Bexpr<D>, s: AbstractState<D>)-> AbstractState<D>;
// }

// struct MyAnalyzer{}


// impl<D: AbstractDomain> StaticAnalyzer<D> for MyAnalyzer{
//     fn eval_stm(stm: Statement<D>, s: AbstractState<D>)-> AbstractState<D> {
//         todo!()
//     }

//     fn eval_aexpr(a: Aexpr<D>, s: AbstractState<D>)-> D {
//         todo!()
//     }

//     fn eval_bexpr(b: Bexpr<D>, s: AbstractState<D>)-> AbstractState<D> {
//         todo!()
//     }
// }

// enum Sign{
//     Positive,
//     Zero,
//     Negative,
// }

// impl AbstractDomain for Sign{

// }
