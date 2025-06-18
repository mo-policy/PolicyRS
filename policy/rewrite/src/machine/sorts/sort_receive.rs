// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

use super::sort_match::{rewrite_rules, RulesOk};

pub struct ReceiveValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> ReceiveValue<'a> {
    pub fn last_id(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"last_id".to_string()).unwrap()) }
    pub fn channel(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"channel".to_string()).unwrap()) }
    pub fn rules(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"rules".to_string()).unwrap()) }

    pub fn create(last_id: Rc<DidValue>, channel: Rc<DidValue>, rules: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("last_id".to_string(), last_id)
            .insert("channel".to_string(), channel)
            .insert("rules".to_string(), rules);
        Rc::new(DidValue::Map(Sort::Receive, ht))
    }
}

pub fn rewrite_receive(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let receive_value = ReceiveValue { value: &context.term };
    let channel_context = context.with_term(Rc::clone(&receive_value.channel()));
    match rewrite_term(machine, channel_context)? {
        RewriteOk::Term(channel_result) => {
            let mut last_id: Option<String> =
                if let Some(id) = receive_value.last_id().try_string() {
                    Some(id.clone())
                } else {
                    None
                };
            loop {
                match machine.comm.reserve(Rc::clone(&channel_result), &last_id) {
                    Some((id, message)) => {
                        let rules_context = context.with_term(Rc::clone(&message));
                        match rewrite_rules(machine, rules_context, receive_value.rules())? {
                            RulesOk::MatchedRule(matched_rule_context) => {
                                let rx = machine.comm.receive(&Some(id));
                                assert!(rx);
                                return rewrite_term(machine, matched_rule_context);
                            },
                            RulesOk::BlockedGuard(blocked_rules) => {
                                return Result::Ok(
                                    RewriteOk::Blocked(
                                        ReceiveValue::create(
                                            Rc::new(id.into()),
                                            channel_result, 
                                            blocked_rules
                                        ).as_hash()
                                    )
                                );
                            },
                            RulesOk::NoRuleMatched => {
                                last_id = Some(id);
                            },
                        };
                    },
                    None => {
                        return Result::Ok(
                            RewriteOk::Blocked(
                                receive_value.value.with_map_value(
                                    "channel".to_string(), 
                                    channel_result
                                ).as_hash()
                            )
                        );
                    }
                }
    
            }
        },
        RewriteOk::Blocked(channel_blocked) => {
            Result::Ok(
                RewriteOk::Blocked(
                    receive_value.value.with_map_value(
                        "channel".to_string(), 
                        channel_blocked
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
    use sort_match::RuleValue;
    use rpds::{List, Vector};

    #[test]
    fn test_receive() {
        // receive succ 1 with | x -> x
        let program = 
            ReceiveValue::create(
                Rc::new(DidValue::Null),
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
        let mut machine = RewriteMachine::new();
        machine.comm.send(Rc::new(2.into()), Rc::new(3.into()));
        let context = 
            RewriteContext {
                base: Rc::clone(&program),
                term: program,
                bindings: DidValue::new_map_constant(),
                policies: Rc::new(List::new()),
            };
        // 3
        let result = rewrite_term(&mut machine, context);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(3.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_receive_second_message() {
        // receive succ 1 with | false -> false | 2 -> succ 2
        let program = 
            ReceiveValue::create(
                Rc::new(DidValue::Null),
                apply_tapl_succ(Rc::new(1.into())),
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(
                            RuleValue::create(
                                Rc::new(1.into()), 
                                Rc::new(false.into()), 
                                Rc::new(false.into())
                            )
                        )
                        .push_back(
                            RuleValue::create(
                                Rc::new(2.into()), 
                                Rc::new(DidValue::Null), 
                                apply_tapl_succ(Rc::new(2.into()))
                            )
                        )
                    )
                )
            );
        let mut machine = RewriteMachine::new();
        machine.comm.send(Rc::new(2.into()), Rc::new(1.into()));
        machine.comm.send(Rc::new(2.into()), Rc::new(2.into()));
        let context = 
            RewriteContext {
                base: Rc::clone(&program),
                term: program,
                bindings: DidValue::new_map_constant(),
                policies: Rc::new(List::new()),
            };
        // 3
        let result = rewrite_term(&mut machine, context);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(3.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_receive_channel_blocked() {
        // receive blocked with | x -> x
        let program = 
            ReceiveValue::create(
                Rc::new(DidValue::Null),
                LookupValue::create("blocked"),
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
        // receive blocked with | x -> x
        let result = run_rewrite(program.clone());
        let expected = Result::Ok(RewriteOk::Blocked(program));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_receive_no_message() {
        // receive succ 1 with | x -> x
        let program = 
            ReceiveValue::create(
                Rc::new(DidValue::Null),
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
        // receive 2 with | x -> x
        let result = run_rewrite(program.clone());
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    ReceiveValue::create(
                        Rc::new(DidValue::Null),
                        Rc::new(2.into()),
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
                    )
                )
            );
        assert_eq!(expected, result);
    }
}
