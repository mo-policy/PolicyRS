// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;

use grammar::{ConstantParser, TermParser};
use lexer::PolicyLexer;
use crate::data::DidValue;

pub mod actions;
pub mod export;
pub mod grammar;
pub mod lexer;

pub fn parse_str(source: &str) -> Rc<DidValue> {
    let lexer = PolicyLexer::new(&source);
    let parser =  TermParser::new();
    parser.parse(lexer).unwrap()
}

pub fn parse_constant_str(source: &str) -> Rc<DidValue> {
    let lexer = PolicyLexer::new(&source);
    let parser =  ConstantParser::new();
    parser.parse(lexer).unwrap()
}


