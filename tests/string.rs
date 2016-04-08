extern crate array_tool;

#[test]
fn it_justifies_one_line_in_type_string() {
  use array_tool::string::Justify;

  assert_eq!("asd asdf asd".justify_line(14), "asd  asdf  asd");
}
