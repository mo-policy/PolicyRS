// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

use super::sort_application::ApplicationValue;
pub struct LoopValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> LoopValue<'a> {
    pub fn pattern(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"pattern".to_string()).unwrap()) }
    pub fn iterator(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"iterator".to_string()).unwrap()) }
    pub fn term(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"term".to_string()).unwrap()) }
    pub fn tag(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"tag".to_string()).unwrap()) }

    pub fn create(pattern: Rc<DidValue>, iterator: Rc<DidValue>, term: Rc<DidValue>, tag: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("pattern".to_string(), pattern)
            .insert("iterator".to_string(), iterator)
            .insert("term".to_string(), term)
            .insert("tag".to_string(), tag);
        Rc::new(DidValue::Map(Sort::Loop, ht))
    }
}

pub fn rewrite_loop(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let loop_value = LoopValue { value: &context.term };
    let iterator_context = context.with_term(Rc::clone(&loop_value.iterator()));
    match rewrite_term(machine, iterator_context)? {
        RewriteOk::Term(iterator_result) => {
            let iterator_sort = term_sort(&iterator_result);
            if let Sort::Function = iterator_sort {
                // apply the function with null
                let app_iterator = 
                    ApplicationValue::create(
                        Rc::clone(&iterator_result), 
                        Rc::new(DidValue::Null)
                    );
                let app_context = context.with_term(app_iterator);
                loop {
                    match rewrite_term(machine, app_context.clone())? {
                        RewriteOk::Term(app_result) => {
                            // needs to return map with value and done.
                            let Some(app_map) = app_result.try_map() else {
                                return Result::Err(RewriteErr::Exception(Rc::new("Iterator function didn't return map.".into())));
                            };
                            let Some(dv) = app_map.get("done") else {
                                return Result::Err(RewriteErr::Exception(Rc::new("Iterator function didn't return map with done.".into())));
                            };
                            let Some(done) = dv.try_bool() else {
                                return Result::Err(RewriteErr::Exception(Rc::new("Iterator function didn't return map with done as boolean.".into())));
                            };
                            if done {
                                return Result::Ok(RewriteOk::Term(Rc::new(DidValue::Null)))
                            };
                            let Some(value) = app_map.get("value") else {
                                return Result::Err(RewriteErr::Exception(Rc::new("Iterator function didn't return map with value.".into())))
                            };
                            let bind_context = context.with_term(Rc::clone(&loop_value.pattern()));
                            match term_bind(machine, bind_context, Rc::clone(&value))? {
                                MatchOk::Bindings(loop_bindings) => {
                                    let term_context = 
                                        RewriteContext {
                                            base: Rc::clone(&context.base),
                                            term: Rc::clone(&loop_value.term()),
                                            bindings: loop_bindings,
                                            policies: Rc::clone(&context.policies),
                                        };
                                    match rewrite_term(machine, term_context)? {
                                        RewriteOk::Term(_) => { 
                                            // do nothing
                                        },
                                        RewriteOk::Blocked(term_blocked) => {
                                            return Result::Ok(
                                                RewriteOk::Blocked( 
                                                    LoopValue::create(
                                                        Rc::clone(&loop_value.pattern()), 
                                                        iterator_result, 
                                                        term_blocked,
                                                        loop_value.tag(),
                                                    ).as_hash()
                                                )
                                            );
                                        },
                                    }
                                },
                                MatchOk::NoMatch => {
                                    panic!("loop failed to bind")
                                },
                            }
                        },
                        RewriteOk::Blocked(app_blocked) => {
                            return Result::Ok(
                                RewriteOk::Blocked( 
                                    LoopValue::create(
                                        Rc::clone(&loop_value.pattern()), 
                                        iterator_result, 
                                        Rc::clone(&app_blocked),
                                        loop_value.tag(),
                                    ).as_hash()
                                )
                            );
                        },
                    }
                }
            } else {
                panic!("Loop iterator must be function")
            }            
        },
        RewriteOk::Blocked(iterator_blocked) => {
            Result::Ok(
                RewriteOk::Blocked(
                    loop_value.value.with_map_value(
                        "iterator".to_string(), 
                        iterator_blocked
                    ).as_hash()
                )
            )
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse::parse_str, machine::sorts::sort_function::FunctionValue};

    #[test]
    fn test_loop_iterator() {
        // loop 
        // pattern: null
        // iterator: fun null -> { done: true, value: null }
        // term: null
        let program = LoopValue::create(
            Rc::new(DidValue::Null), 
            FunctionValue::create(
                Rc::new(DidValue::Null), 
                Rc::new(DidValue::Map(
                    Sort::Constant,
                    HashTrieMap::new()
                    .insert("done".to_string(), Rc::new(true.into()))
                    .insert("value".to_string(), Rc::new(DidValue::Null))
                )), 
                None,
                None
            ), 
            Rc::new(DidValue::Null),
            Rc::new(DidValue::Null)
        );
        // null
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(DidValue::Null)));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_loop_while() {
        // "while" <_condiiton: AtomicTerm> "do" <_term: SequenceItems> "end"
        // let x = ref 1 in 
        //     while !x < 5 do x := !x + 1 end
        // end
        // pattern: null
        // iterator: fun null -> { done: !x < 5, value: null } 
        // term: x := !x + 1
        let source = r#"
            let x = ref 1 in 
                while ((!x) < 5) do x := ((!x) + 1) end
            end"#;
        let program = parse_str(source);
        // null
        let result = run_rewrite_with_std(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(DidValue::Null)));
        assert_eq!(expected, result);
    }


}