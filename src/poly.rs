use itertools::{
    Itertools,
    EitherOrBoth::*,
    iproduct,
};

use num_traits::Num;

struct UniPoly<T: Num> {
    coeffs: Vec<T>,
}


impl<T: Num + std::clone::Clone> UniPoly<T> {
    
    // Implementation of polynomial in a single variable, an element of k[x].

    fn new(coeffs: Vec<T>) -> UniPoly<T> {
        let mut res: Vec<T> = Vec::new();
        let reved: Vec<T> = coeffs.iter().rev().skip_while(|x| **x == T::zero()).cloned().collect::<Vec<i64>>();
        UniPoly{coeffs:reved.iter().rev().cloned().collect()}
    }

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

    fn neg(&self) -> UniPoly<T> {
        UniPoly::new(self.coeffs.iter().map(|&x| T::zero() - x ).collect())
    }

    fn mult(&self, other: &UniPoly<T>) -> UniPoly<T> {
        let len = self.coeffs.len() * other.coeffs.len();
        let mut res = vec![T::zero(); len];
        for ((i, exp_1), (j, exp_2)) in iproduct!(self.coeffs.iter().enumerate(), other.coeffs.iter().enumerate()) {
            dbg!("{}x^{} * {}x^{} = {}x^{}", exp_1, i, exp_2, j, *exp_1 * *exp_2, i + j);
            res[i + j] = res[i + j] + (*exp_1 * *exp_2);
        }
        UniPoly::new(res)
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
        UniPoly::monomial(*self.coeffs.last().unwrap(), self.degree())
    }

    fn div(&self, other:&UniPoly<T>) -> (UniPoly<T>, UniPoly<T>) {
        // Implements the division algorithm in k[x]. 
        // Returns (q, r) where self = f * other + r
        // In the book - self is f, other is g.
        let mut q: &UniPoly<T>; // Result
        let mut r: &UniPoly<T> = &self; // Remainder
        let ltg = other.lead_term();
        while (r.coeffs != vec!(T::zero())) & (r.lead_term().degree() > other.lead_term().degree()) {
            q = &q.add(r.lead_term().div_mono(&ltg));
            r = &r.add(r.lead_term().div_mono(&ltg).mult(other).neg())
        }

        (*q, *r)

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
        assert_eq!(UniPoly::new(vec!(0,2,3,5,0,0,6,0,0)).coeffs, vec!(0,2,3,5,0,0,6));
    }
}