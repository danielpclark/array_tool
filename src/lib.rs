pub fn uniques<T: std::cmp::PartialEq<T> + std::clone::Clone>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
  let mut uniq = vec![];

  for x in &a {
    let mut unique = true;
    for y in &b {
      if x == y {
        unique = false
      }
    };
    if unique {
      uniq.push(x.clone())
    }
  };

  for x in &b {
    let mut unique = true;
    for y in &a {
      if x == y {
        unique = false
      }
    };
    if unique {
      uniq.push(x.clone())
    }
  };
  
  uniq
}
