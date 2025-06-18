// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::{HashTrieMap, HashTrieSet, Vector};

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

use super::sort_list::ListItemValue;
pub struct SetValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> SetValue<'a> {
    pub fn items(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"items".to_string()).unwrap()) }

    pub fn create(items: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("items".to_string(), items);
        Rc::new(DidValue::Map(Sort::Set, ht))
    }

    pub fn create_for_parser(item_vec: Vec<Rc<DidValue>>, last: Option<Rc<DidValue>>) -> Rc<DidValue> {
        let mut items: Vector<Rc<DidValue>> = Vector::new();
        for item in item_vec {
            items.push_back_mut(item);
        }
        if last.is_some() {
            let last_item = ListItemValue::create(last.unwrap(), Rc::new(false.into()));
            items.push_back_mut(last_item);
        }
        SetValue::create(Rc::new(DidValue::List(items)))
    }

}

pub fn rewrite_set(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let set_value = SetValue { value: &context.term };
    if let Some(items) = set_value.items().try_list() {
        let mut items_blocked: Vector<Rc<DidValue>> = Vector::new();
        let mut items_result: HashTrieSet<Rc<DidValue>> = HashTrieSet::new();
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
                            items_result.insert_mut(item_result);
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
                    SetValue::create(
                        Rc::new(DidValue::List(items_blocked))
                    ).as_hash()
                )
            )
        } else {
            Result::Ok(RewriteOk::Term(Rc::new(DidValue::Set(items_result)).as_hash()))
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
    use rpds::{HashTrieSet, Vector};

    #[test]
    fn test_set() {
        // {[ (succ 1) ; (succ 2) ]}
        let program = SetValue::create(
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(1.into())), Rc::new(false.into())))
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))            
                    )
                )
            );
        // {[ 2; 3 ]}
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    Rc::new(
                        DidValue::Set(
                            HashTrieSet::new()
                            .insert(Rc::new(2.into()))
                            .insert(Rc::new(3.into()))
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_set_empty() {
        // {[ ]}
        let program = SetValue::create(
                Rc::new(DidValue::List(Vector::new()))
            );
        // {[ ]}
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    Rc::new(
                        DidValue::Set(HashTrieSet::new())
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_set_parallel() {
        // {[ (succ 1) |, (succ 2) ]}
        let program = SetValue::create(
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(1.into())), Rc::new(true.into())))
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))            
                    )
                )
            );
        // {[ 2; 3 ]}
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    Rc::new(
                        DidValue::Set(
                            HashTrieSet::new()
                            .insert(Rc::new(2.into()))
                            .insert(Rc::new(3.into()))
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_set_parallel_blocked_first_item() {
        // {[ (succ blocked) |, (succ 2) ]}
        let program = SetValue::create(
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(ListItemValue::create(apply_tapl_succ(LookupValue::create("blocked")), Rc::new(true.into())))
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))            
                    )
                )
            );
        // {[ (succ blocked) |, 3 ]}
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    SetValue::create(
                    Rc::new(
                        DidValue::List(
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
    fn test_set_parallel_blocked_second_item() {
        // {[ (succ 1) |, (succ blocked) ]}
        let program = SetValue::create(
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(1.into())), Rc::new(true.into())))            
                        .push_back(ListItemValue::create(apply_tapl_succ(LookupValue::create("blocked")), Rc::new(false.into())))
                    )
                )
            );
        // {[ 2 |, (succ blocked) ]}
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    SetValue::create(
                    Rc::new(
                        DidValue::List(
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
    fn test_set_sequential_blocked_first_item() {
        // {[ (succ blocked) , (succ 2) ]}
        let program = SetValue::create(
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(ListItemValue::create(apply_tapl_succ(LookupValue::create("blocked")), Rc::new(false.into())))
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))            
                    )
                )
            );
        // {[ (succ blocked) |, (succ 2) ]}
        let result = run_rewrite(program.clone());
        let expected = Result::Ok(RewriteOk::Blocked(program));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_set_sequential_blocked_second_item() {
        // {[ (succ 1) , (succ blocked) ]}
        let program = SetValue::create(
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(ListItemValue::create(apply_tapl_succ(Rc::new(1.into())), Rc::new(false.into())))            
                        .push_back(ListItemValue::create(apply_tapl_succ(LookupValue::create("blocked")), Rc::new(false.into())))
                    )
                )
            );
        // {[ 2 , (succ blocked) ]}
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    SetValue::create(
                    Rc::new(
                        DidValue::List(
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

}