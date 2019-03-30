use text_reader::TextReader;

#[test]
fn test_reader() {
  let mut reader = TextReader::new("華文\ndef");

  assert_eq!(6, reader.len());
  assert_eq!(Some('華'), reader.next());
  assert_eq!(Some('文'), reader.next());
  assert_eq!(2, reader.position());
  assert_eq!(Some("華文".to_string()), reader.this_line());
  assert_eq!(1, reader.line());
  assert_eq!(2, reader.cursor());
  assert_eq!(Some('\n'), reader.next());
  assert_eq!(2, reader.line());
  assert_eq!(0, reader.cursor());
  assert_eq!(Some("def".to_string()), reader.this_line());
  reader.back().back();
  assert_eq!(Some('文'), reader.next());
  assert_eq!(true, reader.has_next());
  assert_eq!(Some('\n'), reader.next());
  assert_eq!(Some('d'), reader.next());
  assert_eq!(Some('e'), reader.next());
  assert_eq!(Some('f'), reader.next());
  assert_eq!(false, reader.has_next());
}

#[test]
fn test_while() {
  let mut reader = TextReader::new("華文\ndef");
  while reader.has_next() {
    let position = reader.position();
    match reader.next() {
      Some(ch) => match position {
        0 => assert_eq!('華', ch),
        1 => assert_eq!('文', ch),
        2 => assert_eq!('\n', ch),
        3 => assert_eq!('d', ch),
        4 => assert_eq!('e', ch),
        5 => assert_eq!('f', ch),
        _ => {}
      },
      None => panic!("None")
    }
  }
}

#[test]
fn test_stat() {
  let mut reader = TextReader::new("abc\ndef");
  println!("{:?}", reader);
  reader.next();
  println!("{:?}", reader);
  reader.back();
  println!("{:?}", reader);
  let line_text = reader.this_line();
  println!("{:?}", line_text);
  let position = reader.position();
  println!("{:?}", position);
  println!("{:?}", reader);

  reader.next();
  reader.next();
  reader.next();
  let line = reader.line(); // 1
  assert_eq!(1, line);
  println!("{:?}", reader);
  reader.next();
  let line = reader.line(); // 2
  assert_eq!(2, line);
  println!("{:?}", reader);
}
