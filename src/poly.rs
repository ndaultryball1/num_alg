use itertools::{
    Itertools,
    EitherOrBoth::*,
    iproduct,
};
// let syms = vec!["x", "y", "z", "w"];

pub struct UniPoly {
    coeffs: Vec<i64>,
}


impl UniPoly {
    pub fn new(coeffs: Vec<i64>) -> UniPoly {
        let mut res: Vec<i64> = Vec::new();
        let reved: Vec<i64> = coeffs.iter().rev().skip_while(|x| **x == 0).cloned().collect::<Vec<i64>>();
        UniPoly{coeffs:reved.iter().rev().cloned().collect()}
    }

    pub fn add(&self, other: &UniPoly) -> UniPoly {
        let mut res = Vec::new();
        for pair in self.coeffs.iter().zip_longest(other.coeffs.iter()) {
            match pair {
                Both(l, r) => res.push(l + r),
                Left(l) => res.push(*l),
                Right(r) => res.push(*r)
            };
        }
        UniPoly::new(res)
    }

    pub fn neg(&self) -> UniPoly {
        UniPoly::new(self.coeffs.iter().map(|x| -x ).collect())
    }

    pub fn mult(&self, other: &UniPoly) -> UniPoly {
        let len = self.coeffs.len() * other.coeffs.len();
        let mut res = vec![0; len];
        for ((i, exp_1), (j, exp_2)) in iproduct!(self.coeffs.iter().enumerate(), other.coeffs.iter().enumerate()) {
            println!("{}x^{} * {}x^{} = {}x^{}", exp_1, i, exp_2, j, exp_1 * exp_2, i + j);
            res[i + j] += exp_1 * exp_2;
        }
        UniPoly::new(res)
    }

    pub fn monomial(coeff: i64, deg: i64) -> UniPoly {
        let mut res = vec![0; (deg as usize) + 1];
        if let Some(last) = res.last_mut() {
            *last = coeff;
        };
        UniPoly::new(res)  
    }
}

#[test]
fn test_add() {
    
    let ex_1 = UniPoly{coeffs:vec!(1,2,3,4)};
    let ex_2 = UniPoly{coeffs:vec!(2,2,2,2)};
    // Add same degree
    assert_eq!(ex_1.add(&ex_2).coeffs, vec!(3,4,5,6));
    let ex_3 = UniPoly{coeffs: vec!(1,2)};
    // Different degrees
    assert_eq!(ex_1.add(&ex_3).coeffs, vec!(2,4,3,4));

    // subtraction
    assert_eq!(ex_1.add(&ex_3.neg()).coeffs, vec!(0,0,3,4));
}

#[test]
fn test_mult() {
    let ex_1 = UniPoly{coeffs:vec!(1,2,3)};
    let ex_2 = UniPoly{coeffs:vec!(2)};
    assert_eq!(ex_1.mult(&ex_2).coeffs, vec!(2,4,6));

    let ex_3 = UniPoly{coeffs:vec!(2, 3)};
    assert_eq!(ex_1.mult(&ex_3).coeffs, vec!(2,7,12,9))

}

#[test]
fn test_mon() {
    assert_eq!(UniPoly::monomial(5, 0).coeffs, vec!(5));
}

#[test]
fn test_new() {
    // Test creating a poly correctly deals with trailing zeros.
    assert_eq!(UniPoly::new(vec!(2,3,5,0,0,6,0,0)).coeffs, vec!(2,3,5,0,0,6));
}