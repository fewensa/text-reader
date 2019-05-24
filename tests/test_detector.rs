
use text_reader::TextReader;

#[test]
fn test_detector() {
  let text = r#"
{"type": "typeA", "name": "Earth", "continent": ["Asia", "Europe"]}
  "#;
  let mut reader = TextReader::new(text);
  let mut rets = Vec::new();
  while reader.has_next() {
    match reader.next() {
      Some('"') => {
        let mut detector = reader.detector();
        rets.push('"');
        if detector.next_text("type").yes() {
          detector.rollback();
          rets.push('t');
        }
        continue;
      },
      Some(ch) => {
        rets.push(ch);
        continue;
      }
      None => {}
    }
  }
  let ret = rets.iter().collect::<String>();
  println!("{}", ret);

}

