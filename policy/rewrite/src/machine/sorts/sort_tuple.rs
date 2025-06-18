// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::{HashTrieMap, Vector};

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

use super::sort_list::ListItemValue;
pub struct TupleValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> TupleValue<'a> {
    pub fn items(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"items".to_string()).unwrap()) }

    pub fn create(items: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("items".to_string(), items);
        Rc::new(DidValue::Map(Sort::Tuple, ht))
    }

    pub fn create_for_parser(item_vec: Vec<Rc<DidValue>>, last: Option<Rc<DidValue>>) -> Rc<DidValue> {
        if item_vec.len() == 0 && last.is_some() {
            last.unwrap()
        } else {
            let mut items: Vector<Rc<DidValue>> = Vector::new();
            for item in item_vec {
                items.push_back_mut(item);
            }
            if last.is_some() {
                let last_item = ListItemValue::create(last.unwrap(), Rc::new(false.into()));
                items.push_back_mut(last_item);
            }
            TupleValue::create(Rc::new(DidValue::Tuple(items)))
        }
    }
}

pub fn rewrite_tuple(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let tuple_value = TupleValue { value: &context.term };
    if let Some(items) = tuple_value.items().try_tuple() {
        let mut items_blocked: Vector<Rc<DidValue>> = Vector::new();
        let mut items_result: Vector<Rc<DidValue>> = Vector::new();
        let mut blocked = false;
        let mut prev_parallel = false;
        for item in items {
            let item_value = ListItemValue { value: &item };
            if !blocked || prev_parallel {
                let item_context = context.with_term(Rc::clone(&item_value.term()));
                match rewrite_term(machine, item_context)? {
                    RewriteOk::Term(item_result) => {
                        let list_item = 
                            ListItemValue::create(
                                Rc::clone(&item_result), 
                                item_value.parallel()
                            );
                        items_blocked.push_back_mut(list_item);
                        if !blocked {
                            items_result.push_back_mut(item_result);
                        }
                    },
                    RewriteOk::Blocked(item_blocked) => {
                        let list_item = 
                            ListItemValue::create(
                                Rc::clone(&item_blocked), 
                                item_value.parallel()
                            );
                        items_blocked.push_back_mut(list_item);
                        blocked = true;
                        prev_parallel = false;
                    },
                }
            } else {
                items_blocked.push_back_mut(Rc::clone(&item));
            }
            if let Some(parallel) = item_value.parallel().try_bool() {
                prev_parallel |= parallel;
            }
        };
        if blocked {
            Result::Ok(
                RewriteOk::Blocked(
                    TupleValue::create(
                        Rc::new(DidValue::Tuple(items_blocked))
                    ).as_hash()
                )
            )
        } else {
            Result::Ok(RewriteOk::Term(Rc::new(DidValue::Tuple(items_result)).as_hash()))
        }
    } else {
        panic!("expected tuple")
    }
}

pub fn bind_tuple(machine: &mut RewriteMachine, context: RewriteContext, value: Rc<DidValue>) -> Result<MatchOk, RewriteErr> {
    if let Some(tuple_value) = value.try_tuple() {
        let mut pattern_bindings = context.bindings;
        let pattern_tuple = TupleValue { value: &context.term };
        if let Some(pattern_items) = pattern_tuple.items().try_tuple() {
            if tuple_value.len() == pattern_items.len() {
                let mut index: usize = 0;
                for pattern_item in pattern_items {
                    let pattern_item_value = ListItemValue { value: &pattern_item };
                    let pattern_term = pattern_item_value.term();
                    if let Some(index_value) = tuple_value.get(index) {
                        index += 1;
                        let item_context = RewriteContext {
                            base: Rc::clone(&context.base),
                            term: Rc::clone(&pattern_term),
                            bindings: Rc::clone(&pattern_bindings),
                            policies: Rc::clone(&context.policies),
                        };
                        match term_bind(machine, item_context, Rc::clone(index_value)) {
                            Result::Ok(MatchOk::Bindings(item_bindings)) => {
                                pattern_bindings = item_bindings;
                            },
                            Result::Ok(MatchOk::NoMatch) => {
                                return Result::Ok(MatchOk::NoMatch)
                            },
                            err => return err,
                        }
                    } else {
                        // there's a pattern key but no matching one in value_map
                        return Result::Ok(MatchOk::NoMatch)
                    }
                }
                Result::Ok(MatchOk::Bindings(pattern_bindings))
            } else {
                Result::Ok(MatchOk::NoMatch)
            }
        } else {
            panic!("expected tuple")
        }        
    } else {
        Result::Ok(MatchOk::NoMatch)
    }
}


#[cfg(test)]
mod tests {
    use crate::machine::sorts::sort_match::{MatchValue, RuleValue};

    use super::*;
    use super::super::*;
    use sort_lookup::LookupValue;

    use rpds::Vector;

    #[test]
    fn test_tuple() {
        // ( (succ 1) ; (succ 2) )
        let program = TupleValue::create(
                Rc::new(
                    DidValue::Tuple(
                        Vector::new()
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(1.into())), Rc::new(false.into())))
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))            
                    )
                )
            );
        // ( 2; 3 )
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    Rc::new(
                        DidValue::Tuple(
                            Vector::new()
                            .push_back(Rc::new(2.into()))
                            .push_back(Rc::new(3.into()))
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tuple_empty() {
        // ( )
        let program = TupleValue::create(
                Rc::new(DidValue::Tuple(Vector::new()))
            );
        // ( )
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    Rc::new(
                        DidValue::Tuple(Vector::new())
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tuple_parallel() {
        // ( (succ 1) |, (succ 2) )
        let program = TupleValue::create(
                Rc::new(
                    DidValue::Tuple(
                        Vector::new()
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(1.into())), Rc::new(true.into())))
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))            
                    )
                )
            );
        // ( 2, 3 )
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    Rc::new(
                        DidValue::Tuple(
                            Vector::new()
                            .push_back(Rc::new(2.into()))
                            .push_back(Rc::new(3.into()))
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tuple_parallel_blocked_first_item() {
        // ( (succ blocked) |, (succ 2) )
        let program = TupleValue::create(
                Rc::new(
                    DidValue::Tuple(
                        Vector::new()
                        .push_back(ListItemValue::create(apply_tapl_succ(LookupValue::create("blocked")), Rc::new(true.into())))
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))            
                    )
                )
            );
        // ( (succ blocked) |, 3 )
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    TupleValue::create(
                    Rc::new(
                        DidValue::Tuple(
                            Vector::new()
                            .push_back(ListItemValue::create(apply_tapl_succ(LookupValue::create("blocked")), Rc::new(true.into())))
                            .push_back(ListItemValue::create(Rc::new(3.into()), Rc::new(false.into())))
                        )
                    )
                )
            )
        );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tuple_parallel_blocked_second_item() {
        // ( (succ 1) |, (succ blocked) )
        let program = TupleValue::create(
                Rc::new(
                    DidValue::Tuple(
                        Vector::new()
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(1.into())), Rc::new(true.into())))            
                        .push_back(ListItemValue::create(apply_tapl_succ(LookupValue::create("blocked")), Rc::new(false.into())))
                    )
                )
            );
        // ( 2 |, (succ blocked) )
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    TupleValue::create(
                    Rc::new(
                        DidValue::Tuple(
                            Vector::new()
                            .push_back(ListItemValue::create(Rc::new(2.into()), Rc::new(true.into())))
                            .push_back(ListItemValue::create(apply_tapl_succ(LookupValue::create("blocked")), Rc::new(false.into())))
                        )
                    )
                )
            )
        );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tuple_sequential_blocked_first_item() {
        // ( (succ blocked) , (succ 2) )
        let program = TupleValue::create(
                Rc::new(
                    DidValue::Tuple(
                        Vector::new()
                        .push_back(ListItemValue::create(apply_tapl_succ(LookupValue::create("blocked")), Rc::new(false.into())))
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))            
                    )
                )
            );
        // ( (succ blocked) |, (succ 2) )
        let result = run_rewrite(program.clone());
        let expected = Result::Ok(RewriteOk::Blocked(program));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tuple_sequential_blocked_second_item() {
        // ( (succ 1) , (succ blocked) )
        let program = TupleValue::create(
                Rc::new(
                    DidValue::Tuple(
                        Vector::new()
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(1.into())), Rc::new(false.into())))            
                        .push_back(ListItemValue::create(apply_tapl_succ(LookupValue::create("blocked")), Rc::new(false.into())))
                    )
                )
            );
        // ( 2 , (succ blocked) )
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    TupleValue::create(
                    Rc::new(
                        DidValue::Tuple(
                            Vector::new()
                            .push_back(ListItemValue::create(Rc::new(2.into()), Rc::new(false.into())))
                            .push_back(ListItemValue::create(apply_tapl_succ(LookupValue::create("blocked")), Rc::new(false.into())))
                        )
                    )
                )
            )
        );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tuple_pattern_empty() {
        // match () with | () -> 1 | _ -> 2
        let program = MatchValue::create(
            Rc::new(DidValue::Tuple(Vector::new())),
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(
                        RuleValue::create(
                            TupleValue::create(
                                Rc::new(
                                    DidValue::Tuple(
                                        Vector::new()
                                    )
                                )
                            ), 
                            Rc::new(DidValue::Null), 
                            Rc::new(1.into())
                        )
                    ).push_back(
                        RuleValue::create(
                            LookupValue::create("_"), 
                            Rc::new(DidValue::Null), 
                            Rc::new(2.into())
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
    fn test_tuple_pattern_two() {
        // match ( true, 1 ) with | (x, y) -> (y, x) | _ -> 2
        let program = MatchValue::create(
            Rc::new(
                DidValue::Tuple(
                    Vector::new()
                    .push_back(Rc::new(true.into()))
                    .push_back(Rc::new(1.into()))
                )
            ),
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(
                        RuleValue::create(
                            TupleValue::create(
                                Rc::new(
                                    DidValue::Tuple(
                                        Vector::new()
                                        .push_back(ListItemValue::create(LookupValue::create("x"), Rc::new(false.into())))
                                        .push_back(ListItemValue::create(LookupValue::create("y"), Rc::new(false.into())))
                                    )
                                )
                            ), 
                            Rc::new(DidValue::Null), 
                            TupleValue::create(
                                Rc::new(
                                    DidValue::Tuple(
                                        Vector::new()
                                        .push_back(ListItemValue::create(LookupValue::create("y"), Rc::new(false.into())))
                                        .push_back(ListItemValue::create(LookupValue::create("x"), Rc::new(false.into())))
                                    )
                                )
                            )
                        )
                    ).push_back(
                        RuleValue::create(
                            LookupValue::create("_"), 
                            Rc::new(DidValue::Null), 
                            Rc::new(2.into())
                        )
                    )
                )
            )
        );
        // (1, true)
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    Rc::new(
                        DidValue::Tuple(
                            Vector::new()
                            .push_back(Rc::new(1.into()))
                            .push_back(Rc::new(true.into()))
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_tuple_pattern_no_match() {
        // match ( true, 1 ) with | (false, y) -> (y, false) | _ -> 2
        let program = MatchValue::create(
            Rc::new(
                DidValue::Tuple(
                    Vector::new()
                    .push_back(Rc::new(true.into()))
                    .push_back(Rc::new(1.into()))
                )
            ),
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(
                        RuleValue::create(
                            TupleValue::create(
                                Rc::new(
                                    DidValue::Tuple(
                                        Vector::new()
                                        .push_back(ListItemValue::create(Rc::new(false.into()), Rc::new(false.into())))
                                        .push_back(ListItemValue::create(LookupValue::create("y"), Rc::new(false.into())))
                                    )
                                )
                            ), 
                            Rc::new(DidValue::Null), 
                            TupleValue::create(
                                Rc::new(
                                    DidValue::Tuple(
                                        Vector::new()
                                        .push_back(ListItemValue::create(LookupValue::create("y"), Rc::new(false.into())))
                                        .push_back(ListItemValue::create(Rc::new(false.into()), Rc::new(false.into())))
                                    )
                                )
                            )
                        )
                    ).push_back(
                        RuleValue::create(
                            LookupValue::create("_"), 
                            Rc::new(DidValue::Null), 
                            Rc::new(2.into())
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

}