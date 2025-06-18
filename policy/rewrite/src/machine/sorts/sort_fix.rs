// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

use super::sort_function::FunctionValue;
pub struct FixValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> FixValue<'a> {
    pub fn term(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"term".to_string()).unwrap()) }

    pub fn create(term: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("term".to_string(), term);
        Rc::new(DidValue::Map(Sort::Fix, ht))
    }
}

pub fn rewrite_fix(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let fix_value = FixValue { value: &context.term };
    let fix_term = &fix_value.term();
    let sort = term_sort(fix_term);
    if let Sort::Function = sort {
        let function_value = FunctionValue { value: fix_term };
        let pattern_context = context.with_term(Rc::clone(&function_value.pattern()));
        match term_bind(machine, pattern_context, Rc::clone(&context.term))? {
            MatchOk::Bindings(fix_bindings) => {
                let term_context = 
                    RewriteContext {
                        base: Rc::clone(&context.base),
                        term: Rc::clone(&function_value.term()),
                        bindings: fix_bindings,
                        policies: Rc::clone(&context.policies),
                    };
                rewrite_term(machine, term_context)
            },
            MatchOk::NoMatch => unreachable!("unexpected result"),
        }
    } else {
        panic!("fix only for functions")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    use sort_lookup::LookupValue;

    #[test]
    fn test_fix_normal() {
        // fix(fun x -> 1) => [x: fix(fun x -> 1)] 1
        let program = 
            FixValue::create(
                FunctionValue::create(
                        LookupValue::create("x"),
                        Rc::new(1.into()),
                        None,
                        None
                )
            );
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(1.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_fix_blocked() {
        // fix(fun x -> blocked) => [x: fix(fun x -> blocked)] blocked
        let program = 
            FixValue::create(
                FunctionValue::create(
                        LookupValue::create("x"),
                        LookupValue::create("blocked"),
                        None,
                        None
                )
            );
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Blocked(LookupValue::create("blocked")));
        assert_eq!(expected, result);
    }
}