// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

pub struct PolicyValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> PolicyValue<'a> {
    pub fn term(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"term".to_string()).unwrap()) }
    pub fn rules(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"rules".to_string()).unwrap()) }

    pub fn create(term: Rc<DidValue>, rules: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("term".to_string(), term)
            .insert("rules".to_string(), rules);
        Rc::new(DidValue::Map(Sort::Policy, ht))
    }
}

pub fn rewrite_policy(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let policy_value = PolicyValue { value: &Rc::clone(&context.term) };
    let term_context = 
        RewriteContext {
            base: Rc::clone(&context.base),
            term: policy_value.term(),
            bindings: Rc::clone(&context.bindings),
            policies: Rc::new(context.policies.push_front(context.clone())),
        };
    match rewrite_term(machine, term_context) {
        Result::Ok(RewriteOk::Blocked(blocked_term)) => {
            Result::Ok(
                RewriteOk::Blocked(
                    PolicyValue::create(
                        blocked_term, 
                        policy_value.rules()
                    ).as_hash()
                )
            )
        },
        Result::Err(RewriteErr::PolicyViolation(blocked, term)) => {
            if blocked {
                Result::Ok(RewriteOk::Blocked(term))
            } else {
                Result::Ok(RewriteOk::Term(term))
            }
        }
        result => result,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    use sort_lookup::LookupValue;
    use sort_if::IfValue;
    use sort_match::RuleValue;
    use rpds::Vector;

    #[test]
    fn test_policy_one_rule_no_match() {
        // policy (succ 1) with | false -> false
        let program = 
            PolicyValue::create(
                apply_tapl_succ(Rc::new(1.into())),
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(
                            RuleValue::create(
                                Rc::new(false.into()), 
                                Rc::new(DidValue::Null), 
                                Rc::new(false.into())
                            )
                        )
                    )
                )

            );
        // 2
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(2.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_policy_one_rule_no_match_blocked() {
        // policy if true then blocked with | false -> false
        let program = 
            PolicyValue::create(
                IfValue::create(
                    Rc::new(true.into()), 
                    LookupValue::create("blocked"), 
                    Rc::new(DidValue::Null)
                ),
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(
                            RuleValue::create(
                                Rc::new(false.into()), 
                                Rc::new(DidValue::Null), 
                                Rc::new(false.into())
                            )
                        )
                    )
                )
            );
        // policy blocked with | false -> false
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    PolicyValue::create(
                        LookupValue::create("blocked"),
                        Rc::new(
                            DidValue::List(
                                Vector::new()
                                .push_back(
                                    RuleValue::create(
                                        Rc::new(false.into()), 
                                        Rc::new(DidValue::Null), 
                                        Rc::new(false.into())
                                    )
                                )
                            )
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_policy_one_rule_with_match() {
        // policy (succ 1) with | x -> x 
        let program = 
            PolicyValue::create(
                apply_tapl_succ(Rc::new(1.into())),
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
        // (succ 1)
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(apply_tapl_succ(Rc::new(1.into()))));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_policy_two_rules_with_match() {
        // policy if true then (succ 1) with | false -> 1 | true -> 5
        let program = 
            PolicyValue::create(
                IfValue::create(
                    Rc::new(true.into()), 
                    apply_tapl_succ(Rc::new(1.into())), 
                    Rc::new(DidValue::Null)
                ),
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(
                            RuleValue::create(
                                Rc::new(false.into()), 
                                Rc::new(DidValue::Null), 
                                Rc::new(1.into())
                            )
                        )
                        .push_back(
                            RuleValue::create(
                                Rc::new(true.into()), 
                                Rc::new(DidValue::Null), 
                                Rc::new(5.into())
                            )
                        )
                    )
                )
            );
        // Exception "Condition not Bool"
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(5.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_policy_inner_policy_with_outer_match() {
        // policy (policy if true then (succ 1) with | false -> 1) | true -> 5
        let program = 
            PolicyValue::create(
                PolicyValue::create(
                    IfValue::create(
                        Rc::new(true.into()), 
                        apply_tapl_succ(Rc::new(1.into())), 
                        Rc::new(DidValue::Null)
                    ),
                    Rc::new(
                        DidValue::List(
                            Vector::new()
                            .push_back(
                                RuleValue::create(
                                    Rc::new(false.into()), 
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
                                Rc::new(true.into()), 
                                Rc::new(DidValue::Null), 
                                Rc::new(5.into())
                            )
                        )
                    )
                )
            );
        // Exception "Condition not Bool"
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(5.into())));
        assert_eq!(expected, result);
    }
}
