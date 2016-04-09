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
    /// # Example
    /// ```
    /// use array_tool::string::Justify;
    ///
    /// "asd asdf asd".justify_line(14);
    /// ```
    ///
    /// # Output
    /// ```text
    /// "asd  asdf  asd"
    /// ```
    fn justify_line(&self, width: usize) -> String;
  }

  impl Justify for &'static str {
    fn justify_line(&self, width: usize) -> String {
      let trimmed = self.trim() ;
      let len = trimmed.len();
      if len >= width { return self.to_string(); };
      let difference = width - len;
      let iter = trimmed.split_whitespace();
      let spaces = iter.count() - 1;
      let mut iter = trimmed.split_whitespace().peekable();
      if spaces == 0 { return self.to_string(); }
      let mut obj = String::with_capacity(width);
      
      let div = difference / spaces;
      let mut remainder = difference % spaces;
      
      loop {
        match iter.next() {
          Some(x) => {
            obj.push_str( x );
            let val = if remainder > 0 {
              remainder = remainder - 1;
              div + 1
            } else { div };
            for _ in 0..val+1 {
              match &iter.peek() { // Don't add spaces if last word
                &Some(_) => {
                  obj.push_str( " " );
                },
                &None => {},
              }
            }
          },
          None => { break },
        }
      }
      obj
    }
  }

  /// Substitute string character at each marker given
  pub trait SubstMarks {
    /// # Example
    /// ```
    /// use array_tool::string::SubstMarks;
    ///
    /// "asdf asdf asdf".subst_marks(vec![0,5,8], "Z");
    /// ```
    ///
    /// # Output
    /// ```text
    /// "Zsdf ZsdZ asdf"
    /// ```
    fn subst_marks(&self, marks: Vec<usize>, chr: &'static str) -> String;
  }
  impl SubstMarks for &'static str {
    fn subst_marks(&self, marks: Vec<usize>, chr: &'static str) -> String {
      let mut output = String::with_capacity(self.len());
      for x in 0..self.len() {
        if marks.contains(&x) {
          output.push_str(chr)
        } else {
          output.push(self.chars().nth(x).unwrap())
        }
      }
      output
    }
  }

  /// Word wrapping
  pub trait WordWrap {
    ///
    /// # Example
    /// ```
    /// use array_tool::string::WordWrap;
    ///
    /// "asd asdf asd".word_wrap(8);
    /// ```
    ///
    /// # Output
    /// ```text
    /// "asd asdf\nasd"
    /// ```
    fn word_wrap(&self, width: usize) -> String;
  }
  impl WordWrap for &'static str {
    #![allow(unsafe_code)]
    fn word_wrap(&self, width: usize) -> String {
      let mut markers = vec![];
      fn wordwrap(t: &'static str, chunk: usize, offset: usize, mrkrs: &mut Vec<usize>) -> String {
        let mark = unsafe {
          t.slice_unchecked(offset,offset+chunk)
        }.rfind(" ");
        match mark {
          Some(x) => {
            if offset+chunk < t.len() {
              mrkrs.push(offset + x)
            };
            wordwrap(t, chunk, offset+x+1, mrkrs)
          },
          None => {
            use string::SubstMarks;

            return t.subst_marks(mrkrs.to_vec(), "\n")
          },
        }
      };
      wordwrap(self, width, 0, &mut markers)
    }
  }
}
