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

//! # Array Tool
//!
//! is a collection of powerful methods for working with collections.
//! Some of the most common methods you would use on Arrays made available
//! on Vectors. Polymorphic implementations for handling most of your use cases.
//!
//! In your rust files where you plan to use it put this at the top
//!
//! ```
//! extern crate array_tool;
//! ```
//!
//! And if you plan to use all of the Vector helper methods available:
//!
//! ```
//! use array_tool::vec::*;
//! ```
//!
//! This crate is not limited to just Vector methods and has some helpful
//! string methods as well.

include!("vec.rs");
include!("string.rs");
include!("core.rs");
