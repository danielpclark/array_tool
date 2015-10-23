# array_tool
Array helpers for Rust

Add the following to your Cargo.toml file
```
[dependencies]
array_tool = "0.2.1"
```

And in your rust files where you plan to use it put this at the top
```rust
extern crate array_tool;
```

And if you plan to use all of the Vector helper methods available you may do
```rust
use array_tool::vec::*;
```

###Usage

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

use array_tool::vec::Empty;
fn empty(&self) -> bool
  //  let mut x = vec![1];
  //  assert_eq!(x.empty(), false);
  //  x.pop();
  //  assert_eq!(x.empty(), true);

use arra_tool::vec::Shift;
fn unshift(&mut self, other: T)     // no return value, modifies &mut self directly
  //  let mut x = vec![1,2,3];
  //  x.unshift(0);
  //  assert_eq!(x, vec![0,1,2,3]);
fn shift(&mut self)                 // no return value, modifies &mut self directly
  //  let mut x = vec![0,1,2,3];
  //  x.shift();
  //  assert_eq!(x, vec![1,2,3]);

use array_tool::vec::Intersect;
fn intersect(&self, other: Vec<T>) -> Vec<T>
  //  vec![1,1,3,5].intersect(vec![1,2,3]) // input
  //  vec![1,3])                           // return value
```

##Future plans

I plan on implementing Array like methods for union, difference, and uniq.  Also I plan to add
methods on to the basic Array like collection types.  So methods would be available to use on &[] and Vec.

###Contribute

Feel free to add your own methods here!  And be sure to include an integration test!