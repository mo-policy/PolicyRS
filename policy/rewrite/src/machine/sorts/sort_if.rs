// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

pub struct IfValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> IfValue<'a> {
    pub fn condition(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"condition".to_string()).unwrap()) }
    pub fn then_term(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"then_term".to_string()).unwrap()) }
    pub fn else_term(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"else_term".to_string()).unwrap()) }

    pub fn create(condition: Rc<DidValue>, then_term: Rc<DidValue>, else_term: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("condition".to_string(), condition)
            .insert("then_term".to_string(), then_term)
            .insert("else_term".to_string(), else_term);
        Rc::new(DidValue::Map(Sort::If, ht))
    }

    pub fn create_for_parser(condition: Rc<DidValue>, then_term: Rc<DidValue>, else_term: Option<Rc<DidValue>>) -> Rc<DidValue> {
        if else_term.is_none() {
            IfValue::create(condition, then_term, Rc::new(DidValue::Null))
        } else {
            IfValue::create(condition, then_term, else_term.unwrap())
        }
    }
}

pub fn rewrite_if(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let if_value = IfValue { value: &context.term };
    let condition_context = context.with_term(Rc::clone(&if_value.condition()));
    match rewrite_term(machine, condition_context)? {
        RewriteOk::Term(condition_result) => {
            if let Some(condition) = condition_result.try_bool() {
                if condition {
                    let then_context = context.with_term(Rc::clone(&if_value.then_term()));
                    rewrite_term(machine, then_context)
                } else {
                    let else_context = context.with_term(Rc::clone(&if_value.else_term()));
                    rewrite_term(machine, else_context)
                }
            } else {
                Result::Err(RewriteErr::Exception(Rc::new("Condition not Bool".into())))
            }
        },
        RewriteOk::Blocked(condition_blocked) => {
            Result::Ok(
                RewriteOk::Blocked(
                    if_value.value.with_map_value(
                        "condition".to_string(), 
                        condition_blocked
                    ).as_hash()
                )
            )
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    use sort_lookup::LookupValue;

    #[test]
    fn test_if_true_then() {
        // if true then 1
        let program = IfValue::create(
            Rc::new(true.into()),
            Rc::new(1.into()),
            Rc::new(DidValue::Null)
        );
        // 1
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(1.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_if_true_then_with_else() {
        // if true then 1 else 2
        let program = IfValue::create(
            Rc::new(true.into()),
            Rc::new(1.into()),
            Rc::new(2.into())
        );
        // 1
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(1.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_if_false_no_else() {
        // if false then 1
        let program = IfValue::create(
            Rc::new(false.into()),
            Rc::new(1.into()),
            Rc::new(DidValue::Null)
        );
        // 1
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(DidValue::Null)));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_if_false_with_else() {
        // if false then 1 else 2
        let program = IfValue::create(
            Rc::new(false.into()),
            Rc::new(1.into()),
            Rc::new(2.into())
        );
        // 2
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(2.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_if_blocked_condition() {
        // if blocked then 1 else 2
        let program = IfValue::create(
            LookupValue::create("blocked"),
            Rc::new(1.into()),
            Rc::new(2.into())
        );
        // if blocked then 1 else 2
        let result = run_rewrite(program.clone());
        let expected = Result::Ok(RewriteOk::Blocked(program));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_if_blocked_then() {
        // if true then blocked else 2
        let program = IfValue::create(
            Rc::new(true.into()),
            LookupValue::create("blocked"),
            Rc::new(2.into())
        );
        // blocked
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Blocked(LookupValue::create("blocked")));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_if_blocked_else() {
        // if false then 1 else blocked
        let program = IfValue::create(
            Rc::new(false.into()),
            Rc::new(1.into()),
            LookupValue::create("blocked")
        );
        // blocked
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Blocked(LookupValue::create("blocked")));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_if_condition_not_bool_exception() {
        // if 1 then 1
        let program = IfValue::create(
            Rc::new(1.into()),
            Rc::new(1.into()),
            Rc::new(DidValue::Null)
        );
        // Expected Exception
        let result = run_rewrite(program);
        let expected = Result::Err(RewriteErr::Exception(Rc::new("Condition not Bool".into())));
        assert_eq!(expected, result);
    }
}