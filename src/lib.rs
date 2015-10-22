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

// 43% SLOWER implementation
//
//  pub fn experimental_uniques<T: std::cmp::PartialEq<T> + std::clone::Clone>(a: Vec<T>, b: Vec<T>) -> Vec<Vec<T>> {
//    let mut first_uniq = a.clone();
//    let mut second_uniq = b.clone();
//    
//    let mut x = first_uniq.len();
//  
//    'outer: loop {
//      x -= 1;
//      let mut y = second_uniq.len();
//      let mut removed = false;
//      'inner: loop {
//        y -= 1;
//        if first_uniq[x] == second_uniq[y] {
//          first_uniq.remove(x);
//          second_uniq.remove(y);
//          removed = true
//        }
//        if x == 0{
//          if y == 0{
//            break 'outer
//          }
//        }
//        else {
//          if y == 0{
//            break
//          }
//          if removed {
//            continue 'outer
//          }
//        }
//      }
//    }
//   
//    vec![first_uniq, second_uniq]
//  }
