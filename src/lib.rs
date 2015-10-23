pub trait Uniq{
  fn uniq(&self, other: Self) -> Self;
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
}

pub fn uniques<T: PartialEq + Clone>(a: Vec<T>, b: Vec<T>) -> Vec<Vec<T>> {
  vec![a.uniq(b.clone()), b.uniq(a)]
}
