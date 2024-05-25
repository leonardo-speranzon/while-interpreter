use std::{cmp::Ordering, collections::HashSet, fmt::Display, str::FromStr};

use iter_tools::Itertools;

use crate::{analyzer::types::{domain::AbstractDomain, state::AbstractState}, interpreter::types::State};



#[derive(Debug, PartialEq, Clone)]
pub struct HashMapState<D>(pub Option<State<D>>);

impl<D: AbstractDomain> AbstractState<D> for HashMapState<D>  {
    fn bottom() -> Self{
        HashMapState(None)
    }
    fn top() -> Self{
        HashMapState(Some(State::new()))
    }    
    fn lub(self, other: &Self) -> Self { 
        // print!("LUB {:?}, {:?} -> ",self,other);
        let new_s=  match (self, other) {
            (HashMapState(Some(mut s1)),HashMapState(Some(s2))) => { 
                s1 = s1.into_iter().filter_map(|(k,v)|{
                    match s2.get(&k) {
                        Some(d) => Some((k, v.lub(d.clone()))),
                        None => None,
                    }
                }).collect(); 
                HashMapState(Some(s1))
            },
            (HashMapState(Some(s)), _) => HashMapState(Some(s)),
            (_, HashMapState(Some(s))) => HashMapState(Some(s.clone())),
            (_, _) => HashMapState(None)
        };
        new_s
    } 
    fn glb(self, other: &Self) -> Self { 
        match (self, other) {
            (HashMapState(Some(mut s1)),HashMapState(Some(s2))) => {
                for (k,v) in s2.into_iter() {
                    let new_v = match s1.get(k) {
                        Some(d) => v.glb(d.clone()),
                        None => v.clone(),
                    };
                    if new_v == D::bottom(){
                        return HashMapState(None)
                    }
                    s1.insert(k.to_string(), new_v);
                }
                HashMapState(Some(s1))
            },
            (_,_) => HashMapState(None)
        }
    }
    fn get(&self, k: &str) -> D {
        match self {
            HashMapState(Some(s)) => {
                match s.get(k) {
                    Some(n) => n.clone(),
                    None => D::top(),
                }
            },
            HashMapState(None) =>  D::bottom(),
        }
    }
    fn set(&mut self, k: String, v: D) {
        match self {
            HashMapState(Some(s)) => {
                if v == D::bottom() {
                    self.0 = None
                }else {
                    s.insert(k, v);
                }
            },
            HashMapState(None) => (),
        }
    }

    fn widening(self, other: Self) -> Self {
        match (self.0, other.0){
            (None, s) | (s, None) => HashMapState(s),
            (Some(mut s1), Some(s2)) => {
                for (key, value) in s2 {
                    let d = match s1.remove(&key) {
                        Some(d) => d.widening(value),
                        None => value,
                    };
                    s1.insert(key, d);
                };
                HashMapState(Some(s1))
            },
        }
    }
    fn narrowing(self, other: Self) -> Self {
        match (self.0, other.0){
            (None, s) | (s, None) => HashMapState(s),
            (Some(mut s1), Some(s2)) => {
                for (key, value) in s2 {
                    let d = match s1.remove(&key) {
                        Some(d) => d.narrowing(value),
                        None => value,
                    };
                    s1.insert(key, d);
                };
                HashMapState(Some(s1))
            },
        }
    }
    
}

impl<D: AbstractDomain> PartialOrd for HashMapState<D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.0, &other.0) {
            (None, None) => Some(Ordering::Equal),
            (None, Some(_)) => Some(Ordering::Less),
            (Some(_), None) => Some(Ordering::Greater),
            (Some(s1), Some(s2)) => {
                let mut ord_candidate: Option<Ordering> = None;
                let all_keys = s1.keys().chain(s2.keys()).collect::<HashSet<_>>();
                for k in all_keys {
                    let top = D::top();
                    let v1 = s1.get(k).unwrap_or(&top);
                    let v2 = s2.get(k).unwrap_or(&top);

                    match (v1.partial_cmp(v2), ord_candidate) {
                        (None, _) => return None,
                        (Some(ord), None) => { ord_candidate=Some(ord); },
                        (Some(ord), Some(c_ord)) if ord == c_ord => (),
                        (Some(_), Some(_)) => return None,
                    }
                }
                return ord_candidate;
            },
        }
    }
}

impl<D:AbstractDomain> FromStr for HashMapState<D> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let state = s
            .split(';')
            .map(|pair_str|{
                match pair_str.split_once(':'){
                    Some((var,val)) => {
                        match val.parse::<D>() {
                            Ok(n) => Ok((var.to_string(), n)),
                            Err(_) => Err(format!("Wrong format of state pair, expected '<var-name>:<value>' but found '{pair_str}'")),
                        }                                    
                    }
                    None => Err(format!("Wrong format of state pair, expected '<var-name>:<value>' but found '{pair_str}'")),    
                }
            }).collect::<Result<_,_>>()?;
        Ok(Self(Some(state)))
    }
}

impl<D: Display> Display for HashMapState<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(s) => {
                if s.is_empty() {
                    write!(f, "⊤")
                } else {
                    write!(f, "{{")?;
                    let str = s.iter().sorted_by_key(|(k,_)|*k).map(|(k,v)|format!("{k}: {v}")).join(", ");
                    write!(f, "{str}")?;
                    write!(f, "}}")
                }
            },
            None => write!(f, "⊥"),
        }
    }
}



