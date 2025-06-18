// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::{data::sort::Sort, machine::rewrite_machine::*};
use crate::data::DidValue;

use super::sort_match::{rewrite_rules, RulesOk};
pub struct TryValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> TryValue<'a> {
    pub fn term(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"term".to_string()).unwrap()) }
    pub fn rules(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"rules".to_string()).unwrap()) }

    pub fn create(term: Rc<DidValue>, rules: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("term".to_string(), term)
            .insert("rules".to_string(), rules);
        Rc::new(DidValue::Map(Sort::Try, ht))
    }
}

pub fn rewrite_try(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let try_value = TryValue { value: &context.term };
    let term_context = context.with_term(Rc::clone(&try_value.term()));
    match rewrite_term(machine, term_context) {
        Result::Err(RewriteErr::Exception(term_exception)) => {
            let rules_context = context.with_term(Rc::clone(&term_exception));
            match rewrite_rules(machine, rules_context, try_value.rules())? {
                RulesOk::MatchedRule(rewrite_context) => {
                    rewrite_term(machine, rewrite_context)
                },
                RulesOk::BlockedGuard(blocked_rules) => {
                    Result::Ok(
                        RewriteOk::Blocked(
                            TryValue::create(
                                ThrowValue::create(term_exception), 
                                blocked_rules
                            ).as_hash()
                        )
                    )
                },
                RulesOk::NoRuleMatched => Result::Err(RewriteErr::Exception(term_exception)),
            }
        },
        Result::Ok(RewriteOk::Blocked(term_blocked)) => {
            Result::Ok(
                RewriteOk::Blocked(
                    TryValue::create(term_blocked, try_value.rules()).as_hash()
                )
            )
        },
        result => result,
    }
}

pub struct ThrowValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> ThrowValue<'a> {
    pub fn description(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"description".to_string()).unwrap()) }

    pub fn create(description: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("description".to_string(), description);
        Rc::new(DidValue::Map(Sort::Throw, ht))
    }
}

pub fn rewrite_throw(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let throw_value = ThrowValue { value: &context.term };
    let description_context = context.with_term(Rc::clone(&throw_value.description()));
    match rewrite_term(machine, description_context)? {
        RewriteOk::Term(description_result) => {
            Result::Err(RewriteErr::Exception(description_result))
        },
        RewriteOk::Blocked(description_blocked) => {
            Result::Ok(RewriteOk::Blocked(ThrowValue::create(description_blocked)))
        },
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    use sort_lookup::LookupValue;
    use sort_match::RuleValue;
    use rpds::Vector;

    #[test]
    fn test_throw() {
        // throw "error"
        let program = ThrowValue::create(Rc::new("error".into()));
        // throw "error"
        let result = run_rewrite(program);
        let expected = Result::Err(RewriteErr::Exception(Rc::new("error".into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_try_catch_no_exception() {
        // try 1 with | x -> x
        let program = 
            TryValue::create(
                Rc::new(1.into()), 
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(
                            RuleValue::create(
                                LookupValue::create("x"), 
                                Rc::new(DidValue::Null), 
                                LookupValue::create("x")
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

    #[test]
    fn test_try_catch_with_exception() {
        // try throw "error" with | x -> x
        let program = 
            TryValue::create(
                ThrowValue::create(Rc::new("error".into())), 
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(
                            RuleValue::create(
                                LookupValue::create("x"), 
                                Rc::new(DidValue::Null), 
                                LookupValue::create("x")
                            )
                        )
                    )
                )
            );
        // "error"
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new("error".into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_nexted_try_catch_with_outer_exception() {
        // try try throw "error" with | 1 -> 1 with | x -> x
        let program = 
            TryValue::create(
                TryValue::create(
                    ThrowValue::create(Rc::new("error".into())), 
                    Rc::new(
                        DidValue::List(
                            Vector::new()
                            .push_back(
                                RuleValue::create(
                                    Rc::new(1.into()), 
                                    Rc::new(DidValue::Null), 
                                    Rc::new(1.into())
                                )
                            )
                        )
                    )
                ), 
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(
                            RuleValue::create(
                                LookupValue::create("x"), 
                                Rc::new(DidValue::Null), 
                                LookupValue::create("x")
                            )
                        )
                    )
                )
            );
        // "error"
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new("error".into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_nexted_try_catch_with_inner_exception() {
        // try try throw "error" with | x -> throw x with | "error" -> "worked"
        let program = 
            TryValue::create(
                TryValue::create(
                    ThrowValue::create(Rc::new("error".into())), 
                    Rc::new(
                        DidValue::List(
                            Vector::new()
                            .push_back(
                                RuleValue::create(
                                    LookupValue::create("x"), 
                                    Rc::new(DidValue::Null), 
                                    ThrowValue::create(LookupValue::create("x"))
                                )
                            )
                        )
                    )
                ), 
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(
                            RuleValue::create(
                                Rc::new("error".into()), 
                                Rc::new(DidValue::Null), 
                                Rc::new("worked".into())
                            )
                        )
                    )
                )
            );
        // "worked"
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new("worked".into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_nexted_try_catch_with_no_exception() {
        // try try 1 with | 2 -> 2 with | 3 -> 3
        let program = 
            TryValue::create(
                TryValue::create(
                    Rc::new(1.into()), 
                    Rc::new(
                        DidValue::List(
                            Vector::new()
                            .push_back(
                                RuleValue::create(
                                    Rc::new(2.into()), 
                                    Rc::new(DidValue::Null), 
                                    Rc::new(2.into())
                                )
                            )
                        )
                    )
                ), 
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(
                            RuleValue::create(
                                Rc::new(3.into()), 
                                Rc::new(DidValue::Null), 
                                Rc::new(3.into())
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