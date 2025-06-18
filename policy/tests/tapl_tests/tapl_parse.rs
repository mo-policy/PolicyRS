// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;

use crate::tapl_common::*;
use rewrite::parse::lexer::PolicyLexer;
use rewrite::parse::grammar::TermParser;
use rewrite::data::DidValue;

const TRU: &str = "fun t -> fun f -> t";
const FLS: &str = "fun t -> fun f -> f";

fn parse(source: &str) -> Rc<DidValue> {
    let lexer = PolicyLexer::new(&source);
    let parser =  TermParser::new();
    parser.parse(lexer).unwrap()
}

#[test]
fn test_parse_tapl_tru() {
    let data = tapl_tru();
    let source = TRU;
    let parsed = parse(source);
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_fls() {
    let data = tapl_fls();
    let source = FLS;
    let parsed = parse(source);
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_test() {
    // fun l -> fun m -> fun n -> (l m) n
    let data = tapl_test();
    let source = "fun l -> fun m -> fun n -> (l m) n";
    let parsed = parse(source);
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_and() {
    // fun b -> fun c -> (b c) fls
    let data = tapl_and();
    let source = format!("fun b -> fun c -> (b c) ({FLS})");
    let parsed = parse(source.as_str());
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_or() {
    // fun b -> fun c -> b tru c
    let data = tapl_or();
    let source = format!("fun b -> fun c -> b ({TRU}) c");
    let parsed = parse(source.as_str());
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_not() {
    // fun b -> b fls tru
    let data = tapl_not();
    let source = format!("fun b -> b ({FLS}) ({TRU})");
    let parsed = parse(source.as_str());
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_pair() {
    // fun f -> fun s -> fun b -> b f s
    let data = tapl_pair();
    let source = "fun f -> fun s -> fun b -> b f s";
    let parsed = parse(source);
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_fst() {
    // fun p -> p tru
    let data = tapl_fst();
    let source = format!("fun p -> p ({TRU})");
    let parsed = parse(source.as_str());
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_snd() {
    // fun p -> p fls
    let data = tapl_snd();
    let source = format!("fun p -> p ({FLS})");
    let parsed = parse(source.as_str());
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_scc() {
    // fun n -> fun s -> fun z -> s (n s z)
    let data = tapl_scc();
    let source = "fun n -> fun s -> fun z -> s (n s z)";
    let parsed = parse(source);
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_plus() {
    // fun m -> fun n -> fun s -> fun z -> m s (n s z)
    let data = tapl_plus();
    let source = "fun m -> fun n -> fun s -> fun z -> m s (n s z)";
    let parsed = parse(source);
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_times() {
    // fun m -> fun n -> m (plus n) c0
    let data = tapl_times();
    let plus = "fun m -> fun n -> fun s -> fun z -> m s (n s z)";
    let source = format!("fun m -> fun n -> m (({plus}) n) ({FLS})");
    let parsed = parse(source.as_str());
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_iszro() {
    // fun m -> m (fun x -> fls) tru
    let data = tapl_iszro();
    let source = format!("fun m -> m (fun x -> ({FLS})) ({TRU})");
    let parsed = parse(source.as_str());
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_zz() {
    // pair c0 c0
    let data = tapl_zz();
    let pair = "fun f -> fun s -> fun b -> b f s";
    let source = format!("({pair}) ({FLS}) ({FLS})");
    let parsed = parse(source.as_str());
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_realbool() {
    // fun b -> b true false
    let data = tapl_realbool();
    let source = "fun b -> b true false";
    let parsed = parse(&source);
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_churchbool() {
    // fun b -> if b then tru else fls
    let data = tapl_churchbool();
    let source = format!("fun b -> if b then ({TRU}) else ({FLS})");
    let parsed = parse(source.as_str());
    assert_eq!(data, parsed);
}

#[test]
fn test_parse_tapl_fix() {
    // fun f -> (fun x -> f (fun y -> x x y)) (fun x -> f (fun y -> x x y))
    let data = tapl_fix();
    let source = "fun f -> (fun x -> f (fun y -> x x y)) (fun x -> f (fun y -> x x y))";
    let parsed = parse(&source);
    assert_eq!(data, parsed);
}
