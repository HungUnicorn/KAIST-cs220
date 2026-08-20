//! Small exercises.

use std::collections::HashMap;

use itertools::Itertools;

/// Returns whether the given sequence is a fibonacci sequence starts from the given sequence's
/// first two terms.
///
/// Returns `true` if the length of sequence is less or equal than 2.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(is_fibonacci([1, 1, 2, 3, 5, 8, 13].into_iter()), true);
/// assert_eq!(is_fibonacci([1, 1, 2, 3, 5, 8, 14].into_iter()), false);
/// ```
pub fn is_fibonacci(mut inner: impl Iterator<Item = i64>) -> bool {
    let (mut a, mut b) = match (inner.next(), inner.next()) {
        (Some(x), Some(y)) => (x, y),
        _ => return true,
    };

    for item in inner {
        if item != a + b {
            return false;
        } else {
            a = b;
            b = item;
        }
    }
    true
}

/// Returns the sum of `f(v)` for all element `v` the given array.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(sigma([1, 2].into_iter(), |x| x + 2), 7);
/// assert_eq!(sigma([1, 2].into_iter(), |x| x * 4), 12);
/// ```
pub fn sigma<T, F: Fn(T) -> i64>(inner: impl Iterator<Item = T>, f: F) -> i64 {
    inner.map(f).sum()
}

/// Alternate elements from three iterators until they have run out.
///
/// You can assume that the number of elements of three iterators are same.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(
///     interleave3([1, 2].into_iter(), [3, 4].into_iter(), [5, 6].into_iter()),
///     vec![1, 3, 5, 2, 4, 6]
/// );
/// ```
pub fn interleave3<T>(
    list1: impl Iterator<Item = T>,
    list2: impl Iterator<Item = T>,
    list3: impl Iterator<Item = T>,
) -> Vec<T> {
    itertools::izip!(list1, list2, list3)
        .flat_map(|(a, b, c)| [a, b, c])
        .collect()
}

/// Alternate elements from array of n iterators until they have run out.
///
/// You can assume that the number of elements of iterators are same.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(
///     interleave_n(&mut [[1, 2].into_iter(), [3, 4].into_iter(), [5, 6].into_iter()]),
///     vec![1, 3, 5, 2, 4, 6]
/// );
/// ```
pub fn interleave_n<T, const N: usize>(
    mut iters: [impl Iterator<Item = T>; N],
) -> impl Iterator<Item = T> {
    let mut idx = 0;
    std::iter::from_fn(move || {
        if N == 0 {
            return None;
        }

        let current = &mut iters[idx];
        idx = (idx + 1) % N;
        current.next()
    })
}

/// Returns mean of k smallest value's mean.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(
///     k_smallest_mean(vec![1, 3, 2].into_iter(), 2),
///     ((1 + 2) as f64 / 2.0)
/// );
/// assert_eq!(
///     k_smallest_mean(vec![7, 5, 3, 6].into_iter(), 3),
///     ((3 + 5 + 6) as f64 / 3.0)
/// );
/// ```
pub fn k_smallest_mean(inner: impl Iterator<Item = i64>, k: usize) -> f64 {
    inner.sorted().take(k).sum::<i64>() as f64 / k as f64
}

/// Returns mean for each class.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(
///     calculate_mean(
///         [
///             ("CS100".to_string(), 60),
///             ("CS200".to_string(), 60),
///             ("CS200".to_string(), 80),
///             ("CS300".to_string(), 100),
///         ]
///         .into_iter()
///     ),
///     [
///         ("CS100".to_string(), 60.0),
///         ("CS200".to_string(), 70.0),
///         ("CS300".to_string(), 100.0)
///     ]
///     .into_iter()
///     .collect()
/// );
/// ```
pub fn calculate_mean(inner: impl Iterator<Item = (String, i64)>) -> HashMap<String, f64> {
    let mut totals = HashMap::new();
    for (key, value) in inner {
        let entry = totals.entry(key).or_insert((0.0, 0));
        entry.0 += value as f64;
        entry.1 += 1;
    }

    totals
        .into_iter()
        .map(|(k, (sum, count))| (k, sum / count as f64))
        .collect()
}

/// Among the cartesian product of input vectors, return the number of sets whose sum equals `n`.
///
/// # Example
///
/// The cartesian product of [1, 2, 3] and [2, 3] are:
///     [1, 2], [1, 3], [2, 2], [2, 3], [3, 2], [3, 3].
///
/// Among these sets, the number of sets whose sum is 4 is 2, which is [1, 3] and [2, 2].
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 3), 1);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 4), 2);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 5), 2);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 6), 1);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 2), 0);
/// ```
pub fn sum_is_n(inner: Vec<Vec<i64>>, n: i64) -> usize {
    inner
        .into_iter()
        .multi_cartesian_product()
        .map(|x| x.iter().sum::<i64>())
        .filter(|&x| x == n)
        .count()
}

/// Returns a new vector that contains the item that appears `n` times in the input vector in
/// increasing order.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(find_count_n(vec![1, 2], 1), vec![1, 2]);
/// assert_eq!(find_count_n(vec![1, 3, 3], 1), vec![1]);
/// assert_eq!(find_count_n(vec![1, 3, 3], 2), vec![3]);
/// assert_eq!(find_count_n(vec![1, 2, 3, 4, 4], 1), vec![1, 2, 3]);
/// ```
pub fn find_count_n(inner: Vec<usize>, n: usize) -> Vec<usize> {
    inner
        .iter()
        .counts()
        .into_iter()
        .filter(|&(_, count)| count == n)
        .map(|(key, _)| *key)
        .sorted()
        .collect()
}

/// Return the position of the median element in the vector.
///
/// For a data set `x` of `n` elements, the median can be defined as follows:
///
/// - If `n` is odd, the median is `(n+1)/2`-th smallest element of `x`.
/// - If `n` is even, the median is `(n/2)+1`-th smallest element of `x`.
///
/// Please following these rules:
///
/// - If the list is empty, returns `None`.
/// - If several elements are equally median, the position of the first of them is returned.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(position_median(vec![1, 3, 3, 6, 7, 8, 9]), Some(3));
/// assert_eq!(position_median(vec![1, 3, 3, 3]), Some(1));
/// ```
pub fn position_median<T: Ord>(inner: Vec<T>) -> Option<usize> {
    if inner.is_empty() {
        return None;
    }

    let mut sorted: Vec<&T> = inner.iter().collect();
    sorted.sort();

    let median_val = sorted[sorted.len() / 2];
    inner.iter().position(|x| x == median_val)
}

/// Returns the sum of all elements in a two-dimensional array.
///
/// # Example
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(
///     two_dimensional_sum([[1, 2, 3].into_iter(), [4, 5, 6].into_iter()].into_iter()),
///     21
/// );
/// ```
pub fn two_dimensional_sum(inner: impl Iterator<Item = impl Iterator<Item = i64>>) -> i64 {
    inner.flatten().sum()
}

/// Returns whether the given string is palindrome or not.
///
/// A palindrome is a word, number, phrase, or other sequence of characters which reads the same
/// backward as forward.
///
/// We consider the empty string as a palindrome.
///
/// Consult <https://en.wikipedia.org/wiki/Palindrome>.
pub fn is_palindrome(s: String) -> bool {
    s.chars().eq(s.chars().rev())
}
