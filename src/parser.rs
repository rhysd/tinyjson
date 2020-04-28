use std::char;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::FromStr;

use crate::JsonValue;

#[derive(Debug)]
pub struct JsonParseError {
    msg: String,
    line: usize,
    col: usize,
}

impl JsonParseError {
    fn new(msg: String, line: usize, col: usize) -> JsonParseError {
        JsonParseError {
            msg: msg,
            line: line,
            col: col,
        }
    }
}

pub type JsonParseResult = Result<JsonValue, JsonParseError>;

pub struct JsonParser<I>
where
    I: Iterator<Item = char>,
{
    chars: Peekable<I>,
    line: usize,
    col: usize,
}

impl<I: Iterator<Item = char>> JsonParser<I> {
    pub fn new(it: I) -> Self {
        JsonParser {
            chars: it.peekable(),
            line: 0,
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

    fn peek(&mut self) -> Result<char, JsonParseError> {
        loop {
            match self.chars.peek() {
                Some(c) => {
                    if !c.is_whitespace() {
                        return Ok(*c);
                    }
                    if *c == '\n' {
                        self.col = 0;
                        self.line += 1;
                    } else {
                        self.col += 1;
                    }
                }
                None => break,
            }
            self.chars.next();
        }
        self.unexpected_eof()
    }

    fn next(&mut self) -> Option<char> {
        while let Some(c) = self.chars.next() {
            self.col += 1;
            if !c.is_whitespace() {
                return Some(c);
            }
            if c == '\n' {
                self.col = 0;
                self.line += 1;
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
        match self.chars.next() {
            Some(c) => Ok(c),
            None => self.unexpected_eof(),
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

        let mut v = vec![];
        loop {
            v.push(self.parse_any()?);

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
        }
    }

    fn parse_special_char(&mut self) -> Result<char, JsonParseError> {
        Ok(match self.consume_no_skip()? {
            '\\' => '\\',
            '/' => '/',
            '"' => '"',
            'b' => '\u{0008}',
            'f' => '\u{000c}',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            'u' => {
                let mut u = 0 as u32;
                for _ in 0..4 {
                    let c = self.consume()?;
                    let h = match c.to_digit(16) {
                            Some(n) => n,
                            None => return self.err(format!("Unicode character must be \\uXXXX (X is hex character) format but found '{}'", c)),
                        };
                    u = u * 0x10 + h;
                }
                match char::from_u32(u) {
                    Some(c) => c,
                    None => {
                        return self
                            .err(format!("Cannot convert \\u{:x} into unicode character", u));
                    }
                }
            }
            c => return self.err(format!("'\\{}' is invalid escaped character", c)),
        })
    }

    fn parse_string(&mut self) -> JsonParseResult {
        if self.consume()? != '"' {
            return self.err(String::from("String must starts with double quote"));
        }

        let mut s = String::new();
        loop {
            s.push(match self.consume_no_skip()? {
                '\\' => self.parse_special_char()?,
                '"' => return Ok(JsonValue::String(s)),
                c if c.is_control() => {
                    return self.err(format!(
                        "String cannot convert control character {}",
                        c.escape_debug(),
                    ));
                }
                c => c,
            });
        }
    }

    fn parse_constant(&mut self, s: &'static str) -> Option<JsonParseError> {
        for c in s.chars() {
            match self.consume_no_skip() {
                Ok(x) if x != c => {
                    return Some(JsonParseError::new(
                        format!("err while parsing '{}', invalid character '{}' found", s, c),
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
        let mut c = self.consume()?;
        let negative = match c {
            '-' => {
                c = self.consume()?;
                true
            }
            _ => false,
        };

        let mut s = c.to_string();
        loop {
            let d = match self.chars.peek() {
                Some(x) => *x,
                None => break,
            };

            s.push(match d {
                '0'..='9' | '.' | 'e' | 'E' => d,
                _ => break,
            });
            self.chars.next();
        }

        let n: f64 = match s.parse() {
            Ok(num) => num,
            Err(_) => return self.err(format!("Invalid number: {}", s)),
        };

        Ok(JsonValue::Number(if negative { -n } else { n }))
    }

    fn parse_any(&mut self) -> JsonParseResult {
        match self.peek()? {
            '1'..='9' | '-' => self.parse_number(),
            '"' => self.parse_string(),
            '[' => self.parse_array(),
            '{' => self.parse_object(),
            't' => self.parse_true(),
            'f' => self.parse_false(),
            'n' => self.parse_null(),
            c => self.err(format!("Invalid character: {}", c.escape_debug())),
        }
    }

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

impl FromStr for JsonValue {
    type Err = JsonParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        JsonParser::new(s.chars()).parse()
    }
}
