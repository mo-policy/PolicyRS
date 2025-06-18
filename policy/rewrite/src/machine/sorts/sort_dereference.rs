// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::machine::resolver::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

pub struct DereferenceValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> DereferenceValue<'a> {
    pub fn term(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"term".to_string()).unwrap()) }

    pub fn create(term: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("term".to_string(), term);
        Rc::new(DidValue::Map(Sort::Dereference, ht))
    }
}

pub fn rewrite_dereference(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let dereference_value = DereferenceValue { value: &context.term };
    let term_context = context.with_term(Rc::clone(&dereference_value.term()));
    match rewrite_term(machine, term_context)? {
        RewriteOk::Term(term_result) => {
            let Some(uri) = term_result.try_uri() else {
                return Result::Err(RewriteErr::Exception(Rc::new("dereference term not url".into())));
            };
            if let Some(value) = machine.docket.dereference(uri, None) {
                Result::Ok(RewriteOk::Term(value.as_hash()))
            } else {
                Result::Ok(
                    RewriteOk::Blocked(
                        dereference_value.value.with_map_value(
                            "term".to_string(), 
                            term_result
                        ).as_hash()
                    )
                )
            }
        },
        RewriteOk::Blocked(term_blocked) => {
            Result::Ok(
                RewriteOk::Blocked(
                    dereference_value.value.with_map_value(
                        "term".to_string(), 
                        term_blocked
                    ).as_hash()
                )
            )
        },
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rpds::List;
    use crate::data::uri::DidUri;

    use super::*;

    #[test]
    fn test_resolve_did() {
        // resolve "did:policy:std:2397923/policy"
        let uri: DidUri = "did:policy:std:2397923/policy".parse().unwrap();
        let program = 
            DereferenceValue::create(
                Rc::new(DidValue::Uri(Box::new(uri.clone())))
            );
        let mut machine = RewriteMachine::new();
        let value = 
            DidValue::new_map_constant()
            .with_map_value("id".to_string(), Rc::new(DidValue::Uri(Box::new(uri.clone()))))
            .with_map_value("policy".to_string(), Rc::new(1.into()));
        machine.docket.insert_mut((PathBuf::new(), value));
        let context = RewriteContext {
            base: Rc::clone(&program),
            term: program,
            bindings: DidValue::new_map_constant(),
            policies: Rc::new(List::new()),
        };
        let result = rewrite_term(&mut machine, context);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(1.into())));
        assert_eq!(expected, result);
    }

}
