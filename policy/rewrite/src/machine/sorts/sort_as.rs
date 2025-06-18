// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

pub struct AsValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> AsValue<'a> {
    pub fn term(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"term".to_string()).unwrap()) }
    pub fn name(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"name".to_string()).unwrap()) }

    pub fn create(term: Rc<DidValue>, name: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("term".to_string(), term)
            .insert("name".to_string(), name);
        Rc::new(DidValue::Map(Sort::As, ht))
    }
}

pub fn rewrite_as(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let as_value = AsValue { value: &context.term };
    let term_context = context.with_term(Rc::clone(&as_value.term()));
    match rewrite_term(machine, term_context)? {
        RewriteOk::Term(term_result) => {
            Result::Ok(RewriteOk::Term(term_result))
        },
        RewriteOk::Blocked(term_blocked) => {
            Result::Ok(
                RewriteOk::Blocked(
                    as_value.value.with_map_value(
                        "term".to_string(), 
                        term_blocked
                    ).as_hash()
                )
            )
        },
    }
}

pub fn bind_as(machine: &mut RewriteMachine, context: RewriteContext, value: Rc<DidValue>) -> Result<MatchOk, RewriteErr> {
    let as_pattern = AsValue { value: &context.term };
    let term_pattern_context = context.with_term(as_pattern.term());
    match term_bind(machine, term_pattern_context, Rc::clone(&value)) {
        Result::Ok(MatchOk::Bindings(bindings)) => {
            let name = as_pattern.name().try_string().unwrap().clone();
            if name == "_" {
                Result::Ok(MatchOk::Bindings(Rc::clone(&bindings)))
            } else {
                Result::Ok(MatchOk::Bindings(bindings.with_map_value(name, Rc::clone(&value))))
            }
        },
        result => result,
    }
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use rpds::Vector;
    use crate::machine::sorts::{sort_lookup::LookupValue, sort_match::{MatchValue, RuleValue}};
    use super::*;

    #[test]
    fn test_as() {
        // 1 as x
        let program = AsValue::create(
            Rc::new(1.into()),
            Rc::new("x".into()),
        );
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(1.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_as_blocked() {
        // blocked as x
        let program = AsValue::create(
            LookupValue::create("blocked"),
            Rc::new("x".into()),
        );
        let result = run_rewrite(program.clone());
        let expected = Result::Ok(RewriteOk::Blocked(program));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_as_pattern() {
        // match 1 with | 1 as x -> x
        let program = 
            MatchValue::create(
                Rc::new(1.into()), 
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(
                            RuleValue::create(
                                AsValue::create(
                                    Rc::new(1.into()), 
                                    Rc::new("x".into())
                                ), 
                                Rc::new(DidValue::Null), 
                                LookupValue::create("x")
                            )
                        )
                    )
                )
    
            );
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(1.into())));
        assert_eq!(expected, result);
    }


}
