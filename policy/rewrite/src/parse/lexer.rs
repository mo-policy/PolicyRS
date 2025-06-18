// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use logos::{Lexer, Logos, Skip};
use std::fmt;
use std::num::{ ParseIntError, ParseFloatError };
use std::str::FromStr;

use crate::data::uri::DidUri;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\f]+")]
#[logos(skip r"//[^\r\n]*")]
#[logos(error = LexicalError)]
#[logos(extras = ((usize, usize), (usize, usize)))]
// extras = ((start_line_num, start_line_char), (end_line_num, end_line_char))
pub enum PolicyToken {

    #[regex(r"\r\n|\r|\n", newline_callback)]
    Newline,

    // Multi-line comments
    #[token("/*", comment_callback)]
    CommentStart,
    #[token("*/")]      CommentEnd,

    // Operators
    #[token("=")]       Equal,
    #[token("+")]       Add,
    #[token("-")]       Sub,
    #[token("*")]       Mul,
    #[token("/")]       Div,
    #[token("%")]       Mod,
    #[token("@")]       AtSign,
    #[token("<")]       LessThan,
    #[token(">")]       GreaterThan,
    #[token("<=")]      LessOrEqual,
    #[token(">=")]      GreaterOrEqual,
    #[token("<>")]      NotEqual,
    #[token("||")]      BooleanOr,
    #[token("&&")]      BooleanAnd,

    // Symbolic Keywords
    #[token("(")]       LeftParen,
    #[token(")")]       RightParen,
    #[token("[")]       LeftBracket,
    #[token("]")]       RightBracket,
    #[token("{")]       LeftBrace,
    #[token("}")]       RightBrace,
    #[token("{[")]      LeftSetBrace,
    #[token("]}")]      RightSetBrace,
    #[token("{=")]      LeftEvalBrace,
    #[token("=}")]      RightEvalBrace,
    #[token("->")]      RightArrow,
    #[token(".")]       Dot,
    #[token(",")]       Comma,
    #[token(":")]       Colon,
    #[token(";")]       Semicolon,
    #[token("|")]       Bar,
    #[token("|,")]      BarComma,
    #[token("|;")]      BarSemicolon,
    #[token("!")]       Bang,
    #[token(":=")]      ColonEqual,

    // Keywords
    #[token("as")]      As,
    #[token("begin")]   Begin,
    #[token("do")]      Do,
    #[token("elif")]    Elif,
    #[token("else")]    Else,
    #[token("end")]     End,
    #[token("for")]     For,
    #[token("fun")]     Fun,
    #[token("if")]      If,
    #[token("in")]      In,
    #[token("let")]     Let,
    #[token("match")]   Match,
    #[token("on")]      On,
    #[token("policy")]  Policy,
    #[token("receive")] Receive,
    #[token("ref")]     Ref,
    #[token("send")]    Send,
    #[token("then")]    Then,
    #[token("throw")]   Throw,
    #[token("try")]     Try,
    #[token("use")]     Use,
    #[token("when")]    When,
    #[token("while")]   While,
    #[token("with")]    With,
    #[token("true")]    True,
    #[token("false")]   False,
    #[token("null")]    Null,

    #[regex(r"`[^\r\n`]*`", diduri_callback)]
    DidUri(DidUri),

    #[regex(r#""(([^\u0000-\u001F\\"]+)|(\\(["\\/bfnrt]|u[a-fA-F0-9]{4})))*""#, string_callback)]
    String(String),
    
    #[regex(r"-?(?:0|[1-9]\d*)", integer_callback)]
    Integer(i64),

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)(?:[eE][+-]?\d+)?", double_callback)]
    Double(f64),

    #[regex(r"[a-zA-Z_][0-9a-zA-Z_]*", |lex| lex.slice().to_owned())]
    Ident(String),

}

impl fmt::Display for PolicyToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub line_num: usize,                    // line number starting with 0
    pub line_begin_char: usize,             // absolute char position of start of line
    pub char_num: usize,                    // absolute char position from start of source
}

impl Default for Position {
    fn default() -> Self {
        Self { line_num: 0, char_num: 0, line_begin_char: 0, }
    }
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct PolicyLexer<'input> {
  // instead of an iterator over characters, we have a token iterator
  lexer: Lexer<'input, PolicyToken>,
}

impl<'input> PolicyLexer<'input> {
    pub fn new(input: &'input str) -> Self {
      Self { lexer: PolicyToken::lexer(input) }
    }
}

impl<'input> Iterator for PolicyLexer<'input> {
    type Item = Spanned<PolicyToken, Position, LexicalError>;
    
    fn next(&mut self) -> Option<Self::Item> {
        // extras = ((start_line_num, start_line_char), (end_line_num, end_line_char))
        self.lexer
        .next()
        .map(|token|{
            let pos_start = Position {
                line_num: self.lexer.extras.0.0,
                line_begin_char: self.lexer.extras.0.1,
                char_num: self.lexer.span().start,
            };
            let pos_end = Position {
                line_num: self.lexer.extras.1.0,
                line_begin_char: self.lexer.extras.1.1,
                char_num: self.lexer.span().end,
            };
            Ok((pos_start, token?, pos_end))}
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidDidUrl,
    InvalidString(String),
    InvalidInteger(ParseIntError),
    InvalidDouble(ParseFloatError),
    UnterminatedComment,
    #[default]
    InvalidToken,
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

impl From<ParseFloatError> for LexicalError {
    fn from(err: ParseFloatError) -> Self {
        LexicalError::InvalidDouble(err)
    }
}


fn string_callback(lex: &mut Lexer<'_, PolicyToken>) -> Result<String, LexicalError> {
    let source = lex.slice().to_string();
    match serde_json::from_str(&source.as_str()) {
        Ok(s) => {
            Ok(s)
        },
        Err(err) => {
            Err(LexicalError::InvalidString(err.to_string()))
        },
    }
}

fn integer_callback(lex: &mut Lexer<'_, PolicyToken>) -> Result<i64, LexicalError> {
    let source = lex.slice().to_string();
    match source.parse::<i64>() {
        Ok(i) => {
            Ok(i)
        },
        Err(err) => {
            Err(LexicalError::InvalidInteger(err))
        },
    }
}

fn double_callback(lex: &mut Lexer<'_, PolicyToken>) -> Result<f64, LexicalError> {
    let source = lex.slice().to_string();
    match source.parse::<f64>() {
        Ok(f) => {
            Ok(f)
        },
        Err(err) => {
            Err(LexicalError::InvalidDouble(err))
        },
    }
}


fn diduri_callback(lex: &mut Lexer<'_, PolicyToken>) -> Result<DidUri, LexicalError> {
    let lexeme = lex.slice(); // |lex| lex_url(lex.slice())
    let len = lexeme.len();
    if let Ok(uri) = DidUri::from_str(&lexeme[1..len-1]) {
        Ok(uri)
    } else {
        Err(LexicalError::InvalidDidUrl)
    }
}

fn comment_callback(lex: &mut Lexer<'_, PolicyToken>) -> Result<Skip, LexicalError> {
    let mut comment_lexer = lex.clone();
    let mut result: Result<Skip, LexicalError> = Err(LexicalError::UnterminatedComment);
    while let Some(token) = comment_lexer.next() {
        match token {
            Ok(PolicyToken::CommentEnd) => {
                result = Ok(Skip);
                break;
            },
            Ok(PolicyToken::Newline) => {
                lex.extras.0.0 += 1;
                lex.extras.0.1 = lex.span().end;
                lex.extras.1 = lex.extras.0;
                result = Ok(Skip);
                break;
            },
            Ok(_) => {},
            Err(x) => {
                result = Err(x);
                break;
            },
        }
    }
    *lex = comment_lexer;
    result
}

// Update the line count and the char index.
fn newline_callback(lex: &mut Lexer<PolicyToken>) -> Skip {
    lex.extras.0.0 += 1;
    lex.extras.0.1 = lex.span().end;
    lex.extras.1 = lex.extras.0;
    Skip
}


#[cfg(test)]
mod tests {
    use logos::Logos;
    use crate::data::uri::DidUri;
    use super::{PolicyLexer, PolicyToken, Position};

    #[test]
    fn test_lexer_comment() {
        let source = r#" /* 
        
        hello */
        
         1 "#;
        let mut lex = PolicyToken::lexer(source);
        let expected = 1;
        if let Some(Ok(PolicyToken::Integer(result))) = lex.next() {
            assert_eq!(expected, result);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_lexer_string() {
        let mut lex = PolicyToken::lexer(r#""hello \" \r \n \u123f ""#);
        let expected = "hello \" \r \n \u{123f} ";
        if let Some(Ok(PolicyToken::String(result))) = lex.next() {
            assert_eq!(expected, result);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_lexer_did() {
        let mut lex = PolicyToken::lexer("`did:policy:123`");
        let expected: DidUri = "did:policy:123".parse().unwrap();
        if let Some(Ok(PolicyToken::DidUri(uri))) = lex.next() {
            assert_eq!(expected, uri);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_lexer_location() {
        let code = r#"as
        
        begin
             = }
        "#;
        let mut pl = PolicyLexer::new(code);
        let mut i = 0;
        while let Some(Ok((start, token, end))) = pl.next() {
            println!("{start:?}, {end:?} {token:?}");
            match i {
                0 => {
                    assert_eq!(Position { line_num: 0, line_begin_char: 0, char_num: 0 }, start);
                    assert_eq!(Position { line_num: 0, line_begin_char: 0, char_num: 2 }, end);
                    assert_eq!(PolicyToken::As, token);
                    i += 1;
                },
                1 => {
                    assert_eq!(Position { line_num: 2, line_begin_char: 12, char_num: 20 }, start);
                    assert_eq!(Position { line_num: 2, line_begin_char: 12, char_num: 25 }, end);
                    assert_eq!(PolicyToken::Begin, token);
                    i += 1;
                },
                2 => {
                    assert_eq!(Position { line_num: 3, line_begin_char: 26, char_num: 39 }, start);
                    assert_eq!(Position { line_num: 3, line_begin_char: 26, char_num: 40 }, end);
                    assert_eq!(PolicyToken::Equal, token);
                    i += 1;
                },
                3 => {
                    assert_eq!(Position { line_num: 3, line_begin_char: 26, char_num: 41 }, start);
                    assert_eq!(Position { line_num: 3, line_begin_char: 26, char_num: 42 }, end);
                    assert_eq!(PolicyToken::RightBrace, token);
                    i += 1;
                },
                _ => { assert!(false); },
            }
        }
    }
}
