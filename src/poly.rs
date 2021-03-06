#![allow(dead_code)]

use itertools::{
    Itertools,
    EitherOrBoth::*,
    iproduct,
};

use num_traits::{Num, Zero};
use std::fmt::Debug;

#[derive(Clone, Eq, Debug)]
struct UniPoly<T: Num + Copy + Debug> {
    coeffs: Vec<T>,
}

impl<T: PartialEq + Copy + Num + Debug> PartialEq for UniPoly<T> {
    fn eq(&self, other: &Self) -> bool {
        self.coeffs == other.coeffs
    }
}


impl<T: Num + Copy + Debug> UniPoly<T> {
    
    // Implementation of polynomial in a single variable, an element of k[x].

    fn add(&self, other: &UniPoly<T>) -> UniPoly<T> {
        let mut res = Vec::new();
        for pair in self.coeffs.iter().zip_longest(other.coeffs.iter()) {
            match pair {
                Both(l, r) => res.push(*l + *r),
                Left(l) => res.push(*l),
                Right(r) => res.push(*r)
            };
        }
        UniPoly::new(res)
    }

    fn new(coeffs: Vec<T>) -> UniPoly<T> {
        let reved: Vec<T> = coeffs.iter().rev().skip_while(|x| **x == T::zero()).cloned().collect();
        let res = UniPoly{coeffs:reved.iter().rev().cloned().collect()};
        // Want the zero polynomial to have degree zero rather than be empty
        if res.coeffs.len() != 0 {
            res
        } else {
            UniPoly{coeffs: vec!(T::zero())}
        }
    }

    fn sub(&self, other: &UniPoly<T>) -> UniPoly<T> {
        self.add(&other.neg())
    }

    fn mul(&self, other: &UniPoly<T>) -> UniPoly<T> {
        let len = self.coeffs.len() * other.coeffs.len();
        let mut res = vec![T::zero(); len];
        for ((i, exp_1), (j, exp_2)) in iproduct!(self.coeffs.iter().enumerate(), other.coeffs.iter().enumerate()) {
            res[i + j] = res[i + j] + (*exp_1 * *exp_2);
        }
        UniPoly::new(res)
    }

    fn neg(&self) -> UniPoly<T> {
        UniPoly::new(self.coeffs.iter().map(|&x| T::zero() - x ).collect())
    }

    fn monomial(coeff: T, deg: i64) -> UniPoly<T> {
        let mut res = vec![T::zero(); (deg as usize) + 1];
        if let Some(last) = res.last_mut() {
            *last = coeff;
        };
        UniPoly::new(res)  
    }

    fn degree(&self) -> i64 {
        self.coeffs.len() as i64 - 1
    }

    fn lead_term(&self) -> UniPoly<T> {
        // Pick off the highest term and create a monomial
        if let Some(coeff) = self.coeffs.last() {
            UniPoly::monomial(*coeff, self.degree())
        } else {
            println!("No leading term found in {:?}", self);
            panic!()
        }
    }

    fn div(&self, other:&UniPoly<T>) -> (UniPoly<T>, UniPoly<T>) {
        // Implements the division algorithm in k[x]. 
        // Returns (q, r) where self = f * other + r
        // In the book - self is f, other is g.
        let mut q: UniPoly<T> = UniPoly::monomial(T::zero(), 0); // Result
        let mut r: UniPoly<T> = self.clone(); // Remainder
        let ltg = other.lead_term();
        while (r.coeffs != vec!(T::zero())) & (r.lead_term().degree() >= other.lead_term().degree()) {
            println!("q: {:?}, r: {:?}", q, r);
            q = q.add(&r.lead_term().div_mono(&ltg));
            r = r.sub(&(r.lead_term().div_mono(&ltg).mul(&other)))
        }
        (q, r)
    }

    fn div_mono(&self, other: &UniPoly<T>) -> UniPoly<T> {
        // Problems with integer coefficients? We shouldnt be handling non-fields anyway...
        UniPoly::monomial(*self.coeffs.last().unwrap() / *other.coeffs.last().unwrap(), self.degree() - other.degree())
    } 
}


#[cfg(test)]
mod test_uni {
    use super::*;

    #[test]
    fn test_add() {
        
        let ex_1 = UniPoly{coeffs:vec!(1,2,3,4)};
        let ex_2 = UniPoly{coeffs:vec!(2,2,2,2)};
        // Add same degree
        assert_eq!((ex_1.add(&ex_2)).coeffs, vec!(3,4,5,6));
        let ex_3 = UniPoly{coeffs: vec!(1,2)};
        // Different degrees
        assert_eq!((ex_1.add(&ex_3)).coeffs, vec!(2,4,3,4));

        // subtraction
        assert_eq!((ex_1.sub(&ex_3)).coeffs, vec!(0,0,3,4));
    }

    #[test]
    fn test_mult() {
        let ex_1 = UniPoly{coeffs:vec!(1,2,3)};
        let ex_2 = UniPoly{coeffs:vec!(2)};
        assert_eq!((ex_1.mul(&ex_2)).coeffs, vec!(2,4,6));

        let ex_3 = UniPoly{coeffs:vec!(2, 3)};
        assert_eq!((ex_1.mul(&ex_3)).coeffs, vec!(2,7,12,9))

    }

    #[test]
    fn test_mon() {
        assert_eq!(UniPoly::monomial(5, 0).coeffs, vec!(5));
        assert_eq!(UniPoly::monomial(i64::zero(), 0), UniPoly::new(vec!(0)));
    }

    #[test]
    fn test_new() {
        // Test creating a poly correctly deals with trailing zeros.
        assert_eq!(UniPoly::new(vec!(0,2,3,5,0,0,6,0,0)).coeffs, vec!(0,2,3,5,0,0,6));
    }

    #[test]
    fn test_div_mono() {
        let x_squared = UniPoly::monomial(1, 2);
        let x = UniPoly::monomial(1,1);
        let three_x_cubed = UniPoly::monomial(3, 3);
        assert_eq!(x_squared.div_mono(&x), x);
        assert_eq!(three_x_cubed.div_mono(&x_squared), UniPoly::monomial(3, 1));
    }

    #[test]
    fn test_div_simple() {
        // Test division algo agrees with normal division for simple cases
        let x_squared = UniPoly::monomial(1, 2);
        let x = UniPoly::monomial(1,1);
        let three_x_cubed = UniPoly::monomial(3, 3);
        assert_eq!(x_squared.div(&x), (x, UniPoly::monomial(0, 0)));
        assert_eq!(three_x_cubed.div(&x_squared), (UniPoly::monomial(3, 1), UniPoly::monomial(0, 0)));
    }

    #[test]
    fn test_div_long() {
        let cubic = UniPoly::new(vec!(-3, 2, -4, 1));
        let lin = UniPoly::new(vec!(2,1));
        
        let res = UniPoly::new(vec!(14, -6, 1));
        let rem = UniPoly::monomial(-31, 0);
        assert_eq!(cubic.div(&lin), (res, rem)); 
    }
}