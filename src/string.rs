// Copyright 2015-2016 Daniel P. Clark & array_tool Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// A string is a collection so we should have more methods for handling strings. 
pub mod string {
  /// Justify - expand line to given width
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

  /// Substitute string character for each index given
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
          output.push(self[x..x+1].chars().next().unwrap())
        }
      }
      output
    }
  }

  /// After whitespace
  pub trait AfterWhitespace {
    /// Given offset method will seek from there to end of string to find the first
    /// non white space.  Resulting value is counted from offset.
    ///
    /// # Example
    /// ```
    /// use array_tool::string::AfterWhitespace;
    ///
    /// assert_eq!(
    ///   "asdf           asdf asdf".seek_end_of_whitespace(6),
    ///   Some(9)
    /// );
    /// ```
    fn seek_end_of_whitespace(&self, offset: usize) -> Option<usize>;
  }
  impl AfterWhitespace for &'static str {
    fn seek_end_of_whitespace(&self, offset: usize) -> Option<usize> {
      if self.len() < offset { return None; };
      let mut seeker = self[offset..self.len()].chars();
      let mut val = None;
      let mut indx = 0;
      loop {
        match seeker.next() {
          Some(x) => {
            if x.ne(&" ".chars().next().unwrap()) {
              val = Some(indx);
              break;
            }
            indx += 1;
          },
          None => { break; },
        }
      }
      val
    }
  }

  /// Word wrapping
  pub trait WordWrap {
    ///  White space is treated as valid content and new lines will only be swapped in for
    ///  the last white space character at the end of the given width.  White space may reach beyond
    ///  the width you've provided.  You will need to trim end of lines in your own output (e.g.
    ///  splitting string at each new line and printing the line with trim_right).  Or just trust
    ///  that lines that are beyond the width are just white space and only print the width -
    ///  ignoring tailing white space.
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
  // No need to worry about character encoding since we're only checking for the
  // space and new line characters.
  impl WordWrap for &'static str {
    fn word_wrap(&self, width: usize) -> String {
      let mut markers = vec![];
      fn wordwrap(t: &'static str, chunk: usize, offset: usize, mrkrs: &mut Vec<usize>) -> String {
        match t[offset..*vec![offset+chunk,t.len()].iter().min().unwrap()].rfind("\n") {
          None => {
            match t[offset..*vec![offset+chunk,t.len()].iter().min().unwrap()].rfind(" ") {
              Some(x) => {
                let mut eows = x; // end of white space
                if offset+chunk < t.len() { // check if white space continues
                  match t.seek_end_of_whitespace(offset+x) {
                    Some(a) => {
                      if a.ne(&0) {
                        eows = x+a-1;
                      }
                    },
                    None => {},
                  }
                }
                if offset+chunk < t.len() { // safe to seek ahead by 1 or not end of string
                  if !["\n".chars().next().unwrap(), " ".chars().next().unwrap()].contains(
                    &t[offset+eows+1..offset+eows+2].chars().next().unwrap()
                  ) {
                    mrkrs.push(offset+eows)
                  }
                };
                wordwrap(t, chunk, offset+eows+1, mrkrs)
              },
              None => { 
                if offset+chunk < t.len() { // String may continue
                  wordwrap(t, chunk, offset+1, mrkrs) // Recurse + 1 until next space
                } else {
                  use string::SubstMarks;

                  return t.subst_marks(mrkrs.to_vec(), "\n")
                }
              },
            }
          },
          Some(x) => {
            wordwrap(t, chunk, offset+x+1, mrkrs)
          },
        }
      };
      wordwrap(self, width+1, 0, &mut markers)
    }
  }
}
