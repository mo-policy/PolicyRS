// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;

use crate::data::DidValue;
use crate::machine::resolver::DidDereferencer;
use crate::machine::rewrite_machine::{term_sort, RewriteContext, RewriteErr, RewriteMachine, RewriteOk };
use super::*;

pub fn std_did() -> Rc<DidValue> {
    let mut did_map: HashTrieMap<String, Rc<DidValue>> = HashTrieMap::new();
    did_map.insert_mut("id".to_string(), std_did_id());
    Rc::new(DidValue::Map(Sort::Constant, did_map)).as_hash()
}

fn std_did_id() -> Rc<DidValue> {
    // fun key -> std_did_id_rewrite
    // dereferences relative url with /key
    // if deref returns a constant, a did a policy did is returned
    fn std_did_id_rewrite(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        let Some(key_value) = context.bindings.try_map_value(&"key".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("'key' not bound in std_did_id".into())))
        };
        let Some(key) = key_value.try_string() else {
            return Result::Err(RewriteErr::Exception(Rc::new("'key' not string in std_did_id".into())))
        };
        let uri = key.parse().unwrap();
        let Some(deref_value) = machine.docket.dereference(&uri, Some(&context.base)) else {
            return Result::Ok(RewriteOk::Blocked(Rc::clone(&context.term)))
        };
        if term_sort(&deref_value) == Sort::Constant {
            let Some(hash_str) = deref_value.as_hash().try_hash_str() else {
                return Result::Err(RewriteErr::Exception(Rc::new("expected hash in std_did_id".into())))
            };
            let did = format!("did:policy:{hash_str}");
            let uri: DidUri = did.parse().unwrap();
            Result::Ok(RewriteOk::Term(Rc::new(DidValue::Uri(Box::new(uri)))))
        } else {
            Result::Ok(RewriteOk::Blocked(Rc::clone(&context.term)))
        }
    }
    rewrite_one(Rc::new(DidValue::Rewrite("std::did::id".to_string(), vec!["key".to_string()], std_did_id_rewrite)), "key")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rpds::{List, Vector};

    use super::*;
    use crate::machine::sorts::sort_dereference::DereferenceValue;
    use crate::machine::sorts::sort_list::ListItemValue;
    use crate::machine::sorts::sort_tuple::TupleValue;
    use crate::machine::uniform::apply_one;
    use crate::machine::rewrite_machine::rewrite_term;

    #[test]
    fn test_policy_did() {
        let std_document = policy_std();
        let std_did = std_document.try_document_did().unwrap();
        let id_term = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["did", "id"])), 
                Rc::new(DidValue::String("policy".into()))
            );
        let program = 
            TupleValue::create(
                Rc::new(
                    DidValue::Tuple(
                        Vector::new()
                        .push_back(ListItemValue::create(id_term, Rc::new(false.into())))
                    )
                )
            );
        let mut machine = RewriteMachine::new();
        let base = 
            DidValue::new_map_constant()
            .with_map_value("policy".to_string(), Rc::<DidValue>::new(1.into()).as_hash())
            .as_hash();
        let context = RewriteContext {
            base: Rc::clone(&base),
            term: program,
            bindings: DidValue::new_map_constant(),
            policies: Rc::new(List::new()),
        };
        machine.docket.insert_mut((PathBuf::new(), std_document));
        let result = rewrite_term(&mut machine, context);
        let expected_id_hash = Rc::<DidValue>::new(1.into()).as_hash().hash_str();
        let expected_id = format!("did:policy:{expected_id_hash}").parse().unwrap();
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    Rc::new(
                        DidValue::Tuple(
                            Vector::new()
                            .push_back(Rc::new(DidValue::Uri(Box::new(expected_id))))
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

}