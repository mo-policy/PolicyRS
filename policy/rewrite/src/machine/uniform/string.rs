// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use crate::data::DidValue;
use crate::machine::rewrite_machine::{RewriteContext, RewriteErr, RewriteMachine, RewriteOk };
use super::*;

pub fn std_string() -> Rc<DidValue> {
    let mut string_map: HashTrieMap<String, Rc<DidValue>> = HashTrieMap::new();
    string_map.insert_mut("len".to_string(), std_string_len());
    string_map.insert_mut("trim".to_string(), std_string_trim());
    string_map.insert_mut("trim_start".to_string(), std_string_trim_start());
    string_map.insert_mut("trim_end".to_string(), std_string_trim_end());
    string_map.insert_mut("to_lowercase".to_string(), std_string_to_lowercase());
    string_map.insert_mut("to_uppercase".to_string(), std_string_to_uppercase());
    Rc::new(DidValue::Map(Sort::Constant, string_map)).as_hash()
}


fn string_unary_args(context: &RewriteContext) -> UnaryArgs<&String> {
    let Some(lhs) = context.bindings.try_map_value(&"lhs".to_string()) else {
        return UnaryArgs::Exception(&"expected 'lhs'");
    };
    let Some(lhs_string) = lhs.try_string() else {
        return UnaryArgs::Exception(&"expected 'lhs' as string");
    };
    UnaryArgs::Args(lhs_string)
}

fn std_string_len() -> Rc<DidValue> {
    // fun lhs -> lhs.len()
    fn std_string_len_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match string_unary_args(&context) {
            UnaryArgs::Args(lhs) => {
                let result = lhs.len() as i64;
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            UnaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    unary_op(Rc::new(DidValue::Rewrite("std::string::len".to_string(), vec!["lhs".to_string()], std_string_len_rewrite)))
}

fn std_string_trim() -> Rc<DidValue> {
    // fun lhs -> lhs.trim()
    fn std_string_trim_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match string_unary_args(&context) {
            UnaryArgs::Args(lhs) => {
                let result = lhs.trim();
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            UnaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    unary_op(Rc::new(DidValue::Rewrite("std::string::trim".to_string(), vec!["lhs".to_string()], std_string_trim_rewrite)))
}

fn std_string_trim_start() -> Rc<DidValue> {
    // fun lhs -> lhs.trim_start)
    fn std_string_trim_start_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match string_unary_args(&context) {
            UnaryArgs::Args(lhs) => {
                let result = lhs.trim_start();
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            UnaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    unary_op(Rc::new(DidValue::Rewrite("std::string::trim_start".to_string(), vec!["lhs".to_string()], std_string_trim_start_rewrite)))
}

fn std_string_trim_end() -> Rc<DidValue> {
    // fun lhs -> lhs.trim_end)
    fn std_string_trim_end_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match string_unary_args(&context) {
            UnaryArgs::Args(lhs) => {
                let result = lhs.trim_end();
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            UnaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    unary_op(Rc::new(DidValue::Rewrite("std::string::trim_end".to_string(), vec!["lhs".to_string()], std_string_trim_end_rewrite)))
}

fn std_string_to_lowercase() -> Rc<DidValue> {
    // fun lhs -> lhs.to_lowercase)
    fn std_string_to_lowercase_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match string_unary_args(&context) {
            UnaryArgs::Args(lhs) => {
                let result = lhs.to_lowercase();
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            UnaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    unary_op(Rc::new(DidValue::Rewrite("std::string::to_lowercase".to_string(), vec!["lhs".to_string()], std_string_to_lowercase_rewrite)))
}

fn std_string_to_uppercase() -> Rc<DidValue> {
    // fun lhs -> lhs.to_uppercase)
    fn std_string_to_uppercase_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match string_unary_args(&context) {
            UnaryArgs::Args(lhs) => {
                let result = lhs.to_uppercase();
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            UnaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    unary_op(Rc::new(DidValue::Rewrite("std::string::to_uppercase".to_string(), vec!["lhs".to_string()], std_string_to_uppercase_rewrite)))
}

// pub fn contains<P>(&self, pat: P) -> bool
// pub fn find<P>(&self, pat: P) -> Option<usize>
// pub fn rfind<P>(&self, pat: P) -> Option<usize>
// pub fn replace<P>(&self, from: P, to: &str) -> String
// Add
// 

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rpds::{List, Vector};

    use super::*;
    use crate::machine::sorts::sort_dereference::DereferenceValue;
    use crate::machine::sorts::sort_list::ListItemValue;
    use crate::machine::sorts::sort_tuple::TupleValue;
    use crate::machine::rewrite_machine::rewrite_term;

    #[test]
    fn test_policy_integer() {
        let std_document = policy_std();
        let std_did = std_document.try_document_did().unwrap();
        let len_term = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["string", "len"])), 
                Rc::new(DidValue::String("hello".to_string()))
            );
        let trim_term = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["string", "trim"])), 
                Rc::new(DidValue::String("  hello    ".to_string()))
            );
        let trim_start_term = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["string", "trim_start"])), 
                Rc::new(DidValue::String("  hello    ".to_string()))
            );
        let trim_end_term = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["string", "trim_end"])), 
                Rc::new(DidValue::String("  hello    ".to_string()))
            );
        let to_lowercase_term = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["string", "to_lowercase"])), 
                Rc::new(DidValue::String("HELLO".to_string()))
            );
        let to_uppercase_term = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["string", "to_uppercase"])), 
                Rc::new(DidValue::String("hello".to_string()))
            );
        let program = 
            TupleValue::create(
                Rc::new(
                    DidValue::Tuple(
                        Vector::new()
                        .push_back(ListItemValue::create(len_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(trim_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(trim_start_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(trim_end_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(to_lowercase_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(to_uppercase_term, Rc::new(false.into())))
                    )
                )
            );
        let mut machine = RewriteMachine::new();
        let context = RewriteContext {
            base: Rc::clone(&program),
            term: program,
            bindings: DidValue::new_map_constant(),
            policies: Rc::new(List::new()),
        };
        machine.docket.insert_mut((PathBuf::new(), std_document));
        let result = rewrite_term(&mut machine, context);
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    Rc::new(
                        DidValue::Tuple(
                            Vector::new()
                            .push_back(Rc::new(DidValue::Integer(5)))
                            .push_back(Rc::new(DidValue::String("hello".to_string())))
                            .push_back(Rc::new(DidValue::String("hello    ".to_string())))
                            .push_back(Rc::new(DidValue::String("  hello".to_string())))
                            .push_back(Rc::new(DidValue::String("hello".to_string())))
                            .push_back(Rc::new(DidValue::String("HELLO".to_string())))
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

}