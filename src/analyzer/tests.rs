use super::types::{domain::AbstractDomain, state::AbstractState};



/**
 * X - c = 0
 */
pub fn test_eq_case_1<D: AbstractDomain, B: AbstractState<D>>(mut state: B, x: String, c: D)-> B{
    state.set(x.clone(), state.get(&x).glb(&c));
    state
}

/**
 * X - Y - c = 0
 */
pub fn test_eq_case_2<D: AbstractDomain, B: AbstractState<D>>(state: B, x: String, y: String, c: D)-> B{
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
pub fn test_lte_case_1<D: AbstractDomain, B: AbstractState<D>>(mut state: B, x: String, c: D)-> B{
    state.set(x.clone(), state.get(&x).glb(&D::all_lte(&c)));
    state
}

/**
 * X - Y <= 0
 */
pub fn test_lte_case_2<D: AbstractDomain, B: AbstractState<D>>(mut state: B, x: String, y: String)-> B{
    let x_val = state.get(&x);
    let y_val = state.get(&y);
    state.set(x, x_val.glb(&D::all_lte(&y_val)));
    state.set(y, y_val.glb(&D::all_gte(&x_val)));
    state
}



/**
 * X - c >= 0
 */
pub fn test_gte_case_1<D: AbstractDomain, B: AbstractState<D>>(mut state: B, x: String, c: D)-> B{
    state.set(x.clone(), state.get(&x).glb(&D::all_gte(&c)));
    state
}



/**
 * X - c > 0
 */
pub fn test_lt_case_1<D: AbstractDomain, B: AbstractState<D>>(mut state: B, x: String, c: D)-> B{
    state.set(x.clone(), state.get(&x).glb(&D::all_lt(&c)));
    state
}

/**
 * X - c > 0
 */
pub fn test_gt_case_1<D: AbstractDomain, B: AbstractState<D>>(mut state: B, x: String, c: D)-> B{
    state.set(x.clone(), state.get(&x).glb(&D::all_gt(&c)));
    state
}
/**
 * X - Y > 0 // -> Y - X < 0 -> Y - X <= 1
 */
pub fn test_gt_case_2<D: AbstractDomain, B: AbstractState<D>>(mut state: B, x: String, y: String)-> B{
    let x_val = state.get(&x);
    let y_val = state.get(&y);
    state.set(x, x_val.glb(&D::all_lt(&y_val)));
    state.set(y, y_val.glb(&D::all_gt(&x_val)));
    state
}

/**
 * X - c != 0
 */
pub fn test_neq_case_1<D: AbstractDomain, B: AbstractState<D>>(mut state: B, x: String, c: D) -> B {
    let x_val = state.get(&x);
    let left = x_val.glb(&D::all_lt(&c));
    let right = x_val.glb(&D::all_gt(&c));
    state.set(x.clone(), left.lub(&right));
    state
}