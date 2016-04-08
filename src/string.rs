// Copyright 2015-2016 Daniel P. Clark & array_tool Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// A string is a collection so we should have more methods for handling strings. 
pub mod string {
  /// Justified
  pub trait Justify {
    ///
    /// # Example
    /// ```
    /// ```
    ///
    /// # Output
    /// ```text
    /// ```
    fn justify_line(&self, width: usize) -> &Self;
  }

  impl Justify for &'static str {
    fn justify_line(&self, width: usize) -> &Self {
      let trimmed = self.trim() ;
      let mut len = trimmed.len();
      if len >= width { return self; };
      let difference = width - len;
      let mut iter = trimmed.split_whitespace();
      let spaces = iter.count() - 1;
      if spaces == 0 { return self; }
      let mut obj = String::with_capacity(width);
      
      let mut div = difference / spaces;
      let mut remainder = difference % spaces;

      loop {
        match iter.next() {
          Some(x) => {
            obj.push( x );
            let val = if remainder > 0 {
              remainder = remainder - 1;
              div + 1
            } else { div };
            for _ in 0..val {
              if iter.count() != 0 { // Don't add spaces if last word
                obj.push( " " );
              }
            }
          },
          None => { break },
        }
      }

      obj
    }
  }
}
