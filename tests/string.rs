extern crate array_tool;

#[test]
fn it_justifies_one_line_in_for_string() {
  use array_tool::string::Justify;

  assert_eq!("asd asdf asd".justify_line(14), "asd  asdf  asd");
  assert_eq!("asd asdf asd".justify_line(16), "asd   asdf   asd");
  assert_eq!("asd as df asd".justify_line(16), "asd  as  df  asd");
  assert_eq!("asd as df asd".justify_line(18), "asd   as   df  asd");
  assert_eq!("  asd as df asd  ".justify_line(16), "asd  as  df  asd");
  assert_eq!("asdasdfasd".justify_line(16), "asdasdfasd");
  assert_eq!("asdasdfasd".justify_line(6), "asdasdfasd");
}
