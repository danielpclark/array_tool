// Copyright 2015-2016 Daniel P. Clark & array_tool Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// Get `uniques` from two vectors
///
/// # Example
/// ```
/// use array_tool::uniques;
///
/// uniques(vec![1,2,3,4,5], vec![2,5,6,7,8]);
/// ```
///
/// # Output
/// ```text
/// vec![vec![1,3,4], vec![6,7,8]]
/// ```
pub fn uniques<T: PartialEq + Clone>(a: Vec<T>, b: Vec<T>) -> Vec<Vec<T>> {
  use self::vec::Uniq;
  vec![a.uniq(b.clone()), b.uniq(a)]
}

