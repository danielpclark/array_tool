#![deny(missing_docs,trivial_casts,trivial_numeric_casts,
        missing_debug_implementations, missing_copy_implementations,
        unsafe_code,unstable_features,unused_import_braces,unused_qualifications)
]

// Copyright 2015-2016 Daniel P. Clark & array_tool Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Array Tool
//!
//! Array Tool is a collection of powerful methods for working with collections.

/// Array Tool provides many useful methods for vectors
pub mod vec {

  /// Several different methods for getting, or evaluating, uniqueness.
  pub trait Uniq {
    /// `uniq` returns a vector of unique values within itself as compared to
    /// the other vector which is provided as a input parameter.
    ///
    /// # Example
    /// ```
    /// use array_tool::vec::Uniq;
    ///
    /// vec![1,2,3,4,5,6].uniq( vec![1,2,5,7,9] );
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![3,4,6]
    /// ```
    fn uniq(&self, other: Self) -> Self;

    /// `unique` removes duplicates from within the vector and returns Self.
    ///
    /// # Example
    /// ```
    /// use array_tool::vec::Uniq;
    ///
    /// vec![1,2,1,3,2,3,4,5,6].unique();
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![1,2,3,4,5,6]
    /// ```
    fn unique(&self) -> Self;
    /// `is_unique` returns boolean value on whether all values within
    /// Self are unique.
    ///
    /// # Example
    /// ```
    /// use array_tool::vec::Uniq;
    ///
    /// vec![1,2,1,3,4,3,4,5,6].is_unique();
    /// ```
    ///
    /// # Output
    /// ```text
    /// false
    /// ```
    fn is_unique(&self) -> bool;
  }

  impl<T: Clone + PartialEq> Uniq for Vec<T> {
    fn uniq(&self, other: Vec<T>) -> Vec<T> {
      let mut out = self.unique();
      for x in other.unique() {
        for y in (0..out.len()).rev() {
          if x == out[y] {
            out.remove(y);
          }
        }
      }
      out
    }
    fn unique(&self) -> Vec<T> {
      let mut a = self.clone();
      for x in (0..a.len()).rev() {
        for y in (x+1..a.len()).rev() {
          if a[x] == a[y] {
            a.remove(y);
          }
        }
      }
      a
    }
    fn is_unique(&self) -> bool {
      let mut a = true;
      for x in 0..self.len() {
        for y in x+1..self.len() {
          if self[x] == self[y] {
            a = false;
            break;
          }
        }
      }
      a
    }
  }

  /// Removes, or Adds, the first element of self.
  pub trait Shift<T> {
    /// Removes and returns the first item from the vector
    ///
    /// # Example
    /// ```
    /// use array_tool::vec::Shift;
    ///
    /// let mut x = vec![0,1,2,3];
    /// assert_eq!(x.shift(), 0);
    /// assert_eq!(x, vec![1,2,3]);
    /// ```
    fn shift(&mut self) -> T;
    /// Insert item at the beginning of the vector.  No return value.
    ///
    /// # Example
    /// ```
    /// use array_tool::vec::Shift;
    ///
    /// let mut x = vec![1,2,3];
    /// x.unshift(0);
    /// assert_eq!(x, vec![0,1,2,3]);
    /// ```
    fn unshift(&mut self, other: T);
  }
  impl<T: PartialEq> Shift<T> for Vec<T> {
    fn shift(&mut self) -> T {
      self.remove(0)
    }
    fn unshift(&mut self, other: T) {
      &self.insert(0, other);
    }
  }

  /// Set Intersection — Returns a new array containing elements common to the two arrays,
  /// excluding any duplicates. The order is preserved from the original array.
  pub trait Intersect {
    /// # Example
    /// ```
    /// use array_tool::vec::Intersect;
    ///
    /// vec![1,1,3,5].intersect(vec![1,2,3]);
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![1,3]
    /// ```
    fn intersect(&self, Self) -> Self;
  }
  impl<T: PartialEq + Clone> Intersect for Vec<T> {
    fn intersect(&self, other: Vec<T>) -> Vec<T> {
      let mut out = vec![];
      let a = self.unique();
      let length = other.len();
      for x in a {
        for y in 0..length {
          if x == other[y] {
            out.push(x);
            break;
          }
        }
      }
      out
    }
  }

  /// Join vector of ToString capable things to a String with given delimiter.
  pub trait Join {
    /// # Example
    /// ```
    /// use array_tool::vec::Join;
    ///
    /// vec![1,2,3].join(",");
    /// ```
    ///
    /// # Output
    /// ```text
    /// "1,2,3"
    /// ```
    fn join(&self, joiner: &'static str) -> String;
  }
  impl<T: ToString> Join for Vec<T> {
    fn join(&self, joiner: &'static str) -> String {
      let mut out = String::from("");
      for x in 0..self.len() {
        out.push_str(&self[x].to_string());
        if x < self.len()-1 {
          out.push_str(&joiner)
        }
      }
      out
    }
  }

  /// Expand and duplicate the vectors content `times` the integer given
  pub trait Times {
    /// # Example
    /// ```
    /// use array_tool::vec::Times;
    ///
    /// vec![1,2,3].times(3);
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![1,2,3,1,2,3,1,2,3]
    /// ```
    fn times(&self, qty: i32) -> Self;
  }
  impl<T: Clone> Times for Vec<T> {
    /// Expand and duplicate the vectors content `times` the integer given
    fn times(&self, qty: i32) -> Vec<T> {
      let mut out = vec![self[0].clone();self.len()*(qty as usize)];
      let mut cycle = self.iter().cycle();
      for x in 0..self.len()*(qty as usize) {
        out[x] = cycle.next().unwrap().clone();
      }
      out
    }
  }
  
  /// Create a `union` between two vectors.
  /// Returns a new vector by joining with other, excluding any duplicates and preserving
  /// the order from the original vector.
  pub trait Union {
    /// # Example
    /// ```
    /// use array_tool::vec::Union;
    ///
    /// vec!["a","b","c"].union(vec!["c","d","a"]);
    /// ```
    ///
    /// # Output
    /// ```text
    /// vec![ "a", "b", "c", "d" ]
    /// ```
    fn union(&self, other: Self) -> Self;
  }
  impl<T: PartialEq + Clone> Union for Vec<T> {
    fn union(&self, other: Vec<T>) -> Vec<T> {
      let mut stack = self.clone();
      for x in other { // don't use append method as it's destructive
        stack.push(x)
      }
      stack.unique()
    }
  }
}

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

