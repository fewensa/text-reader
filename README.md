text-reader
===

[![Build Status](https://drone.0u0.me/api/badges/fewensa/text-reader/status.svg)](https://drone.0u0.me/fewensa/text-reader)

Rust string character reader.

# Usage

```toml
[dependencies]
text-reader = "0.1"
```

# Examples

```rust
use text_reader::TextReader;

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
```

[more](./tests)


# Analysis

```rust
use text_reader::TextReader;

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
```

When create a TextReader `TextReader::new("abc\ndef")`, TextReader status is:

`TextReader { len: 7, text: ['a', 'b', 'c', '\n', 'd', 'e', 'f'], position: 0, line: 1, cursor: 0 }`

## next

And then, read next character `let ch = reader.next()`, `ch` will return `Some('a')`; TextReader status:

`TextReader { len: 7, text: ['a', 'b', 'c', '\n', 'd', 'e', 'f'], position: 1, line: 1, cursor: 1 }`

## peek

peek function not change status, only get current character `let ch = reader.peek()`

`TextReader { len: 7, text: ['a', 'b', 'c', '\n', 'd', 'e', 'f'], position: 1, line: 1, cursor: 1 }`

## back

Back will change TextReader to previous status. return TextReader reference. `reader.back()`.

`TextReader { len: 7, text: ['a', 'b', 'c', '\n', 'd', 'e', 'f'], position: 0, line: 1, cursor: 0 }`

## this_line

this line return current line text, not change TextReader status. `let line_text = reader.this_line()`.

`TextReader { len: 7, text: ['a', 'b', 'c', '\n', 'd', 'e', 'f'], position: 0, line: 1, cursor: 0 }`

line_text is `Some("abc")`

## position

position function return TextReader position value. the num of character position. `let position = reader.position()`

position is `0`

`TextReader { len: 7, text: ['a', 'b', 'c', '\n', 'd', 'e', 'f'], position: 0, line: 1, cursor: 0 }`

## line

return current line number, split of `\n`.

```rust
reader.next();
reader.next();
reader.next();
let line = reader.line(); // 1
// TextReader { len: 7, text: ['a', 'b', 'c', '\n', 'd', 'e', 'f'], position: 3, line: 1, cursor: 3 }
reader.next();
let line = reader.line(); // 2
// TextReader { len: 7, text: ['a', 'b', 'c', '\n', 'd', 'e', 'f'], position: 4, line: 2, cursor: 0 }
```

## cursor

position of line. starting from 0. and change to 0 when it encounters `\n`

## has_next

has next character. can be used with while. or determine if the last character 

