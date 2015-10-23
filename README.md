# array_tool
Array helpers for Rust

Add the following to your Cargo.toml file
```
[dependencies]
array_tool = "0.1.2"
```

And in your rust files where you plan to use it put this at the top
```rust
extern crate array_tool;
```

And if you plan to use all of the methods available you may do
```rust
use array_tool::*;
```

###Example usage

```rust
assert_eq!(
  array_tool::uniques(vec![1,2,3,4,5], vec![2,5,6,7,8]), // input
  vec![vec![1,3,4], vec![6,7,8]]                         // return value  
)

use array_tool::Uniq; // This adds the .uniq and .unique methods onto Vectors
assert_eq!(
  vec![1,2,3,4,5,6].uniq( vec![1,2,5,7,9] ), // input
  vec![3,4,6]                                // return value
);
assert_eq!(
  vec![1,2,1,3,2,3,4,5,6].unique(), // input
  vec![1,2,3,4,5,6]                 // return value
);

use array_tool::Empty;
let mut x = vec![1];
assert_eq!(x.empty(), false);
x.pop();
assert_eq!(x.empty(), true);
```

##Future plans

I plan on implementing Array like methods for union, difference, and uniq.  Also I plan to add
methods on to the basic Array like collection types.  So methods would be available to use on &[] and Vec.

###Contribute

Feel free to add your own methods here!  And be sure to include an integration test!
