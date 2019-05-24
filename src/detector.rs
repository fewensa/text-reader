use std::ffi::OsStr;

use crate::TextReader;

/// Text reader detector
///
/// # Examples
///
/// ```rust
/// use text_reader::TextReader;
/// use text_reader::Detector;
///
/// let text = r#""typeA""#;
/// let mut reader = TextReader::new(text);
/// let mut rets = Vec::new();
/// while reader.has_next() {
///   match reader.next() {
///     Some('"') => {
///       let mut detector = reader.detector();
///       rets.push('"');
///       if detector.next_text("type").yes() {
///         detector.rollback();
///         rets.push('t');
///       }
///       continue;
///     }
///     Some(ch) => {
///       rets.push(ch);
///       continue;
///     }
///     None => {}
///   }
/// }
/// let ret = rets.iter().collect::<String>();
/// println!("{}", ret); // "ttypeA"
/// ```
#[derive(Debug)]
pub struct Detector<'a> {
  reader: &'a mut TextReader,
  compares: Vec<char>,
  last_len: usize,
}

impl<'a> Detector<'a> {

  /// Create detector
  ///
  /// # Examples
  ///
  /// ```rust
  /// use text_reader::TextReader;
  ///
  /// let mut reader = TextReader::new("abc");
  /// ```
  pub fn new(reader: &'a mut TextReader) -> Self {
    Self { reader, compares: Vec::new(), last_len: 0 }
  }

  /// Detect next char
  ///
  /// # Examples
  ///
  /// ```rust
  /// use text_reader::TextReader;
  ///
  /// let mut reader = TextReader::new("abc");
  /// let mut detector = reader.detector();
  /// detector.next_char('a');
  /// ```
  pub fn next_char(&mut self, ch: char) -> &mut Self {
    self.compares.push(ch);
    self
  }

  /// Detect next string
  ///
  /// # Examples
  ///
  /// ```rust
  /// use text_reader::TextReader;
  ///
  /// let mut reader = TextReader::new("abc");
  /// let mut detector = reader.detector();
  /// detector.next_text("ab");
  /// ```
  pub fn next_text<S: AsRef<OsStr>>(&mut self, text: S) -> &mut Self {
    text.as_ref().to_str().unwrap().chars().collect::<Vec<char>>()
      .iter()
      .for_each(|ch| { self.next_char(ch.clone()); });
    self
  }

  /// Detect result is true
  ///
  /// # Examples
  ///
  /// ```rust
  /// use text_reader::TextReader;
  ///
  /// let mut vec = Vec::new();
  /// let mut reader = TextReader::new("abc");
  ///
  /// while reader.has_next() {
  ///   match reader.next() {
  ///     Some(ch) => {
  ///       let mut detector = reader.detector();
  ///       if detector.next_text("bc").yes() {
  ///         println!("Detect ab");
  ///       }
  ///       vec.push(ch);
  ///     },
  ///     None => {}
  ///   }
  /// }
  /// println!("{}", vec.iter().collect::<String>()); // a
  /// ```
  pub fn yes(&mut self) -> bool {
    self.detect()
  }

  /// Detect result is false
  pub fn no(&mut self) -> bool {
    !self.detect()
  }

  /// Rollback detector
  /// If detect success detector not back position. if want, use rollback function to reset reader position
  ///
  /// # Examples
  ///
  /// ```rust
  /// use text_reader::TextReader;
  ///
  /// let mut vec = Vec::new();
  ///
  /// let mut reader = TextReader::new("abc");
  /// while reader.has_next() {
  ///   match reader.next() {
  ///     Some(ch) => {
  ///       let mut detector = reader.detector();
  ///       if detector.next_text("bc").yes() {
  ///         detector.rollback();
  ///         println!("Detect ab");
  ///       }
  ///       vec.push(ch);
  ///     },
  ///     None => {}
  ///   }
  /// }
  /// println!("{}", vec.iter().collect::<String>()); // abc
  /// ```
  pub fn rollback(&mut self) -> &mut Self {
    for _ in 0..self.last_len {
      self.reader.back();
    }
    self
  }

  fn detect(&mut self) -> bool {
    let mut ix = 0;
    let len = self.compares.len();

    while self.reader.has_next() {
      match self.reader.next() {
        Some(ch) => {
          let compare = match self.compares.get(ix) {
            Some(c) => c.clone(),
            None => {
              self.reader.back();
              self.last_len = len;
              return true;
            }
          };
          if ch != compare {
            self.restore(ix + 1);
            return false;
          }
          ix += 1;
        }
        None => {
          self.restore(ix);
          return false;
        }
      }
    }
    let same = ix == len;
    if !same {
      self.restore(ix);
    } else {
      self.last_len = len;
    }
    same
  }

  fn restore(&mut self, count: usize) -> &mut Self {
    if count == 0 {
      return self;
    }
    for _ in 0..count {
      self.reader.back();
    }
    self
  }
}

