use std::{fmt::{Display, write, Debug}, collections::HashMap};

use iter_tools::Itertools;

use super::{AbstractState, domains::sign_domain::Sign, program::Label, AbstractDomain};



impl<D: Display> Display for AbstractState<D> {
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

pub fn map_to_str<D: Display>(map: &HashMap<Label,D>) -> String{
    let str = map.iter().sorted_by_key(|(k,_)|*k).map(|(k,v)|format!("{k}: {v}")).join(", ");
    format!("{{{str}}}")
}

