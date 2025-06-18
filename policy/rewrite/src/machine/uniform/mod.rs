// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;

use comparison::std_comparison;
use did::std_did;
use double::std_double;
use integer::std_integer;
use map::std_map;
use math::std_math;
use rpds::{HashTrieMap, HashTrieSet};
use string::std_string;

use crate::data::uri::{DidUri, DidUriPart, DidUriPartKind};
use crate::data::DidValue;
use crate::data::sort::Sort;
use crate::machine::rewrite_machine::RewriteContext;
use crate::machine::sorts::sort_application::ApplicationValue;
use crate::machine::sorts::sort_function::FunctionValue;
use crate::machine::sorts::sort_lookup::LookupValue;


pub mod comparison;
pub mod did;
pub mod double;
pub mod integer;
pub mod string;
pub mod map;
pub mod math;

pub enum UnaryArgs<T> {
    Args(T),
    Exception(&'static str),
}
pub enum BinaryArgs<T1, T2> {
    Args(T1, T2),
    Exception(String),
}

impl<T1, T2> BinaryArgs<T1, T2>
    where 
        T1: TryFrom<Rc<DidValue>>,
        T2: TryFrom<Rc<DidValue>>,
     {
    
    pub fn from_context(context: &RewriteContext, arg1: &str, arg2: &str) -> Self {
        let Some(arg1_value) = context.bindings.try_map_value(&arg1.to_string()) else {
            return BinaryArgs::Exception(format!("expected '{arg1}'"));
        };
        let Some(arg2_value) = context.bindings.try_map_value(&arg2.to_string()) else {
            return BinaryArgs::Exception(format!("expected '{arg2}'"));
        };
        if let Ok(arg1_typed) = T1::try_from(Rc::clone(arg1_value)) {
            if let Ok(arg2_typed) = T2::try_from(Rc::clone(arg2_value)) {
                return BinaryArgs::Args(arg1_typed, arg2_typed);
            } else {
                return BinaryArgs::Exception(format!("wrong type for '{arg2}'"));
            }
        } else {
            return BinaryArgs::Exception(format!("wrong type for '{arg1}'"));
        }
    }
}

pub enum TupleArgs<T0, T1> {
    Args(T0, T1),
    Exception(&'static str),
}

pub enum TripleArgs<T0, T1, T2> {
    Args(T0, T1, T2),
    Exception(&'static str),
}

pub fn unary_op(rewrite: Rc<DidValue>) -> Rc<DidValue> {
    FunctionValue::create(
        LookupValue::create("lhs"),
        rewrite, 
        None,
        None
    )
}

pub fn rewrite_one(rewrite: Rc<DidValue>, arg1: &str) -> Rc<DidValue> {
    FunctionValue::create(
        LookupValue::create(arg1),
        rewrite, 
        None,
        None
    )
}

pub fn rewrite_two(rewrite: Rc<DidValue>, arg1: &str, arg2: &str) -> Rc<DidValue> {
    FunctionValue::create(
        LookupValue::create(arg1),
        FunctionValue::create(
            LookupValue::create(arg2), 
            rewrite, 
            None,
            None
        ), 
        None,
        None
    )
}

pub fn apply_one(op: Rc<DidValue>, p0: Rc<DidValue>) -> Rc<DidValue> {
    ApplicationValue::create(
        op, 
        p0
    )
}

pub fn apply_two(op: Rc<DidValue>, p0: Rc<DidValue>, p1: Rc<DidValue>) -> Rc<DidValue> {
    ApplicationValue::create(
        ApplicationValue::create(
            op, 
            p0
        ), 
        p1
    )
}

pub fn apply_three(op: Rc<DidValue>, p0: Rc<DidValue>, p1: Rc<DidValue>, p2: Rc<DidValue>) -> Rc<DidValue> {
    ApplicationValue::create(
        ApplicationValue::create(
            ApplicationValue::create(
                op, 
                p0
            ), 
            p1
        ), 
        p2
    )
}

pub fn policy_url(uri: &DidUri, path: Vec<&str>) -> Rc<DidValue> {
    let mut did = uri.get_did().unwrap();
    let last_part = did.parts.last().unwrap();
    let mut start = last_part.start + (last_part.len as u16);
    did.text.push_str("/policy");
    let policy_part = 
        DidUriPart { 
            kind: DidUriPartKind::Path, 
            start, 
            len: 7,
        };
    did.parts.push(policy_part);        
    start += 7;
    for s in path {
        did.text.push_str(&"/");
        did.text.push_str(s);
        let part = 
            DidUriPart { 
                kind: DidUriPartKind::Path, 
                start, 
                len: (s.len() as u8) + 1,
            };
        start = part.start + (part.len as u16);
        did.parts.push(part);
    }
    Rc::new(
        DidValue::Uri(
            Box::new(did)
        )
    )
}

pub fn policy_std() -> Rc<DidValue> {
    /*
    {
        id: "did:policy:123",
        alsoKnownAs: "did:policy:std",
        policy: {
            comparison: {
                eq: fun lhs -> fun rhs -> #(lhs.eq(rhs)),
                ...
            },
            double: {
                min: ...
            }
        }
    }
    */
    let std_aka_id: DidUri = "did:policy:std".parse().unwrap();
    let std_aka = 
        Rc::new(
            DidValue::Set(
                HashTrieSet::new().insert(
                    Rc::new(DidValue::Uri(Box::new(std_aka_id.clone())))
                )
            )
        );

    let mut policy_map: HashTrieMap<String, Rc<DidValue>> = HashTrieMap::new();
    policy_map.insert_mut("comparison".to_string(), std_comparison());
    policy_map.insert_mut("did".to_string(), std_did());
    policy_map.insert_mut("double".to_string(), std_double());
    policy_map.insert_mut("integer".to_string(), std_integer());
    policy_map.insert_mut("string".to_string(), std_string());
    policy_map.insert_mut("map".to_string(), std_map());
    policy_map.insert_mut("math".to_string(), std_math());
    let policy_value = Rc::new(DidValue::Map(Sort::Constant, policy_map)).as_hash();
    policy_value.as_policy_did(Some(std_aka))
}
