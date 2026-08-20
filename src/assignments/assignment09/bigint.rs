//! Big integer with infinite precision.

use std::fmt;
use std::iter::zip;
use std::ops::*;

/// An signed integer with infinite precision implemented with an "carrier" vector of `u32`s.
///
/// The vector is interpreted as a base 2^(32 * (len(carrier) - 1)) integer, where negative
/// integers are represented in their [2's complement form](https://en.wikipedia.org/wiki/Two%27s_complement).
///
/// For example, the vector `vec![44,345,3]` represents the integer
/// `44 * (2^32)^2 + 345 * (2^32) + 3`,
/// and the vector `vec![u32::MAX - 5, u32::MAX - 7]` represents the integer
/// `- (5 * 2^32 + 8)`
///
/// You will implement the `Add` and `Sub` trait for this type.
///
/// Unlike standard fix-sized intergers in Rust where overflow will panic, the carrier is extended
/// to save the overflowed bit. On the contrary, if the precision is too much (e.g, vec![0,0] is
/// used to represent 0, where `vec![0]` is sufficent), the carrier is truncated.
///
/// See [this section](https://en.wikipedia.org/wiki/Two%27s_complement#Arithmetic_operations) for a rouge guide on implementation,
/// while keeping in mind that the carrier should be extended to deal with overflow.
///
/// The `sign_extension()`, `two_complement()`, and `truncate()` are non-mandatory helper methods.
///
/// For testing and debugging purposes, the `Display` trait is implemented for you, which shows the
/// integer in hexadecimal form.
#[derive(Debug, Clone)]
pub struct BigInt {
    /// The carrier for `BigInt`.
    ///
    /// Note that the carrier should always be non-empty.
    pub carrier: Vec<u32>,
}

impl BigInt {
    /// Create a new `BigInt` from a `usize`.
    pub fn new(n: u32) -> Self {
        Self { carrier: vec![n] }
    }

    /// Creates a new `BigInt` from a `Vec<u32>`.
    ///
    /// # Panic
    ///
    /// Panics if `carrier` is empty.
    pub fn new_large(carrier: Vec<u32>) -> Self {
        assert!(!carrier.is_empty());
        Self { carrier }.truncate()
    }
}

const SIGN_MASK: u32 = 1 << 31;

impl BigInt {
    /// Extend `self` to `len` bits.
    fn sign_extension(&self, len: usize) -> Self {
        if len <= self.carrier.len() {
            return self.clone();
        }

        let diff = len - self.carrier.len();
        let pad_val = if (self.carrier[0] & SIGN_MASK) != 0 {
            u32::MAX
        } else {
            0
        };

        let mut new_carrier = vec![pad_val; diff];
        new_carrier.extend_from_slice(&self.carrier);

        Self {
            carrier: new_carrier,
        }
    }

    /// Compute the two's complement of `self`.
    fn two_complement(&self) -> Self {
        let inverted: Vec<u32> = self.carrier.iter().map(|&x| !x).collect();
        let inverted_bigint = BigInt::new_large(inverted);
        inverted_bigint + BigInt::new(1)
    }

    /// Truncate a `BigInt` to the minimum length.
    fn truncate(&self) -> Self {
        let mut slice = &self.carrier[..];

        while let [first, second, ..] = slice {
            let redundant_zero = *first == 0 && (second & SIGN_MASK) == 0;
            let redundant_neg = *first == u32::MAX && (second & SIGN_MASK) != 0;

            if redundant_zero || redundant_neg {
                slice = &slice[1..];
            } else {
                break;
            }
        }

        Self {
            carrier: slice.to_vec(),
        }
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let target_len = self.carrier.len().max(rhs.carrier.len()) + 1;
        let s_ext = self.sign_extension(target_len).carrier;
        let r_ext = rhs.sign_extension(target_len).carrier;

        let mut res = vec![0u32; target_len];
        let mut carry = 0;

        for i in (0..target_len).rev() {
            let sum = (s_ext[i] as u64) + (r_ext[i] as u64) + carry;

            res[i] = sum as u32;

            carry = sum >> 32
        }

        BigInt::new_large(res)
    }
}

impl Sub for BigInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs.two_complement())
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Hex formatting so that each u32 can be formatted independently.
        for i in self.carrier.iter() {
            write!(f, "{:08x}", i)?;
        }
        Ok(())
    }
}
