pub mod vec {
  pub trait Uniq {
    fn uniq(&self, other: Self) -> Self;
    fn unique(&self) -> Self;
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

  pub trait Empty {
    //#[deprecated(since = "0.2.3", reason = "already implemented with is_empty()")]
    fn empty(&self)-> bool;
  }

  impl<T: PartialEq> Empty for Vec<T> {
    //#[deprecated(since = "0.2.3", reason = "already implemented with is_empty()")]
    fn empty(&self) -> bool {
      self.len() == 0
    }
  }

  pub trait Shift<T> {
    fn shift(&mut self) -> T;
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

  pub trait Intersect {
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

  pub trait Join {
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

  pub trait Times {
    fn times(&self, qty: i32) -> Self;
  }

  impl<T: Clone> Times for Vec<T> {
    fn times(&self, qty: i32) -> Vec<T> {
      let mut out = vec![self[0].clone();self.len()*(qty as usize)];
      let mut cycle = self.iter().cycle();
      for x in 0..self.len()*(qty as usize) {
        out[x] = cycle.next().unwrap().clone();
      }
      out
    }
  }
  
  pub trait Union {
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

pub fn uniques<T: PartialEq + Clone>(a: Vec<T>, b: Vec<T>) -> Vec<Vec<T>> {
  use self::vec::Uniq;
  vec![a.uniq(b.clone()), b.uniq(a)]
}

