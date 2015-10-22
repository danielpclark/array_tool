pub fn uniques<T: std::cmp::PartialEq<T> + std::clone::Clone>(a: Vec<T>, b: Vec<T>) -> Vec<Vec<T>> {
  let mut first_uniq = vec![];

  for x in &a {
    let mut unique = true;
    for y in &b {
      if x == y {
        unique = false
      }
    };
    if unique {
      first_uniq.push(x.clone())
    }
  };

  let mut second_uniq = vec![];

  for x in &b {
    let mut unique = true;
    for y in &a {
      if x == y {
        unique = false
      }
    };
    if unique {
      second_uniq.push(x.clone())
    }
  };
  
  vec![first_uniq, second_uniq]
}
