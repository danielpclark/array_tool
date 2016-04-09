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

#[test]
fn it_substitutes_character_at_each_indexed_point() {
  use array_tool::string::SubstMarks;

  assert_eq!("asdf asdf asdf".subst_marks(vec![0,5,8], "Z"), "Zsdf ZsdZ asdf");
  assert_eq!("asdf asdf asdf".subst_marks(vec![0,5,8], "\n"), "\nsdf \nsd\n asdf");
}

#[test]
fn it_word_wraps_for_string() {
  use array_tool::string::WordWrap;

  assert_eq!(
    "01234 67 9 BC EFG IJ".word_wrap(6),
    "01234\n67 9\nBC\nEFG IJ"
  );
}


