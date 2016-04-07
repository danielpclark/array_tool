extern crate array_tool;

#[test]
fn it_implements_uniques() {
  assert_eq!(array_tool::uniques(vec![1,2,3,4,5,6],vec![1,2]), vec![vec![3,4,5,6],vec![]]);
  assert_eq!(array_tool::uniques(vec![1,2,3,4,5,6],vec![1,2,3,4]), vec![vec![5,6], vec![]]);
  assert_eq!(array_tool::uniques(vec![1,2,3],vec![1,2,3,4,5]), vec![vec![], vec![4,5]]);
  assert_eq!(array_tool::uniques(vec![1,2,9],vec![1,2,3,4,5]), vec![vec![9], vec![3,4,5]]);
}

#[test]
fn it_implements_individual_uniq_on_vec() {
  use array_tool::vec::Uniq;
  assert_eq!(vec![1,2,3,4,5,6].uniq(vec![1,2,5,7,9]),vec![3,4,6]);
  assert_eq!(vec![1,2,3,1,3,2,1,3,1,2,3,1,2,3,3,1,2,3,3,1,2,3,1,2,3,3,4,1,5,4,6].uniq(vec![3,5]),
             vec![1,2,4,6]
            );
}

#[test]
fn it_can_return_its_own_unique() {
  use array_tool::vec::Uniq;
  assert_eq!(vec![1,2,1,3,4,3,4,5,6].unique(),vec![1,2,3,4,5,6]);
  assert_eq!(vec![1,2,3,1,3,2,1,3,1,2,3,1,2,3,3,1,2,3,3,1,2,3,1,2,3,3,4,1,5,4,6].unique(),
             vec![1,2,3,4,5,6]
            );
}

#[test]
fn it_answers_about_uniqueness() {
  use array_tool::vec::Uniq;
  assert_eq!(vec![1,2,1,3,4,3,4,5,6].is_unique(), false);
  assert_eq!(vec![1,2,3,4,5,6].is_unique(), true);
}

#[test]
fn it_shifts() {
  use array_tool::vec::Shift;
  let mut x = vec![1,2,3];
  x.unshift(0);
  assert_eq!(x, vec![0,1,2,3]);
  assert_eq!(x.shift(), 0);
  assert_eq!(x, vec![1,2,3]);
}

#[test]
fn it_intersects() {
  use array_tool::vec::Intersect;
  assert_eq!(vec![1,1,3,5].intersect(vec![1,2,3]), vec![1,3])
}

#[test]
fn it_multiplies(){
  use array_tool::vec::Times;
  assert_eq!(vec![1,2,3].times(3), vec![1,2,3,1,2,3,1,2,3])
}

#[test]
fn it_joins(){
  use array_tool::vec::Join;
  assert_eq!(vec![1,2,3].join(","), "1,2,3")
}

#[test]
fn it_creates_union() {
  use array_tool::vec::Union;
  assert_eq!(vec!["a","b","c"].union(vec!["c","d","a"]), vec![ "a", "b", "c", "d" ]);
  assert_eq!(vec![1,2,3,1,3,2,1,3,1,2,3,1,2,3,3,1,2,3,3,1,2,3,1,2,3,3,4,1,4,6].union(vec![3,5,7,8,0]),
             vec![1,2,3,4,6,5,7,8,0]
            );
}

//#[test]
//fn it_iterates(){
//  use array_tool::vec::MovingSlice;
//}

//#[test]
//fn it_moves_over_slices() {
//  use array_tool::vec::MovingSlice;
//  // first item in downto returns self
//  assert_eq!(
//    vec!['A','2','3','4','5','6'].downto(3).next,
//    vec!['A','2','3','4','5','6']
//  );
//  // collect from down to returns all slices from current to minimum
//  assert_eq!(
//    vec!['2','3','4'].downto(1).collect(),
//    vec![vec!['2','3','4'],vec!['2','3'],vec!['3','4'],vec!['2'],vec!['3'],vec!['4']]
//  );
//  // collect up from min vecs to max
//  assert_eq!(
//    vec!['5','6','7'].upfrom(1).collect(),
//    vec![vec!['5'],vec!['6'],vec!['7'],vec!['5','6'],vec!['6','7'],vec!['5','6','7']]
//  );
//  // first item in upfrom returns the frist slice of minimum
//  assert_eq!(
//    vec!['A','2','3'].upfrom(1).next,
//    vec!['A']
//  );
//}
