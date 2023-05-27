use logos::Logos;

fn string_to_constant(s: &str) -> i64{
    let mut s = s;
    if s.starts_with("0b") {
        s = &s[2..];
        return i64::from_str_radix(s, 2).unwrap();
    }
    if s.starts_with("0x") {
        s = &s[2..];
        return i64::from_str_radix(s, 16).unwrap();
    }
    if s.starts_with("0o") {
        s = &s[2..];
        return i64::from_str_radix(s, 8).unwrap();
    }
    return s.parse().unwrap();
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    NumberParseError,
    #[default]
    Other
}

impl From<std::num::ParseIntError> for LexingError {
   fn from(_: std::num::ParseIntError) -> Self {
      LexingError::NumberParseError
  }
}

impl From<std::num::ParseFloatError> for LexingError {
  fn from(_: std::num::ParseFloatError) -> Self {
     LexingError::NumberParseError
  }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),
    #[regex(r"0b[0-1]+", |lex| string_to_constant(lex.slice()))]
    Binary(i64),
    #[regex(r"0x[0-9a-fA-F]+", |lex| string_to_constant(lex.slice()))]
    Hexadecimal(i64),
    #[regex(r"0o[0-7]+", |lex| string_to_constant(lex.slice()))]
    Octal(i64),
    #[token("**")]
    Exponent,
    #[token("//")]
    Root,
    #[token("<<")]
    LeftShift,
    #[token(">>")]
    RightShift,
    #[token("&")]
    And,
    #[token("|")]
    Or,
    #[token("^")]
    Xor,
    #[token("!")]
    Not,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token("(")]
    LeftParenthesis,
    #[token(")")]
    RightParenthesis,
}
