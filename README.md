# array_tool
Array helpers for Rust

Add the following to your Cargo.toml file
```
[dependencies]
array_tool = "0.1.0"
```

And in your rust files where you plan to use it put this at the top
```rust
extern crate array_tool;
```

###Example usage

```rust
assert_eq!(
  array_tool::uniques(vec![1,2,3,4,5], vec![2,5,6,7,8]), // input
  vec![vec![1,3,4], vec![6,7,8]]                         // return value  
)
```

##Future plans

I plan on implementing Array like methods for union, difference, and uniq.  Also I plan to add
methods on to the basic Array like collection types.  So methods would be available to use on &[] and Vec.

###Contribute

Feel free to add your own methods here!  And be sure to include an integration test!
