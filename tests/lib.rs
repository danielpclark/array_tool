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
}

#[test]
fn it_can_return_its_own_unique(){
  use array_tool::vec::Uniq;
  assert_eq!(vec![1,2,1,3,4,3,4,5,6].unique(),vec![1,2,3,4,5,6]);
}

#[test]
fn it_answers_about_uniqueness(){
  use array_tool::vec::Uniq;
  assert_eq!(vec![1,2,1,3,4,3,4,5,6].is_unique(), false);
  assert_eq!(vec![1,2,3,4,5,6].is_unique(), true);
}

#[test]
fn it_checks_emptiness(){
  use array_tool::vec::Empty;
  let mut x = vec![1];
  assert_eq!(x.empty(), false);
  x.pop();
  assert_eq!(x.empty(), true);
}
