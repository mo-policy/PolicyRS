// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::{HashTrieMap, Vector};

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

pub struct RuleValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> RuleValue<'a> {
    pub fn pattern(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"pattern".to_string()).unwrap()) }
    pub fn guard(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"guard".to_string()).unwrap()) }
    pub fn term(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"term".to_string()).unwrap()) }

    pub fn create(pattern: Rc<DidValue>, guard: Rc<DidValue>, term: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("pattern".to_string(), pattern)
            .insert("guard".to_string(), guard)
            .insert("term".to_string(), term);
        Rc::new(DidValue::Map(Sort::Constant, ht))
    }

    pub fn create_for_parser(pattern: Rc<DidValue>, guard: Option<Rc<DidValue>>, term: Rc<DidValue>) -> Rc<DidValue> {
        if guard.is_none() {
            RuleValue::create(pattern, Rc::new(DidValue::Null), term)
        } else {
            RuleValue::create(pattern, guard.unwrap(), term)
        }
    }

    pub fn create_rules_for_parser(rules_vec: Vec<Rc<DidValue>>) -> Rc<DidValue> {
        let mut rules: Vector<Rc<DidValue>> = Vector::new();
        for rule in rules_vec {
            rules.push_back_mut(rule);
        }
        Rc::new(DidValue::List(rules))
    }
}

pub enum RulesOk {
    MatchedRule(RewriteContext),    // RewriteContext with term of matching rule, and bindings from rule pattern
    BlockedGuard(Rc<DidValue>),     // DidValue::List of remaining rules, first one has blocked guard
    NoRuleMatched,                  // None of the rules matched
}

pub fn rewrite_rules(machine: &mut RewriteMachine, context: RewriteContext, rules: Rc<DidValue>) -> Result<RulesOk, RewriteErr> {
    if let Some(rules) = rules.try_list() {
        let mut rule_index: usize = 0;
        for rule in rules {
            let rule_value = RuleValue { value: rule };
            let pattern_context = context.with_term(Rc::clone(&rule_value.pattern()));
            match term_bind(machine, pattern_context, Rc::clone(&context.term))? {
                MatchOk::Bindings(rule_bindings) => {
                    let mut guard_result = true;
                    if let None = rule_value.guard().try_null() {
                        let guard_context = 
                            RewriteContext {
                                base: Rc::clone(&context.base),
                                term: Rc::clone(&rule_value.guard()),
                                bindings: Rc::clone(&rule_bindings),
                                policies: Rc::clone(&context.policies),
                            };
                        match rewrite_term(machine, guard_context)? {
                            RewriteOk::Term(guard_term) => {
                                if let Some(v) = guard_term.try_bool() {
                                    guard_result = v;
                                } else {
                                    return Result::Err(RewriteErr::Exception(Rc::new("Guard not Bool".into())));
                                }
                            },
                            RewriteOk::Blocked(guard_blocked) => {
                                let mut blocked_rules: Vector<Rc<DidValue>> = Vector::new();
                                let blocked_rule = 
                                    RuleValue::create(
                                        Rc::clone(&rule_value.pattern()), 
                                        guard_blocked, 
                                        Rc::clone(&rule_value.term())
                                    );
                                blocked_rules.push_back_mut(blocked_rule);
                                let mut i: usize = 0;
                                for r in rules {
                                    if i > rule_index {
                                        blocked_rules.push_back_mut(Rc::clone(r));
                                    }
                                    i += 1;
                                }
                                return Result::Ok(RulesOk::BlockedGuard(Rc::new(DidValue::List(blocked_rules))));
                            },
                        };
                    };
                    if guard_result {
                        let rule_context = 
                            RewriteContext {
                                base: Rc::clone(&context.base),
                                term: rule_value.term(),
                                bindings: rule_bindings,
                                policies: Rc::clone(&context.policies),
                            };
                        return Result::Ok(RulesOk::MatchedRule(rule_context));
                    }
                },
                MatchOk::NoMatch => { },
            };
            rule_index += 1;
        };
        Result::Ok(RulesOk::NoRuleMatched)
    } else {
        panic!("expected rules as list")
    }

}

pub struct MatchValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> MatchValue<'a> {
    pub fn term(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"term".to_string()).unwrap()) }
    pub fn rules(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"rules".to_string()).unwrap()) }

    pub fn create(term: Rc<DidValue>, rules: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("term".to_string(), term)
            .insert("rules".to_string(), rules);
        Rc::new(DidValue::Map(Sort::Match, ht))
    }
}


pub fn rewrite_match(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let match_value = MatchValue { value: &context.term };
    let term_context = context.with_term(Rc::clone(&match_value.term()));
    match rewrite_term(machine, term_context)? {
        RewriteOk::Term(term_result) => {
            let rules_context = context.with_term(Rc::clone(&term_result));
            match rewrite_rules(machine, rules_context, match_value.rules())? {
                RulesOk::MatchedRule(rule_context) => 
                    rewrite_term(machine, rule_context),
                RulesOk::BlockedGuard(blocked_rules) => {
                    Result::Ok(
                        RewriteOk::Blocked(
                            MatchValue::create(
                                term_result, 
                                blocked_rules
                            )
                        )
                    )
                },
                RulesOk::NoRuleMatched => 
                    Result::Err(RewriteErr::Exception(Rc::new("No rule matched".into()))),
            }
        },
        RewriteOk::Blocked(term_blocked) => {
            Result::Ok(
                RewriteOk::Blocked(
                    match_value.value.with_map_value(
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
    use super::*;
    use super::super::*;
    use sort_lookup::LookupValue;
    use sort_if::IfValue;

    #[test]
    fn test_match_one_rule() {
        // match (succ 1) with | x -> x
        let program = MatchValue::create(
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
        // 2
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(2.into())));
        assert_eq!(expected, result);
    }

   #[test]
   fn test_match_one_rule_with_true_guard() {
       // match (succ 1) with | x when true -> x
       let program = MatchValue::create(
           apply_tapl_succ(Rc::new(1.into())),
           Rc::new(
               DidValue::List(
                   Vector::new()
                   .push_back(
                       RuleValue::create(
                           LookupValue::create("x"), 
                           Rc::new(true.into()), 
                           LookupValue::create("x")
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
    fn test_match_two_rules_match_first() {
        // match (succ 1) with | 2 -> 3 | x -> x
        let program = MatchValue::create(
            apply_tapl_succ(Rc::new(1.into())),
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(
                        RuleValue::create(
                            Rc::new(2.into()), 
                            Rc::new(true.into()), 
                            Rc::new(3.into()),
                        )
                    )
                    .push_back(
                        RuleValue::create(
                            LookupValue::create("x"), 
                            Rc::new(true.into()), 
                            LookupValue::create("x")
                        )
                    )
                )
            )
        );
        // 3
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(3.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_match_two_rules_match_second() {
        // match (succ 1) with | 3 -> 4 | x -> (succ x)
        let program = MatchValue::create(
            apply_tapl_succ(Rc::new(1.into())),
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(
                        RuleValue::create(
                            Rc::new(3.into()), 
                            Rc::new(DidValue::Null), 
                            Rc::new(4.into()),
                        )
                    )
                    .push_back(
                        RuleValue::create(
                            LookupValue::create("x"), 
                            Rc::new(DidValue::Null), 
                            apply_tapl_succ(LookupValue::create("x"))
                        )
                    )
                )
            )
        );
        // 3
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(3.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_match_two_rules_match_second_with_failed_guard() {
        // match (succ 1) with | x when false -> x | x -> (succ x)
        let program = MatchValue::create(
            apply_tapl_succ(Rc::new(1.into())),
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(
                        RuleValue::create(
                            LookupValue::create("x"), 
                            Rc::new(false.into()), 
                            LookupValue::create("x"),
                        )
                    )
                    .push_back(
                        RuleValue::create(
                            LookupValue::create("x"), 
                            Rc::new(DidValue::Null), 
                            apply_tapl_succ(LookupValue::create("x"))
                        )
                    )
                )
            )
        );
        // 3
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(3.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_match_three_rules_match_two_failed_guards() {
        // match (succ 1) with | 2 when false -> 2 | x when false -> x | x -> (succ x)
        let program = MatchValue::create(
            apply_tapl_succ(Rc::new(1.into())),
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(
                        RuleValue::create(
                            Rc::new(2.into()), 
                            Rc::new(false.into()), 
                            Rc::new(2.into()), 
                        )
                    )
                    .push_back(
                        RuleValue::create(
                            LookupValue::create("x"), 
                            Rc::new(false.into()), 
                            LookupValue::create("x")
                        )
                    )
                    .push_back(
                        RuleValue::create(
                            LookupValue::create("x"), 
                            Rc::new(DidValue::Null), 
                            apply_tapl_succ(LookupValue::create("x"))
                        )
                    )
                )
            )
        );
        // 3
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(3.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_match_four_rules_match_two_failed_guards() {
        // match (succ 1) with | 2 when false -> 2 | x when false -> x | x -> (succ x) | 4 -> 4
        let program = MatchValue::create(
            apply_tapl_succ(Rc::new(1.into())),
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(
                        RuleValue::create(
                            Rc::new(2.into()), 
                            Rc::new(false.into()), 
                            Rc::new(2.into()), 
                        )
                    )
                    .push_back(
                        RuleValue::create(
                            LookupValue::create("x"), 
                            Rc::new(false.into()), 
                            LookupValue::create("x")
                        )
                    )
                    .push_back(
                        RuleValue::create(
                            LookupValue::create("x"), 
                            Rc::new(DidValue::Null), 
                            apply_tapl_succ(LookupValue::create("x"))
                        )
                    )
                    .push_back(
                        RuleValue::create(
                            Rc::new(4.into()), 
                            Rc::new(DidValue::Null), 
                            Rc::new(4.into()), 
                        )
                    )
                )
            )
        );
        // 3
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(3.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_match_one_rule_blocked_term() {
        // match blocked with | x -> x
        let program = MatchValue::create(
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
        // match blocked with | x -> x
        let result = run_rewrite(program.clone());
        let expected = Result::Ok(RewriteOk::Blocked(program));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_match_one_rule_blocked_rule_term() {
        // match 1 with | x -> blocked
        let program = MatchValue::create(
            Rc::new(1.into()),
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(
                        RuleValue::create(
                            LookupValue::create("x"), 
                            Rc::new(DidValue::Null), 
                            LookupValue::create("blocked")
                        )
                    )
                )
            )
        );
        // blocked
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Blocked(LookupValue::create("blocked")));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_match_one_rule_blocked_rule_guard() {
        // match (succ 1) with | x when if true then blocked -> x
        let program = MatchValue::create(
            apply_tapl_succ(Rc::new(1.into())),
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(
                        RuleValue::create(
                            LookupValue::create("x"), 
                            IfValue::create(
                                Rc::new(true.into()),
                                LookupValue::create("blocked"),
                                Rc::new(DidValue::Null)
                            ), 
                            LookupValue::create("x")
                        )
                    )
                )
            )
        );
        // match 2 with | x when blocked -> x
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    MatchValue::create(
                    Rc::new(2.into()),
                    Rc::new(
                        DidValue::List(
                            Vector::new()
                            .push_back(
                                RuleValue::create(
                                    LookupValue::create("x"), 
                                    LookupValue::create("blocked"), 
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

    #[test]
    fn test_match_three_rules_no_rule_matched() {
        // match (succ 1) with | 1 -> 1 | 2 when false -> 2 | 3 -> 3
        let program = MatchValue::create(
            apply_tapl_succ(Rc::new(1.into())),
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
                    .push_back(
                        RuleValue::create(
                            Rc::new(2.into()), 
                            Rc::new(false.into()), 
                            Rc::new(2.into())
                        )
                    )
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
        // Exception "No rule matched"
        let result = run_rewrite(program);
        let expected = Result::Err(RewriteErr::Exception(Rc::new("No rule matched".into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_match_one_rule_guard_not_bool() {
        // match (succ 1) with | x when 1 -> 2
        let program = MatchValue::create(
            apply_tapl_succ(Rc::new(1.into())),
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(
                        RuleValue::create(
                            LookupValue::create("x"), 
                            Rc::new(1.into()), 
                            Rc::new(2.into())
                        )
                    )
                )
            )
        );
        // Exception "Guard not Bool"
        let result = run_rewrite(program);
        let expected = Result::Err(RewriteErr::Exception(Rc::new("Guard not Bool".into())));
        assert_eq!(expected, result);
    }
}
