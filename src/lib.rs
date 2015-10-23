pub mod vec {
  pub trait Uniq {
    fn uniq(&self, other: Self) -> Self;
    fn unique(&self) -> Self;
    fn is_unique(&self) -> bool;
  }

  impl<T: Clone + PartialEq> Uniq for Vec<T> {
    fn uniq(&self, other: Vec<T>) -> Vec<T> {

      let mut uniq_val = vec![];

      for x in self.to_vec() {
        let mut unique = true;
        for y in other.to_vec() {
          if x == y {
            unique = false
          }
        };
        if unique {
          uniq_val.push(x.clone())
        }
      };
      uniq_val
    }

    fn unique(&self) -> Vec<T> {
      let mut a = self.clone();
      for x in 0..a.len() {
        for y in x+1..a.len() {
          if a[x] == a[y] {
            a.remove(y);
            break;
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
    fn empty(&self)-> bool;
  }

  impl<T: PartialEq> Empty for Vec<T> {
    fn empty(&self) -> bool {
      self.len() == 0
    }
  }

}

pub fn uniques<T: PartialEq + Clone>(a: Vec<T>, b: Vec<T>) -> Vec<Vec<T>> {
  use self::vec::Uniq;
  vec![a.uniq(b.clone()), b.uniq(a)]
}

