use std::{fmt::Display, str::FromStr};

use iter_tools::Itertools as _;

use super::ast::{Num, NumLiteral};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct LitInterval (pub Num, pub Num);

impl From<Num> for LitInterval {
    fn from(value: Num) -> Self {
        LitInterval(value, value)
    }
}
impl FromStr for LitInterval {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<Num>(){
            return Ok(LitInterval(n,n));
        };
        let mut chars = s.chars();
        match chars.next() {
            Some('[') => (),
            _ => return Err(format!("Expected \"[l,u]\", found {s}")),
        }
        let lower: String = chars.take_while_ref(|c|c!=&',').collect();
        
        match chars.next() {
            Some(',') => (),
            _ => return Err(format!("Expected \"[l,u]\", found {s}")),
        }
        match chars.next_back() {
            Some(']') => (),
            _ => return Err(format!("Expected \"[l,u]\", found {s}")),
        }

        let upper: String = chars.collect();

        let lower = lower.parse::<Num>().map_err(|e|e.to_string())?;
        let upper = upper.parse::<Num>().map_err(|e|e.to_string())?;
        Ok(LitInterval(lower,upper))
    }
}
impl Display for LitInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == self.1 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "[{},{}]", self.0, self.1)
        }
    }
}
impl NumLiteral for LitInterval {}