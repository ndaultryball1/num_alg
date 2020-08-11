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
    pub fn add(&self, other: &UniPoly) -> UniPoly {
        let mut res = Vec::new();
        for pair in self.coeffs.iter().zip_longest(other.coeffs.iter()) {
            match pair {
                Both(l, r) => res.push(l + r),
                Left(l) => res.push(*l),
                Right(r) => res.push(*r)
            };
        }
        UniPoly{coeffs :res}
    }

    pub fn neg(&self) -> UniPoly {
        UniPoly{coeffs: self.coeffs.iter().map(|x| -x ).collect()}
    }

    pub fn mult(&self, other: &UniPoly) -> UniPoly {
        let len = self.coeffs.len() * other.coeffs.len();
        let mut res = vec![0; len];
        for ((i, exp_1), (j, exp_2)) in iproduct!(self.coeffs.iter().enumerate(), other.coeffs.iter().enumerate()) {
            res[(i + 1) * (j + 1) - 1] += exp_1 * exp_2;
        }
        UniPoly{coeffs: res}
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