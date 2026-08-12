//! Semiring

use std::collections::HashMap;
use std::fmt::Debug;

use itertools::Itertools;

/// Semiring.
///
/// Consult <https://en.wikipedia.org/wiki/Semiring>.
pub trait Semiring: Debug + Clone + PartialEq {
    /// Additive identity.
    fn zero() -> Self;
    /// Multiplicative identity.
    fn one() -> Self;
    /// Addition operation.
    fn add(&self, rhs: &Self) -> Self;
    /// Multiplication operation.
    fn mul(&self, rhs: &Self) -> Self;
}

/// Converts integer to semiring value.
pub fn from_usize<T: Semiring>(value: usize) -> T {
    let mut result = T::zero();
    let one = T::one();

    for _ in 0..value {
        result = T::add(&result, &one);
    }

    result
}

impl Semiring for u64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn add(&self, rhs: &Self) -> Self {
        *self + *rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        *self * *rhs
    }
}

impl Semiring for i64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn add(&self, rhs: &Self) -> Self {
        *self + *rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        *self * *rhs
    }
}

impl Semiring for f64 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn add(&self, rhs: &Self) -> Self {
        *self + *rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        *self * *rhs
    }
}

/// Polynomials with coefficient in `C`.
///
/// For example, polynomial `x^2 + 5x + 6` is represented in `Polynomial<u64>` as follows:
///
/// ```ignore
/// Polynomial {
///     coefficients: {
///         2: 1,
///         1: 5,
///         0: 6,
///     },
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial<C: Semiring> {
    coefficients: HashMap<u64, C>,
}

impl<C: Semiring> Semiring for Polynomial<C> {
    fn zero() -> Self {
        Self {
            coefficients: HashMap::new(),
        }
    }

    fn one() -> Self {
        Self::term(C::one(), 0)
    }

    fn add(&self, rhs: &Self) -> Self {
        let mut result = self.coefficients.clone();

        for (&exp, coeff) in &rhs.coefficients {
            add_coeff(&mut result, exp, coeff.clone());
        }

        result.retain(|_, c| c != &C::zero());

        Self {
            coefficients: result,
        }
    }

    fn mul(&self, rhs: &Self) -> Self {
        let mut result = HashMap::new();

        for (&exp1, coeff1) in &self.coefficients {
            for (&exp2, coeff2) in &rhs.coefficients {
                add_coeff(&mut result, exp1 + exp2, coeff1.mul(coeff2));
            }
        }

        result.retain(|_, c| c != &C::zero());

        Self {
            coefficients: result,
        }
    }
}

impl<C: Semiring> Polynomial<C> {
    /// Constructs polynomial `x`.
    pub fn x() -> Self {
        Self::term(C::one(), 1)
    }

    /// Evaluates the polynomial with the given value.
    pub fn eval(&self, value: C) -> C {
        let mut result = C::zero();
        for (&exp, coeff) in &self.coefficients {
            let term_val = coeff.mul(&pow(&value, exp));
            result = result.add(&term_val);
        }
        result
    }

    /// Constructs polynomial `ax^n`.
    pub fn term(a: C, n: u64) -> Self {
        if a == C::zero() {
            Self::zero()
        } else {
            Self {
                coefficients: HashMap::from([(n, a)]),
            }
        }
    }
}

impl<C: Semiring> From<C> for Polynomial<C> {
    fn from(value: C) -> Self {
        Self::term(value, 0)
    }
}

/// Given a string `s`, parse it into a `Polynomial<C>`.
/// You may assume that `s` follows the criteria below.
/// Therefore, you do not have to return `Err`.
///
/// Assumptions:
/// - Each term is separated by ` + `.
/// - Each term is one of the following form: `a`, `x`, `ax`, `x^n`, and `ax^n`, where `a` is a
///   `usize` number and `n` is a `u64` number. This `a` should then be converted to a `C` type.
/// - In `a`, it is guaranteed that `a >= 1`.
/// - In `ax` and `ax^n`, it is guaranteed that `a >= 2`.
/// - In `x^n` and `ax^n`, it is guaranteed that `n >= 2`.
/// - All terms have unique degrees.
///
/// Consult `assignment06/grade.rs` for example valid strings.
///
/// Hint: `.split`, `.parse`, and `Polynomial::term`
fn add_coeff<C: Semiring>(map: &mut HashMap<u64, C>, exp: u64, coeff: C) {
    let _ = map
        .entry(exp)
        .and_modify(|c| *c = c.add(&coeff))
        .or_insert(coeff);
}

fn pow<C: Semiring>(base: &C, exp: u64) -> C {
    let mut result = C::one();
    for _ in 0..exp {
        result = result.mul(base);
    }
    result
}

fn parse_coeff<C: Semiring>(coeff_str: &str) -> C {
    let a_val: usize = coeff_str.parse().unwrap_or(1);
    from_usize(a_val)
}

fn parse_degree(exp_str: &str) -> u64 {
    exp_str
        .strip_prefix('^')
        .and_then(|e| e.parse().ok())
        .unwrap_or(1)
}

fn parse_term<C: Semiring>(term: &str) -> (C, u64) {
    if let Some((coeff_str, exp_str)) = term.split_once('x') {
        (parse_coeff(coeff_str), parse_degree(exp_str))
    } else {
        let constant: usize = term.parse().unwrap();
        (from_usize(constant), 0)
    }
}

impl<C: Semiring> std::str::FromStr for Polynomial<C> {
    type Err = (); // Ignore this for now...

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self::zero();
        for term_str in s.split(" + ") {
            let (coeff, degree) = parse_term(term_str);
            result = result.add(&Polynomial::term(coeff, degree));
        }
        Ok(result)
    }
}
