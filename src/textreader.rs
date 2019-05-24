use std::ffi::OsStr;
use crate::Detector;

#[derive(Debug)]
pub struct TextReader {
  len: usize,
  text: Vec<char>,
  position: usize,
  line: usize,
  cursor: isize,
}

///
/// Text character reader.
///
/// # Examples
/// ```
/// use text_reader::TextReader;
/// let mut reader = TextReader::new("abc\ndef");
/// while reader.has_next() {
///   let ch = reader.next();
///   println!("{:?}", ch);
///   if let Some('\n') = ch {
///     println!("{:?}", reader.this_line());
///   }
/// }
/// println!("{:?}", reader.this_line());
/// ```
impl TextReader {
  pub fn new<S: AsRef<OsStr>>(text: S) -> TextReader {
    let chars: Vec<char> = text.as_ref().to_str().unwrap().chars().collect();
    let len = chars.len();
    TextReader {
      text: chars,
      position: 0,
      line: 1,
      len,
      cursor: 0,
    }
  }

  ///
  /// Detect possible strings
  ///
  /// # Examples
  /// ```
  /// use text_reader::TextReader;
  /// let mut reader = TextReader::new("abc\ndef");
  /// let mut detector = reader.detector();
  ///
  /// ```
  pub fn detector(&mut self) -> Detector {
    Detector::new(self)
  }

  ///
  /// Reset to first character
  ///
  /// # Examples
  /// ```
  /// use text_reader::TextReader;
  /// let mut reader = TextReader::new("abc\ndef");
  /// let reader = reader.reset();
  /// ```
  pub fn reset(&mut self) -> &mut TextReader {
    self.line = 1;
    self.position = 0;
    self.cursor = 0;
    self
  }

  ///
  /// Peek current character
  ///
  /// # Examples
  /// ```
  /// use text_reader::TextReader;
  /// let mut reader = TextReader::new("abc\ndef");
  /// let ch = reader.peek();
  /// ```
  pub fn peek(&self) -> Option<char> {
    if self.position == 0 {
      return None;
    }
    if self.position - 1 >= self.len {
      return None;
    }
    self.text.get(self.position - 1).cloned()
  }

  ///
  /// Next character
  ///
  /// # Examples
  /// ```
  /// use text_reader::TextReader;
  /// let mut reader = TextReader::new("abc\ndef");
  /// let ch = reader.next();
  /// ```
  pub fn next(&mut self) -> Option<char> {
    if !self.has_next() {
      return None;
    }
    let &ch = self.text.get(self.position).unwrap();
    self.position += 1;
    self.cursor += 1;

    if ch == '\n' {
      self.line += 1;
      self.cursor = 0;
    }

    Some(ch)
  }

  ///
  /// Position of text
  ///
  /// # Examples
  /// ```
  /// use text_reader::TextReader;
  /// let mut reader = TextReader::new("abc\ndef");
  /// let position = reader.position();
  /// ```
  ///
  pub fn position(&self) -> usize {
    self.position
  }

  /// Current text line number
  /// # Examples
  /// ```
  /// use text_reader::TextReader;
  /// let mut reader = TextReader::new("abc\ndef");
  /// let line = reader.line();
  /// ```
  pub fn line(&self) -> usize {
    self.line
  }

  /// Current line position
  /// # Examples
  /// ```
  /// use text_reader::TextReader;
  /// let mut reader = TextReader::new("abc\ndef");
  /// reader.next();
  /// reader.next();
  /// reader.next();
  /// reader.next();
  /// let cursor = reader.cursor();
  /// println!("CURSOR: {}", cursor); // 2
  /// ```
  pub fn cursor(&self) -> usize {
    self.cursor as usize
  }

  /// Text length
  /// # Examples
  /// ```
  /// use text_reader::TextReader;
  /// let reader = TextReader::new("abc\ndef");
  /// let len = reader.len();
  /// ```
  pub fn len(&self) -> usize {
    self.len
  }

  ///
  /// Back to previous character
  ///
  /// # Examples
  ///
  /// ```
  /// use text_reader::TextReader;
  /// let mut reader = TextReader::new("abc\ndef");
  /// reader.back();
  /// ```
  ///
  pub fn back(&mut self) -> &mut TextReader {
    if self.position == 0 {
      return self;
    }

    match self.peek() {
      None => return self,
      Some(ch) => {
        if ch != '\n' {
          self.position -= 1;
          self.cursor -= 1;
          return self;
        }
      }
    }

    self.position -= 1;
    self.line -= 1;

    let position = self.position;
    let line = self.line;

    let mut distance = 0;
    loop {
      match self.back().peek() {
        Some('\n') | None => break,
        _ => distance += 1
      }
    }

    self.position = position;
    self.line = line;
    self.cursor = distance + 1;
    self
  }

  ///
  /// Current line string
  ///
  /// # Examples
  /// ```
  /// use text_reader::TextReader;
  /// let mut reader = TextReader::new("abc\ndef");
  /// let line_text = reader.this_line();
  /// ```
  pub fn this_line(&mut self) -> Option<String> {
    let position = self.position;
    let cursor = self.cursor;
    let line = self.line;

    loop {
      if self.position == 0 || self.cursor == 0 {
        break;
      }

      match self.back().peek() {
        Some('\n') => break,
        _ => continue,
      }
    }

    let start_position = self.position;

    while self.has_next() {
      match self.next() {
        Some('\n') => {
          self.back();
          break;
        }
        _ => continue
      }
    }

    if start_position == self.position {
      return None;
    }

    let line_text = self.text.iter().enumerate()
      .filter(|(ix, _ch)| *ix >= start_position && *ix < self.position)
      .map(|(_ix, ch)| ch)
      .into_iter()
      .collect();

    self.position = position;
    self.cursor = cursor;
    self.line = line;

    Some(line_text)
  }

  ///
  /// Has next character
  ///
  /// # Examples
  /// ```
  /// use text_reader::TextReader;
  /// let mut reader = TextReader::new("abc\ndef");
  /// while  reader.has_next(){
  ///    let ch = reader.next();
  /// }
  /// ```
  pub fn has_next(&self) -> bool {
    self.position < self.len
  }
}

