use std::char;
use std::collections::HashMap;
use std::fmt;
use std::iter::Peekable;
use std::str::FromStr;

use crate::JsonValue;

/// Parse error.
///
/// ```
/// use tinyjson::{JsonParser, JsonParseError};
/// let error = JsonParser::new("[1, 2, 3".chars()).parse().unwrap_err();
/// assert!(matches!(error, JsonParseError{..}));
/// ```
#[derive(Debug)]
pub struct JsonParseError {
    msg: String,
    line: usize,
    col: usize,
}

impl JsonParseError {
    fn new(msg: String, line: usize, col: usize) -> JsonParseError {
        JsonParseError { msg, line, col }
    }

    /// Get the line numbr where the parse error happened. This value is 1-based.
    ///
    /// ```
    /// use tinyjson::{JsonParser, JsonParseError};
    /// let error = JsonParser::new("[1, 2, 3".chars()).parse().unwrap_err();
    /// assert_eq!(error.line(), 1);
    /// ```
    pub fn line(&self) -> usize {
        self.line
    }

    /// Get the column numbr where the parse error happened. This value is 1-based.
    ///
    /// ```
    /// use tinyjson::{JsonParser, JsonParseError};
    /// let error = JsonParser::new("[1, 2, 3".chars()).parse().unwrap_err();
    /// assert_eq!(error.column(), 8);
    /// ```
    pub fn column(&self) -> usize {
        self.col
    }
}

impl fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parse error at line:{}, col:{}: {}",
            self.line, self.col, &self.msg,
        )
    }
}

impl std::error::Error for JsonParseError {}

/// Convenient type alias for parse results.
pub type JsonParseResult = Result<JsonValue, JsonParseError>;

// Note: char::is_ascii_whitespace is not available because some characters are not defined as
// whitespace character in JSON spec. For example, U+000C FORM FEED is whitespace in Rust but
// it isn't in JSON.
fn is_whitespace(c: char) -> bool {
    match c {
        '\u{0020}' | '\u{000a}' | '\u{000d}' | '\u{0009}' => true,
        _ => false,
    }
}

/// JSON parser to parse UTF-8 string into `JsonValue` value.
///
/// Basically you don't need to use this struct directly thanks to `FromStr` trait implementation.
///
/// ```
/// use tinyjson::{JsonParser, JsonValue};
///
/// let mut parser = JsonParser::new("[1, 2, 3]".chars());
/// let array = parser.parse().unwrap();
///
/// // Equivalent to the above code using `FromStr`
/// let array: JsonValue = "[1, 2, 3]".parse().unwrap();
/// ```
pub struct JsonParser<I>
where
    I: Iterator<Item = char>,
{
    chars: Peekable<I>,
    line: usize,
    col: usize,
}

impl<I: Iterator<Item = char>> JsonParser<I> {
    /// Create a new parser instance from an iterator which iterates characters. The iterator is usually built from
    /// `str::chars` for parsing `str` or `String` values.
    pub fn new(it: I) -> Self {
        JsonParser {
            chars: it.peekable(),
            line: 1,
            col: 0,
        }
    }

    fn err<T>(&self, msg: String) -> Result<T, JsonParseError> {
        Err(JsonParseError::new(msg, self.line, self.col))
    }

    fn unexpected_eof(&self) -> Result<char, JsonParseError> {
        Err(JsonParseError::new(
            String::from("Unexpected EOF"),
            self.line,
            self.col,
        ))
    }

    fn next_pos(&mut self, c: char) {
        if c == '\n' {
            self.col = 0;
            self.line += 1;
        } else {
            self.col += 1;
        }
    }

    fn peek(&mut self) -> Result<char, JsonParseError> {
        while let Some(c) = self.chars.peek().copied() {
            if !is_whitespace(c) {
                return Ok(c);
            }
            self.next_pos(c);
            self.chars.next().unwrap();
        }
        self.unexpected_eof()
    }

    fn next(&mut self) -> Option<char> {
        while let Some(c) = self.chars.next() {
            self.next_pos(c);
            if !is_whitespace(c) {
                return Some(c);
            }
        }
        None
    }

    fn consume(&mut self) -> Result<char, JsonParseError> {
        if let Some(c) = self.next() {
            Ok(c)
        } else {
            self.unexpected_eof()
        }
    }

    fn consume_no_skip(&mut self) -> Result<char, JsonParseError> {
        if let Some(c) = self.chars.next() {
            self.next_pos(c);
            Ok(c)
        } else {
            self.unexpected_eof()
        }
    }

    fn parse_object(&mut self) -> JsonParseResult {
        if self.consume()? != '{' {
            return self.err(String::from("Object must starts with '{'"));
        }

        if self.peek()? == '}' {
            self.consume().unwrap();
            return Ok(JsonValue::Object(HashMap::new()));
        }

        let mut m = HashMap::new();
        loop {
            let key = match self.parse_any()? {
                JsonValue::String(s) => s,
                v => return self.err(format!("Key of object must be string but found {:?}", v)),
            };

            let c = self.consume()?;
            if c != ':' {
                return self.err(format!(
                    "':' is expected after key of object but actually found '{}'",
                    c
                ));
            }

            m.insert(key, self.parse_any()?);

            match self.consume()? {
                ',' => {}
                '}' => return Ok(JsonValue::Object(m)),
                c => {
                    return self.err(format!(
                        "',' or '}}' is expected for object but actually found '{}'",
                        c.escape_debug(),
                    ))
                }
            }
        }
    }

    fn parse_array(&mut self) -> JsonParseResult {
        if self.consume()? != '[' {
            return self.err(String::from("Array must starts with '['"));
        }

        if self.peek()? == ']' {
            self.consume().unwrap();
            return Ok(JsonValue::Array(vec![]));
        }

        let mut v = vec![self.parse_any()?];
        loop {
            match self.consume()? {
                ',' => {}
                ']' => return Ok(JsonValue::Array(v)),
                c => {
                    return self.err(format!(
                        "',' or ']' is expected for array but actually found '{}'",
                        c
                    ))
                }
            }

            v.push(self.parse_any()?); // Next element
        }
    }

    fn push_utf16(&self, s: &mut String, utf16: &mut Vec<u16>) -> Result<(), JsonParseError> {
        if utf16.is_empty() {
            return Ok(());
        }

        match String::from_utf16(utf16) {
            Ok(utf8) => s.push_str(&utf8),
            Err(err) => return self.err(format!("Invalid UTF-16 sequence {:?}: {}", &utf16, err)),
        }
        utf16.clear();
        Ok(())
    }

    fn parse_string(&mut self) -> JsonParseResult {
        if self.consume()? != '"' {
            return self.err(String::from("String must starts with double quote"));
        }

        let mut utf16 = Vec::new(); // Buffer for parsing \uXXXX UTF-16 characters
        let mut s = String::new();
        loop {
            let c = match self.consume_no_skip()? {
                '\\' => match self.consume_no_skip()? {
                    '\\' => '\\',
                    '/' => '/',
                    '"' => '"',
                    'b' => '\u{0008}',
                    'f' => '\u{000c}',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    'u' => {
                        let mut u = 0u16;
                        for _ in 0..4 {
                            let c = self.consume()?;
                            if let Some(h) = c.to_digit(16) {
                                u = u * 0x10 + h as u16;
                            } else {
                                return self.err(format!("Unicode character must be \\uXXXX (X is hex character) format but found character '{}'", c));
                            }
                        }
                        utf16.push(u);
                        // Additional \uXXXX character may follow. UTF-16 characters must be converted
                        // into UTF-8 string as sequence because surrogate pairs must be considered
                        // like "\uDBFF\uDFFF".
                        continue;
                    }
                    c => return self.err(format!("'\\{}' is invalid escaped character", c)),
                },
                '"' => {
                    self.push_utf16(&mut s, &mut utf16)?;
                    return Ok(JsonValue::String(s));
                }
                // Note: c.is_control() is not available here because JSON accepts 0x7f (DEL) in
                // string literals but 0x7f is control character.
                // Rough spec of JSON says string literal cannot contain control characters. But it
                // can actually contain 0x7f.
                c if (c as u32) < 0x20 => {
                    return self.err(format!(
                        "String cannot contain control character {}",
                        c.escape_debug(),
                    ));
                }
                c => c,
            };

            self.push_utf16(&mut s, &mut utf16)?;

            s.push(c);
        }
    }

    fn parse_constant(&mut self, s: &'static str) -> Option<JsonParseError> {
        for c in s.chars() {
            match self.consume_no_skip() {
                Ok(x) if x != c => {
                    return Some(JsonParseError::new(
                        format!("Unexpected character '{}' while parsing '{}'", c, s),
                        self.line,
                        self.col,
                    ));
                }
                Ok(_) => {}
                Err(e) => return Some(e),
            }
        }
        None
    }

    fn parse_null(&mut self) -> JsonParseResult {
        match self.parse_constant("null") {
            Some(err) => Err(err),
            None => Ok(JsonValue::Null),
        }
    }

    fn parse_true(&mut self) -> JsonParseResult {
        match self.parse_constant("true") {
            Some(err) => Err(err),
            None => Ok(JsonValue::Boolean(true)),
        }
    }

    fn parse_false(&mut self) -> JsonParseResult {
        match self.parse_constant("false") {
            Some(err) => Err(err),
            None => Ok(JsonValue::Boolean(false)),
        }
    }

    fn parse_number(&mut self) -> JsonParseResult {
        let mut s = String::new();

        if let Some('-') = self.chars.peek() {
            self.consume_no_skip().unwrap();
            s.push('-');
        };

        match self.consume_no_skip()? {
            '0' => s.push('0'),
            d @ '1'..='9' => {
                s.push(d);
                while let Some('0'..='9') = self.chars.peek() {
                    s.push(self.consume_no_skip().unwrap());
                }
            }
            c => {
                let msg = format!("Expected '0'~'9' for integer part of number but got {}", c);
                return self.err(msg);
            }
        }

        if let Some('.') = self.chars.peek() {
            s.push(self.consume_no_skip().unwrap()); // Eat '.'

            match self.consume_no_skip()? {
                d @ '0'..='9' => s.push(d),
                c => {
                    let msg = format!("At least one digit must follow after '.' but got {}", c);
                    return self.err(msg);
                }
            }

            while let Some('0'..='9') = self.chars.peek() {
                s.push(self.consume_no_skip().unwrap());
            }
        }

        if let Some('e' | 'E') = self.chars.peek() {
            s.push(self.consume_no_skip().unwrap()); // Eat 'e' or 'E'

            if let Some('-' | '+') = self.chars.peek() {
                s.push(self.consume_no_skip().unwrap());
            }

            match self.consume_no_skip()? {
                d @ '0'..='9' => s.push(d),
                c => {
                    return self.err(format!(
                        "At least one digit must follow exponent part of number but got {}",
                        c
                    ));
                }
            };

            while let Some('0'..='9') = self.chars.peek() {
                s.push(self.consume_no_skip().unwrap());
            }
        }

        match s.parse() {
            Ok(n) => Ok(JsonValue::Number(n)),
            Err(err) => self.err(format!("Invalid number literal '{}': {}", s, err)),
        }
    }

    fn parse_any(&mut self) -> JsonParseResult {
        match self.peek()? {
            '0'..='9' | '-' => self.parse_number(),
            '"' => self.parse_string(),
            '[' => self.parse_array(),
            '{' => self.parse_object(),
            't' => self.parse_true(),
            'f' => self.parse_false(),
            'n' => self.parse_null(),
            c => self.err(format!("Invalid character: {}", c.escape_debug())),
        }
    }

    /// Run the parser to parse one JSON value.
    pub fn parse(&mut self) -> JsonParseResult {
        let v = self.parse_any()?;

        if let Some(c) = self.next() {
            return self.err(format!(
                "Expected EOF but got character '{}'",
                c.escape_debug(),
            ));
        }

        Ok(v)
    }
}

/// Parse given `str` object into `JsonValue` value. This is recommended way to parse strings into JSON value with
/// this library.
///
/// ```
/// use tinyjson::JsonValue;
///
/// let array: JsonValue = "[1, 2, 3]".parse().unwrap();
/// assert!(array.is_array());
/// ```
impl FromStr for JsonValue {
    type Err = JsonParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        JsonParser::new(s.chars()).parse()
    }
}
