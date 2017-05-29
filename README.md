# array_tool
[![Build Status](https://travis-ci.org/danielpclark/array_tool.svg)](https://travis-ci.org/danielpclark/array_tool)
[![Documentation](https://img.shields.io/badge/docs-100%25-brightgreen.svg)](http://danielpclark.github.io/array_tool/index.html)
[![crates.io version](https://img.shields.io/crates/v/array_tool.svg)](https://crates.io/crates/array_tool)
[![License](https://img.shields.io/crates/l/array_tool.svg)]()

Array helpers for Rust.  Some of the most common methods you would
use on Arrays made available on Vectors.  Polymorphic implementations
for handling most of your use cases.


### Installation

Add the following to your Cargo.toml file
```
[dependencies]
array_tool = "0.3.7"
```

And in your rust files where you plan to use it put this at the top
```rust
extern crate array_tool;
```

And if you plan to use all of the Vector helper methods available you may do
```rust
use array_tool::vec::*;
```

This crate has helpful methods for strings as well.

### Usage

```rust
pub fn uniques<T: PartialEq + Clone>(a: Vec<T>, b: Vec<T>) -> Vec<Vec<T>>
  //  array_tool::uniques(vec![1,2,3,4,5], vec![2,5,6,7,8]), // input
  //  vec![vec![1,3,4], vec![6,7,8]]                         // return value

use array_tool::vec::Uniq;
fn uniq(&self, other: Vec<T>) -> Vec<T>
  //  vec![1,2,3,4,5,6].uniq( vec![1,2,5,7,9] ), // input
  //  vec![3,4,6]                                // return value
fn unique(&self) -> Vec<T>
  //  vec![1,2,1,3,2,3,4,5,6].unique(),          // input
  //  vec![1,2,3,4,5,6]                          // return value
fn is_unique(&self) -> bool
  //  vec![1,2,1,3,4,3,4,5,6].is_unique(),       // input
  //  false                                      // return value
  //  vec![1,2,3,4,5,6].is_unique(),             // input
  //  true                                       // return value

use array_tool::vec::Shift;
fn unshift(&mut self, other: T)     // no return value, modifies &mut self directly
  //  let mut x = vec![1,2,3];
  //  x.unshift(0);
  //  assert_eq!(x, vec![0,1,2,3]);
fn shift(&mut self) -> T
  //  let mut x = vec![0,1,2,3];
  //  assert_eq!(x.shift(), 0);
  //  assert_eq!(x, vec![1,2,3]);

use array_tool::vec::Intersect;
fn intersect(&self, other: Vec<T>) -> Vec<T>
  //  vec![1,1,3,5].intersect(vec![1,2,3]) // input
  //  vec![1,3]                            // return value
fn intersect_if<F: Fn(&T, &T) -> bool>(&self, other: Vec<T>, validator: F) -> Vec<T>
  //  vec!['a','a','c','e'].intersect_if(vec!['A','B','C'], |l, r| l.eq_ignore_ascii_case(r)) // input
  //  vec!['a','c']                                                                           // return value

use array_tool::vec::Join;
fn join(&self, joiner: &'static str) -> String
  //  vec![1,2,3].join(",")                // input
  //  "1,2,3"                              // return value

use array_tool::vec::Times;
fn times(&self, qty: i32) -> Vec<T>
  //  vec![1,2,3].times(3)                 // input
  //  vec![1,2,3,1,2,3,1,2,3]              // return value

use array_tool::vec::Union;
fn union(&self, other: Vec<T>) -> Vec<T>
  //  vec!["a","b","c"].union(vec!["c","d","a"])   // input
  //  vec![ "a", "b", "c", "d" ]                   // return value

use array_tool::string::ToGraphemeBytesIter;
fn grapheme_bytes_iter(&'a self) -> GraphemeBytesIter<'a>;
  //  let string = "a s—d féZ";
  //  let mut graphemes = string.grapheme_bytes_iter()
  //  graphemes.skip(3).next();            // input
  //  [226, 128, 148]                      // return value for emdash `—`

use array_tool::string::Justify;
fn justify_line(&self, width: usize) -> String;
  //  "asd as df asd".justify_line(16)     // input
  //  "asd  as  df  asd"                   // return value
  //  "asd as df asd".justify_line(18)     // input
  //  "asd   as   df  asd"                 // return value

use array_tool::string::SubstMarks;
fn subst_marks(&self, marks: Vec<usize>, chr: &'static str) -> String;
  //  "asdf asdf asdf".subst_marks(vec![0,5,8], "Z") // input
  //  "Zsdf ZsdZ asdf"                               // return value

use array_tool::string::WordWrap;
fn word_wrap(&self, width: usize) -> String;
  //  "01234 67 9 BC EFG IJ".word_wrap(6)  // input
  //  "01234\n67 9\nBC\nEFG IJ"            // return value

use array_tool::string::AfterWhitespace;
fn seek_end_of_whitespace(&self, offset: usize) -> Option<usize>;
  //  "asdf           asdf asdf".seek_end_of_whitespace(6) // input
  //  Some(9)                                              // return value
  //  "asdf".seek_end_of_whitespace(3)                     // input
  //  Some(0)                                              // return value
  //  "asdf           ".seek_end_of_whitespace(6)          // input
  //  None                                                 // return_value

```

## Future plans

Expect methods to become more polymorphic over time (same method implemented
for similar & compatible types).  I plan to implement many of the methods
available for Arrays in higher languages; such as Ruby. Expect regular updates.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
