// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;
pub struct EvalValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> EvalValue<'a> {
    pub fn sort(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"sort".to_string()).unwrap()) }
    pub fn term(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"term".to_string()).unwrap()) }

    pub fn create(sort: String, term: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("sort".to_string(), Rc::new(sort.into()))
            .insert("term".to_string(), term);
        Rc::new(DidValue::Map(Sort::Eval, ht))
    }
}

pub fn rewrite_eval(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let eval_value = EvalValue { value: &context.term };
    if let Some(sort_name) = eval_value.sort().try_string() {
        let sort = Sort::try_from(sort_name.to_string()).map_err(|_| RewriteErr::Exception(Rc::new("bad sort".into())))?;
        let term_context = context.with_term(Rc::clone(&eval_value.term()));
        match rewrite_term(machine, term_context)? {
            RewriteOk::Term(term_result) => {
                if let Some(result_map) = term_result.try_map() {
                    let eval_term = Rc::new(DidValue::Map(sort, result_map.clone()));
                    let eval_context = context.with_term(eval_term);
                    rewrite_term(machine, eval_context)
                } else {
                    if let Sort::Constant = sort {
                        let eval_context = context.with_term(term_result);
                        rewrite_term(machine, eval_context)
                    } else {
                        Result::Err(RewriteErr::Exception(Rc::new("expected constant".into())))
                    }
                }                
            },
            RewriteOk::Blocked(term_blocked) => {
                Result::Ok(
                    RewriteOk::Blocked(
                        eval_value.value.with_map_value(
                            "term".to_string(), 
                            term_blocked
                        ).as_hash()
                    )
                )
            },
        }
    } else {
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    use sort_lookup::LookupValue;
    use sort_let::LetValue;
    use sort_map::{MapItemValue, MapValue};
    use rpds::Vector;

    #[test]
    fn test_eval() {
        // let x = 1 in {= { $policy: "Lookup", name: "x" } =}
        let program = 
            LetValue::create(
                false, 
                LookupValue::create("x"), 
                Rc::new(1.into()), 
                EvalValue::create(
                    "Lookup".to_string(),
                    MapValue::create(
                        Rc::new(
                            DidValue::List(
                                Vector::new()
                                .push_back(MapItemValue::create(Rc::new("name".into()), Rc::new("x".into()), Rc::new(false.into())))
                            )
                        )
                    )
                )
            );
        // 1
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(1.into())));
        assert_eq!(expected, result);
    }

}