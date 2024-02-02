use std::{fmt::Display, collections::HashMap};
use iter_tools::Itertools;
use regex::Regex;
use crate::types::ast::{Num, Statement};

use super::types::program::Label;


pub fn map_to_str<D: Display>(map: &HashMap<Label,D>) -> String{
    let str = map.iter().sorted_by_key(|(k,_)|*k).map(|(k,v)|format!("{k}: {v}")).join(", ");
    format!("{{{str}}}")
}

pub fn print_stm_with_inv(stm:Statement<Num>) -> String{
    let str = stm.to_string();
    let re = Regex::new(r"^ *while( |\()").unwrap();
    let mut inv_num = 0;
    
    str
        .lines()
        .fold(String::new(), |a,s| {
            if re.is_match(s) { inv_num +=1; format!("{a}i{inv_num}  > {s}\n") }
            else { a+"    > "+s+"\n" }
        } )   
}