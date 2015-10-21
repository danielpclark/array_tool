extern crate array_tool;

#[test]
fn it_implements_uniques() {
  assert_eq!(array_tool::uniques(vec![1,2,3,4,5,6],vec![1,2]), vec![3,4,5,6]);
  assert_eq!(array_tool::uniques(vec![1,2,3,4,5,6],vec![1,2,3,4]), vec![5,6]);
  assert_eq!(array_tool::uniques(vec![1,2,3],vec![1,2,3,4,5]), vec![4,5])
}
