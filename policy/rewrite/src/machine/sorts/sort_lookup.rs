// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

pub struct LookupValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> LookupValue<'a> {
    pub fn name(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"name".to_string()).unwrap()) }

    pub fn create(name: &str) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("name".to_string(), Rc::new(name.into()));
        Rc::new(DidValue::Map(Sort::Lookup, ht))
    }
}

pub fn rewrite_lookup(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let lookup = LookupValue { value: &context.term };
    match context.bindings.try_map_value(lookup.name().try_string().unwrap()) {
        Some(bound_term) => {
            let bound_sort = term_sort(&bound_term);
            if let Sort::Fix = bound_sort {
                let fix_context = context.with_term(Rc::clone(bound_term));
                rewrite_term(machine, fix_context)
            } else {
                Result::Ok(RewriteOk::Term(Rc::clone(bound_term)))
            }
        },
        None => {
            Result::Ok(RewriteOk::Blocked(context.term))
        },
    }
}

pub fn bind_lookup(_machine: &mut RewriteMachine, context: RewriteContext, value: Rc<DidValue>) -> Result<MatchOk, RewriteErr> {
    let lookup = LookupValue { value: &context.term };
    let key = lookup.name().try_string().unwrap().clone();
    if key == "_" {
        Result::Ok(MatchOk::Bindings(Rc::clone(&context.bindings)))
    } else {
        Result::Ok(MatchOk::Bindings(context.bindings.with_map_value(key, Rc::clone(&value))))
    }
}

#[cfg(test)]
mod tests {
    use rpds::List;

    use super::*;

    #[test]
    fn test_lookup() {
        let program = LookupValue::create("x");
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new().insert("x".to_string(), Rc::new(1.into()));
        let context = RewriteContext {
            base: Rc::clone(&program),
            term: program,
            bindings: Rc::new(DidValue::Map(Sort::Constant, ht)),
            policies: Rc::new(List::new()),
        };
        let mut machine = RewriteMachine::new();
        let result = rewrite_term(&mut machine, context);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(1.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_lookup_blocked() {
        // blocked
        let program = LookupValue::create("blocked");
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new().insert("x".to_string(), Rc::new(1.into()));
        let context = RewriteContext {
            base: Rc::clone(&program),
            term: program,
            bindings: Rc::new(DidValue::Map(Sort::Constant, ht)),
            policies: Rc::new(List::new()),
        };
        let mut machine = RewriteMachine::new();
        let result = rewrite_term(&mut machine, context);
        let expected = Result::Ok(RewriteOk::Blocked(LookupValue::create("blocked")));
        assert_eq!(expected, result);
    }
}