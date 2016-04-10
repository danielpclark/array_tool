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
fn it_seeks_end_of_whitespace_after_offset() {
  use array_tool::string::AfterWhitespace;

  assert_eq!("asdf           asdf asdf".seek_end_of_whitespace(6), Some(9));
  assert_eq!("asdf".seek_end_of_whitespace(3), Some(0));
  assert_eq!("asdf           ".seek_end_of_whitespace(6), None);
  assert_eq!("asdf".seek_end_of_whitespace(6), None);
}

#[test]
fn it_word_wraps_for_string() {
  use array_tool::string::WordWrap;

  assert_eq!(
    "01234 67 9 BC EFG IJ".word_wrap(6),
    "01234\n67 9\nBC EFG\nIJ"
  );

  assert_eq!(
    "0123456789ABC EFG IJ".word_wrap(6),
    "0123456789ABC\nEFG IJ"
  );
  assert_eq!(
    "1234\n 1234 6789 1234".word_wrap(10),
    "1234\n 1234 6789\n1234"
  );
  assert_eq!(
    "1234\n 1234 67 90 1234".word_wrap(10),
    "1234\n 1234 67\n90 1234"
  );
  assert_eq!(
    "1234\n 1234 67 90A 1234".word_wrap(10),
    "1234\n 1234 67\n90A 1234"
  );
  assert_eq!("1  \n34 ".word_wrap(3), "1  \n34 ");
  assert_eq!("1   34".word_wrap(3),   "1  \n34");
  assert_eq!("\n \n \n \n".word_wrap(1), "\n \n \n \n" );

  // White space to new line shouldn't add new lines.  Use seek ahead.
  assert_eq!("\nAA\nA \nA   \n".word_wrap(1), "\nAA\nA \nA   \n" );
  assert_eq!("\n \n \n \n     ".word_wrap(1), "\n \n \n \n     " );
}

