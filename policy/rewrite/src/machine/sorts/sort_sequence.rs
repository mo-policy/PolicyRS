// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::{HashTrieMap, Vector};

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

use super::sort_list::ListItemValue;
pub struct SequenceValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> SequenceValue<'a> {
    pub fn items(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"items".to_string()).unwrap()) }

    pub fn create(items: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("items".to_string(), items);
        Rc::new(DidValue::Map(Sort::Sequence, ht))
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
            SequenceValue::create(Rc::new(DidValue::List(items)))
        }
    }
}

pub fn rewrite_sequence(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let seq_value = SequenceValue { value: &context.term };
    if let Some(items) = seq_value.items().try_list() {
        let mut next_items: Vector<Rc<DidValue>> = Vector::new();
        let mut blocked = false;
        let mut prev_parallel = false;
        let mut last_result: Option<Rc<DidValue>> = None;
        for item in items {
            let item_value = ListItemValue { value: &item };
            if !blocked || prev_parallel {
                let item_context = context.with_term(Rc::clone(&item_value.term()));
                match rewrite_term(machine, item_context)? {
                    RewriteOk::Term(item_result) => {
                        if blocked {
                            let list_item = 
                                ListItemValue::create(
                                    Rc::clone(&item_result), 
                                    item_value.parallel()
                                );
                            next_items.push_back_mut(list_item);
                        };
                        last_result = Some(item_result);
                    },
                    RewriteOk::Blocked(item_blocked) => {
                        let list_item = 
                            ListItemValue::create(
                                Rc::clone(&item_blocked), 
                                item_value.parallel()
                            );
                        next_items.push_back_mut(list_item);
                        last_result = Some(item_blocked);
                        blocked = true;
                        prev_parallel = false;
                    },
                }
            } else {
                next_items.push_back_mut(Rc::clone(&item));
            }
            if let Some(parallel) = item_value.parallel().try_bool() {
                prev_parallel |= parallel;
            }
        };
        if blocked {
            if next_items.len() > 1 {
                Result::Ok(
                    RewriteOk::Blocked(
                        SequenceValue::create(
                            Rc::new(DidValue::List(next_items))    
                        ).as_hash()
                    )
                )
            } else {
                Result::Ok(RewriteOk::Blocked(last_result.unwrap()))
            }
        } else {
            Result::Ok(RewriteOk::Term(last_result.unwrap()))
        }
    } else {
        panic!("expected list")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    use sort_lookup::LookupValue;

    #[test]
    fn test_sequence_two_sequential() {
        // (succ 1) ; (succ 2)
        let program = SequenceValue::create(
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(1.into())), Rc::new(false.into())))
                    .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))
                )
            )
        );
        // 3
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(3.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_sequence_two_parallel() {
        // (succ 1) |; (succ 2)
        let program = SequenceValue::create(
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(1.into())), Rc::new(true.into())))
                    .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(2.into())), Rc::new(true.into())))
                )
            )
        );
        // 3
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(3.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_sequence_two_sequential_first_blocked() {
        // blocked ; (succ 2)
        let program = SequenceValue::create(
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(ListItemValue::create(LookupValue::create("blocked"), Rc::new(false.into())))
                    .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))
                )
            )
        );
        // blocked ; (succ 2)
        let result = run_rewrite(program.clone());
        let expected = Result::Ok(RewriteOk::Blocked(program));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_sequence_two_sequential_second_blocked() {
        // (succ 1) ; blocked
        let program = SequenceValue::create(
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(1.into())), Rc::new(false.into())))
                    .push_back(ListItemValue::create(LookupValue::create("blocked"), Rc::new(false.into())))
                )
            )
        );
        // blocked
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Blocked(LookupValue::create("blocked")));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_sequence_two_parallel_first_blocked() {
        // blocked |; (succ 2)
        let program = SequenceValue::create(
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(ListItemValue::create(LookupValue::create("blocked"), Rc::new(true.into())))
                    .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))
                )
            )
        );
        // blocked |; 3
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    SequenceValue::create(
                        Rc::new(
                            DidValue::List(
                                Vector::new()
                                .push_back(ListItemValue::create(LookupValue::create("blocked"), Rc::new(true.into())))
                                .push_back(ListItemValue::create(Rc::new(3.into()), Rc::new(false.into())))
                            )
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_sequence_two_parallel_second_blocked() {
        // (succ 1) |; blocked
        let program = SequenceValue::create(
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(1.into())), Rc::new(true.into())))
                    .push_back(ListItemValue::create(LookupValue::create("blocked"), Rc::new(false.into())))
                )
            )
        );
        // blocked
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Blocked(LookupValue::create("blocked")));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_sequence_three_parallel_second_blocked() {
        // (succ 1) |; blocked |; (succ 3)
        let program = SequenceValue::create(
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(1.into())), Rc::new(true.into())))
                    .push_back(ListItemValue::create(LookupValue::create("blocked"), Rc::new(true.into())))
                    .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(3.into())), Rc::new(false.into())))
                )
            )
        );
        // blocked |; 4
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    SequenceValue::create(
                        Rc::new(
                            DidValue::List(
                                Vector::new()
                                .push_back(ListItemValue::create(LookupValue::create("blocked"), Rc::new(true.into())))
                                .push_back(ListItemValue::create(Rc::new(4.into()), Rc::new(false.into())))
                            )
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_sequence_three_mixed_second_blocked() {
        // (succ 1) |; blocked ; (succ 3)
        let program = SequenceValue::create(
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(1.into())), Rc::new(true.into())))
                    .push_back(ListItemValue::create(LookupValue::create("blocked"), Rc::new(false.into())))
                    .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(3.into())), Rc::new(false.into())))
                )
            )
        );
        // blocked ; (succ 3)
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    SequenceValue::create(
                        Rc::new(
                            DidValue::List(
                                Vector::new()
                                .push_back(ListItemValue::create(LookupValue::create("blocked"), Rc::new(false.into())))
                                .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(3.into())), Rc::new(false.into())))
                            )
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }
}