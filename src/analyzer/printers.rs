use std::{fmt::Display, collections::HashMap};
use iter_tools::Itertools;
use super::types::program::Label;


pub fn map_to_str<D: Display>(map: &HashMap<Label,D>) -> String{
    let str = map.iter().sorted_by_key(|(k,_)|*k).map(|(k,v)|format!("{k}: {v}")).join(", ");
    format!("{{{str}}}")
}

