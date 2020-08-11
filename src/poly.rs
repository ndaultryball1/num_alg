use itertools::{
    Itertools,
    EitherOrBoth::*,
};
// let syms = vec!["x", "y", "z", "w"];

pub struct UniPoly {
    pub coeffs: Vec<i64>,
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
}

#[test]
fn test_add() {
    let ex_1 = UniPoly{coeffs:vec!(1,2,3,4)};
    let ex_2 = UniPoly{coeffs:vec!(2,2,2,2)};
    assert_eq!(ex_1.add(&ex_2).coeffs, vec!(3,4,5,6));
    let ex_3 = UniPoly{coeffs: vec!(1,2)};
    assert_eq!(ex_1.add(&ex_3).coeffs, vec!(2,4,3,4));
}