use std::{fmt::Display, ops::{Add, Div, Mul, Sub}, str::FromStr};

use crate::{analyzer::types::domain::{AbstractDomain, Interval}, types::ast::Num};

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum CongruenceDomain{
    Bottom,
    Congruence {a: Num, b: Num},
}



impl Add for CongruenceDomain  {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (CongruenceDomain::Bottom, _) | (_, CongruenceDomain::Bottom) => CongruenceDomain::Bottom,
            (CongruenceDomain::Congruence { a:a1, b:b1 }, CongruenceDomain::Congruence { a:a2, b:b2 }) =>
                CongruenceDomain::Congruence { a: gcd(a1,a2), b: b1+b2 },
        }
    }
}
impl Sub for CongruenceDomain  {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (CongruenceDomain::Bottom, _) | (_, CongruenceDomain::Bottom) => CongruenceDomain::Bottom,
            (CongruenceDomain::Congruence { a:a1, b:b1 }, CongruenceDomain::Congruence { a:a2, b:b2 }) =>
                CongruenceDomain::Congruence { a: gcd(a1,a2), b: b1-b2 },
        }
    }
}
impl Mul for CongruenceDomain  {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (CongruenceDomain::Bottom, _) | (_, CongruenceDomain::Bottom) => CongruenceDomain::Bottom,
            (CongruenceDomain::Congruence { a:a1, b:b1 }, CongruenceDomain::Congruence { a:a2, b:b2 }) =>
                CongruenceDomain::Congruence { 
                    a: gcd(gcd(a1*a2,a1*b2),a2*b1), 
                    b: b1*b2 
                },
        }
    }
}
impl Div for CongruenceDomain  {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (CongruenceDomain::Bottom, _) | (_, CongruenceDomain::Bottom) => CongruenceDomain::Bottom,
            (CongruenceDomain::Congruence { a:a1, b:b1 }, CongruenceDomain::Congruence { a:a2, b:b2 }) =>{
                if a2 == 0 && b2 == 0 {
                    CongruenceDomain::Bottom 
                } else if a2 == 0 && b2 != 0 && (a1%b2 == 0) && (b2%b2 == 0) {
                    CongruenceDomain::Congruence { a: a1/b2.abs(), b: b1/b2 }
                } else {
                    Self::top()
                }
            }
        }
    }
}

impl PartialOrd for CongruenceDomain{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl FromStr for CongruenceDomain{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "⊥" || s == "bot" {
            return Ok(CongruenceDomain::Bottom);
        }

        let (a, rest) = match s.split_once(&['z', 'Z', 'ℤ']){
            Some(p) => p,
            None => return Err("Malformed congruence expected: \"<a>Z+<b>\"".to_owned()),
        };
        let (ws, b) = match rest.split_once('+') {
            Some(p) => p,
            None =>  return Err("Malformed congruence expected: \"<a>Z+<b>\"".to_owned()),
        };
        if !ws.trim().is_empty() {
            return Err("Malformed congruence expected: \"<a>Z+<b>\"".to_owned())
        }

        let a = match a.trim().parse() {
            Ok(n) => n,
            Err(_) => return Err(format!("Invalid <a> in congruence: {a}")),
        };
        let b = match b.trim().parse() {
            Ok(n) => n,
            Err(_) => return Err(format!("Invalid <b> in congruence: {b}")),
        };

        return Ok(CongruenceDomain::Congruence { a, b })
    }
}
impl From<Interval> for CongruenceDomain{
    fn from(value: Interval) -> Self {
        match value {
            Interval::Closed(l, r) if l == r => 
                CongruenceDomain::Congruence { a: 0, b: l },
            _ => CongruenceDomain::top(),
        }
    }
}
impl From<Num> for CongruenceDomain{
    fn from(value: Num) -> Self {
        CongruenceDomain::Congruence { a: 0, b: value }
    }
}
impl Display for CongruenceDomain{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            CongruenceDomain::Bottom => write!(f, "⊥"),
            CongruenceDomain::Congruence { a, b } => write!(f, "{}ℤ+{}",a,b),
        }
        
    }
}

// impl CongruenceDomain {
//     fn new(a: Num, b: Num) -> Self {
//         CongruenceDomain::Congruence { a, b: b % a }
//     }
// }

impl AbstractDomain for CongruenceDomain{
    fn bottom() -> Self {
        Self::Bottom
    }

    fn top() -> Self {
        Self::Congruence{a:1, b:0}
    }

    fn lub(self, other: Self) -> Self {
        match (self,other) {
            (CongruenceDomain::Bottom, cong) |
            (cong, CongruenceDomain::Bottom) => cong,
            (CongruenceDomain::Congruence { a: a1, b: b1 }, CongruenceDomain::Congruence { a: a2, b: b2 }) => {
                let a = gcd(gcd(a1,a2), (b1-b2).abs());
                CongruenceDomain::Congruence { a, b: b1} // Indifferent between b1 and b2
            }
        }
    }

    fn glb(self, other: Self) -> Self {
        match (self,other) {
            (CongruenceDomain::Bottom, _) | (_, CongruenceDomain::Bottom)  => CongruenceDomain::Bottom,
            (CongruenceDomain::Congruence { a: a1, b: b1 }, CongruenceDomain::Congruence { a: a2, b: b2 }) => {
                    let gcd = extended_euclidean_algorithm(a1, a2).0;

                    if gcd == 0 {
                        if b1 == b2 {
                            return CongruenceDomain::Congruence { a: 0, b: b1 };
                        } else {
                            return CongruenceDomain::Bottom;
                        }
                    }

                    if (b2 - b1) % gcd != 0 {
                        return CongruenceDomain::Bottom;                        
                    }
                    
                    // m1 m2 are coprime modules
                    let m1 = a1 / gcd;
                    let m2 = a2 / gcd;
                    let b_diff = b2 - b1;
                
                    // We need x = b [m1 v m2]
                    let lcm = m1 * m2 * gcd;
                
                    // Use the formula x = b1 + m1 * (b_diff / g) * inv_mod(m1, m2)
                    let inverse = mod_inv(m1, m2).unwrap();
                    let solution = b1 + m1 * ((b_diff / gcd) * inverse % m2);
                
                    if lcm == 0 {
                        CongruenceDomain::Congruence{a: 0, b: solution}
                    } else { 
                        CongruenceDomain::Congruence{a: lcm, b: solution.rem_euclid(lcm)}
                    }
            }
        }
    }

    fn narrowing(self, other:Self) -> Self {
        match (self, other) {
            (CongruenceDomain::Congruence {a: 1, b: _ }, _) => other,
            _ => self
        }
    }
}

// gcd(x,y) extended with gcd(0,x) = gcd(x,0) = x
fn gcd(a: Num, b: Num) -> Num{
    extended_euclidean_algorithm(a, b).0
}

// fn cong(a:Num, b:Num, m:Num) -> bool {
//     let dif = (a).abs_diff(b) as i128;
//     (dif % m) == 0
// }


fn mod_inv(x: Num, n: Num) -> Option<Num> {
    let (g, x, _) = extended_euclidean_algorithm(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
fn update_step(a: &mut Num, old_a: &mut Num, quotient: Num) {
    let temp = *a;
    *a = *old_a - quotient * temp;
    *old_a = temp;
}
fn extended_euclidean_algorithm(a: Num, b: Num) -> (Num, Num, Num) {
    let (mut old_r, mut rem) = (a, b);
    let (mut old_s, mut coeff_s) = (1, 0);
    let (mut old_t, mut coeff_t) = (0, 1);

    while rem != 0 {
        let quotient = old_r / rem;

        update_step(&mut rem, &mut old_r, quotient);
        update_step(&mut coeff_s, &mut old_s, quotient);
        update_step(&mut coeff_t, &mut old_t, quotient);
    }

    (old_r, old_s, old_t)
}