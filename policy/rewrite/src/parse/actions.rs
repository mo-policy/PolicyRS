// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;

use rpds::{HashTrieMap, Vector};
use crate::data::DidValue;
use crate::data::sort::Sort;
use crate::machine::sorts::sort_application::ApplicationValue;
use crate::machine::sorts::sort_dereference::DereferenceValue;
use crate::machine::sorts::sort_function::FunctionValue;
use crate::machine::sorts::sort_if::IfValue;
use crate::machine::sorts::sort_loop::LoopValue;
use crate::machine::sorts::sort_map::{MapItemValue, MapValue};

pub fn map_constant(item_vec: Vec<(String, Rc<DidValue>)>, last: Option<(String, Rc<DidValue>)>) -> Rc<DidValue> {
    let mut members: HashTrieMap<String, Rc<DidValue>> = HashTrieMap::new();
    for (key, value) in &item_vec {
        members.insert_mut(key.clone(), Rc::clone(value));
    }
    if let Some((key, value)) = &last {
        members.insert_mut(key.clone(), Rc::clone(value));
    }
    Rc::new(DidValue::Map(Sort::Constant, members))
}

pub fn list_constant(item_vec: Vec<Rc<DidValue>>, last: Option<Rc<DidValue>>) -> Rc<DidValue> {
    let mut items: Vector<Rc<DidValue>> = Vector::new();
    for item in &item_vec {
        items.push_back_mut(Rc::clone(item));
    }
    if let Some(item) = &last {
        items.push_back_mut(Rc::clone(item));
    }
    Rc::new(DidValue::List(items))
}

pub fn member_lookup(term: Rc<DidValue>, ident: Rc<DidValue>) -> Rc<DidValue> {
    // ((@`did:policy:std/policy/map/get`) term ident
    ApplicationValue::create(
        ApplicationValue::create(
            DereferenceValue::create(
                Rc::new(DidValue::Uri(Box::new("did:policy:std/policy/map/get".parse().unwrap())))
            ),
            term
        ), 
        ident
    )
}

pub fn lazy_boolean_or(left: Rc<DidValue>, right: Rc<DidValue>) -> Rc<DidValue> {
    // left || right, if left then true else right
    IfValue::create(left, Rc::new(true.into()), right)
}

pub fn lazy_boolean_and(left: Rc<DidValue>, right: Rc<DidValue>) -> Rc<DidValue> {
    // left && right, if left then right else false
    IfValue::create(left, right, Rc::new(false.into()))
}

pub fn while_loop(condition: Rc<DidValue>, term: Rc<DidValue>) -> Rc<DidValue> {
    // pattern: null
    // iterator: fun null -> { done: condition = false, value: null } 
    // term: term
    LoopValue::create(
        Rc::new(DidValue::Null), 
        FunctionValue::create(
            Rc::new(DidValue::Null), 
            MapValue::create(
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(MapItemValue::create(
                            Rc::new("done".into()), 
                            apply_not(condition),
                            Rc::new(false.into())))
                        .push_back(MapItemValue::create(
                            Rc::new("value".into()), 
                            Rc::new(DidValue::Null), 
                            Rc::new(false.into())))
                    )
                )
            ), 
            None, 
            None
        ), 
        term, 
        Rc::new("while".into())
    )
}

pub fn apply_infix(op: &str, left: Rc<DidValue>, right: Rc<DidValue>) -> Rc<DidValue> {
    let op_url = 
        match op {
            "<" => "did:policy:std/policy/comparison/lt",
            ">" => "did:policy:std/policy/comparison/gt",
            "<=" => "did:policy:std/policy/comparison/le",
            ">=" => "did:policy:std/policy/comparison/ge",
            "=" => "did:policy:std/policy/comparison/eq",
            "<>" => "did:policy:std/policy/comparison/ne",
            "+" => "did:policy:std/policy/math/add",
            "-" => "did:policy:std/policy/math/sub",
            "*" => "did:policy:std/policy/math/mul",
            "/" => "did:policy:std/policy/math/div",
            "%" => "did:policy:std/policy/math/rem",
            _ => panic!("unexpected op")
        };
    // op left right
    ApplicationValue::create(
        ApplicationValue::create(
            DereferenceValue::create(Rc::new(DidValue::Uri(Box::new(op_url.parse().unwrap())))), 
            left
        ), 
        right
    )
}

pub fn apply_not(term: Rc<DidValue>) -> Rc<DidValue> {
    let not_url = "did:policy:std/policy/comparison/not";
    ApplicationValue::create(
        DereferenceValue::create(Rc::new(DidValue::Uri(Box::new(not_url.parse().unwrap())))), 
        term
    )
}
