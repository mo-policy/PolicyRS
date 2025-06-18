// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use crate::data::DidValue;
use crate::machine::rewrite_machine::{RewriteContext, RewriteErr, RewriteMachine, RewriteOk };
use super::*;

pub fn std_map() -> Rc<DidValue> {
    let mut map_map: HashTrieMap<String, Rc<DidValue>> = HashTrieMap::new();
    map_map.insert_mut("get".to_string(), std_map_get());
    Rc::new(DidValue::Map(Sort::Constant, map_map)).as_hash()
}

fn std_map_get() -> Rc<DidValue> {
    // fun map -> fun key -> map.get(key)
    fn std_map_get_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        if let Some(map_binding) = context.bindings.try_map_value(&"map".to_string()) {
            if let Some(key_binding) = context.bindings.try_map_value(&"key".to_string()) {
                if let Some(map) = map_binding.try_map() {
                    if let Some(key) = key_binding.try_string() {
                        if let Some(value) = map.get(key) {
                            Result::Ok(RewriteOk::Term(Rc::clone(value)))
                        } else {
                            Result::Err(RewriteErr::Exception(Rc::new("'key' not found in 'map'".into())))
                        }
                    } else {
                        Result::Err(RewriteErr::Exception(Rc::new("'key' not string type".into())))
                    }
                } else {
                    Result::Err(RewriteErr::Exception(Rc::new("'map' not map type".into())))
                }
            } else {
                Result::Err(RewriteErr::Exception(Rc::new("missing arg, 'key'".into())))
            }
        } else {
            Result::Err(RewriteErr::Exception(Rc::new("missing arg, 'map'".into())))
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::map::get".to_string(), vec!["map".to_string(), "key".to_string()], std_map_get_rewrite)), "map", "key")

}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rpds::{List, Vector};

    use super::*;
    use crate::machine::sorts::sort_dereference::DereferenceValue;
    use crate::machine::sorts::sort_list::ListItemValue;
    use crate::machine::sorts::sort_tuple::TupleValue;
    use crate::machine::uniform::apply_two;
    use crate::machine::rewrite_machine::rewrite_term;

    #[test]
    fn test_policy_map() {
        let std_document = policy_std();
        let std_did = std_document.try_document_did().unwrap();
        let map_value = 
            DidValue::new_map_constant()
            .with_map_value("x".to_string(), Rc::new(1.into()));
        let get_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["map", "get"])), 
                map_value, 
                Rc::new("x".into())
            );
        let program = 
            TupleValue::create(
                Rc::new(
                    DidValue::Tuple(
                        Vector::new()
                        .push_back(ListItemValue::create(get_term, Rc::new(false.into())))
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
                            .push_back(Rc::new(DidValue::Integer(1)))
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

}