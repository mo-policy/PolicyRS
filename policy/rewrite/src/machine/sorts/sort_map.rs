// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::{HashTrieMap, Vector};

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

pub struct MapValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> MapValue<'a> {
    pub fn items(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"items".to_string()).unwrap()) }

    pub fn create(items: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("items".to_string(), items);
        Rc::new(DidValue::Map(Sort::Map, ht))
    }

    pub fn create_for_parser(item_vec: Vec<Rc<DidValue>>, last: Option<Rc<DidValue>>) -> Rc<DidValue> {
        let mut items: Vector<Rc<DidValue>> = Vector::new();
        for item in item_vec {
            items.push_back_mut(item);
        }
        if last.is_some() {
            items.push_back_mut(last.unwrap());
        }
        MapValue::create(Rc::new(DidValue::List(items)))
    }
}
pub struct MapItemValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> MapItemValue<'a> {
    pub fn key(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"key".to_string()).unwrap()) }
    pub fn value(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"value".to_string()).unwrap()) }
    pub fn parallel(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"parallel".to_string()).unwrap()) }

    pub fn create(key: Rc<DidValue>, value: Rc<DidValue>, parallel: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("key".to_string(), key)
            .insert("value".to_string(), value)
            .insert("parallel".to_string(), parallel);
        Rc::new(DidValue::Map(Sort::Constant, ht))
    }
}

pub fn rewrite_map(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let map_value = MapValue { value: &context.term };
    if let Some(items) = map_value.items().try_list() {
        let mut items_blocked: Vector<Rc<DidValue>> = Vector::new();
        let mut items_result: HashTrieMap<String, Rc<DidValue>> = HashTrieMap::new();
        let mut blocked = false;
        let mut prev_parallel = false;
        for item in items {
            let item_value= MapItemValue { value: &item };
            if !blocked || prev_parallel {
                let item_value_context = context.with_term(Rc::clone(&item_value.value()));
                match rewrite_term(machine, item_value_context)? {
                    RewriteOk::Term(item_value_result) => {
                        let list_item = 
                            MapItemValue::create(
                                Rc::clone(&item_value.key()), 
                                Rc::clone(&item_value_result),
                                item_value.parallel()
                            );
                        items_blocked.push_back_mut(list_item);
                        if !blocked {
                            let key_value = item_value.key();
                            let Some(key) = key_value.try_string() else {
                                return Result::Err(RewriteErr::Exception(Rc::new("map key not string".into())))
                            };
                            items_result.insert_mut(key.clone(), item_value_result);
                        }
                    },
                    RewriteOk::Blocked(item_value_blocked) => {
                        let list_item = 
                            MapItemValue::create(
                                Rc::clone(&item_value.key()), 
                                Rc::clone(&item_value_blocked),
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
                    MapValue::create(
                        Rc::new(DidValue::List(items_blocked))
                    ).as_hash()
                )
            )
        } else {
            Result::Ok(RewriteOk::Term(Rc::new(DidValue::Map(Sort::Constant, items_result)).as_hash()))
        }
    } else {
        panic!("expected list")
    }
}

pub fn bind_map(machine: &mut RewriteMachine, context: RewriteContext, value: Rc<DidValue>) -> Result<MatchOk, RewriteErr> {
    if let Some(map_value) = value.try_map() {
        let mut pattern_bindings = context.bindings;
        let pattern_map = MapValue { value: &context.term };
        if let Some(pattern_items) = pattern_map.items().try_list() {
            for pattern_item in pattern_items {
                let pattern_item_value = MapItemValue { value: &pattern_item };
                let pattern_key = pattern_item_value.key();
                let pattern_value = pattern_item_value.value();
                let key_string = pattern_key.try_string().unwrap();
                if let Some(key_value) = map_value.get(key_string) {
                    let item_context = RewriteContext {
                        base: Rc::clone(&context.base),
                        term: Rc::clone(&pattern_value),
                        bindings: Rc::clone(&pattern_bindings),
                        policies: Rc::clone(&context.policies),
                    };
                    match term_bind(machine, item_context, Rc::clone(key_value)) {
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
            panic!("expected list")
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
    use rpds::HashTrieMap;

    #[test]
    fn test_map() {
        // { "a": (succ 1) ; "b": (succ 2) }
        let program = MapValue::create(
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(MapItemValue::create(Rc::new("a".into()), apply_tapl_succ(Rc::new(1.into())), Rc::new(false.into())))
                        .push_back(MapItemValue::create(Rc::new("b".into()), apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))
                    )
                )
            );
        // { "a": 2 ; "b": 3 }
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    Rc::new(
                        DidValue::Map(
                            Sort::Constant,
                            HashTrieMap::new()
                            .insert("a".into(), Rc::new(2.into()))
                            .insert("b".into(), Rc::new(3.into()))
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_map_empty() {
        // { }
        let program = MapValue::create(
                Rc::new(DidValue::List(Vector::new()))
            );
        // { }
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    Rc::new(DidValue::Map(Sort::Constant, HashTrieMap::new()))
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_map_parallel() {
        // { "a": (succ 1) |; "b": (succ 2) }
        let program = MapValue::create(
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(MapItemValue::create(Rc::new("a".into()), apply_tapl_succ(Rc::new(1.into())), Rc::new(true.into())))
                        .push_back(MapItemValue::create(Rc::new("b".into()), apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))
                    )
                )
            );
        // { "a": 2 ; "b": 3 }
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    Rc::new(
                        DidValue::Map(
                            Sort::Constant,
                            HashTrieMap::new()
                            .insert("a".into(), Rc::new(2.into()))
                            .insert("b".into(), Rc::new(3.into()))
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_map_sequential_blocked_first_value() {
        // { "a": (succ blocked) ; "b": (succ 2) }
        let program = MapValue::create(
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(MapItemValue::create(Rc::new("a".into()), apply_tapl_succ(LookupValue::create("blocked")), Rc::new(false.into())))
                        .push_back(MapItemValue::create(Rc::new("b".into()), apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))
                    )
                )
            );
        // { "a": (succ blocked) ; "b": (succ 2) }
        let result = run_rewrite(program.clone());
        let expected = Result::Ok(RewriteOk::Blocked(program));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_map_sequential_blocked_second_value() {
        // { "a": (succ 1) ; "b": (succ blocked) }
        let program = MapValue::create(
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(MapItemValue::create(Rc::new("a".into()), apply_tapl_succ(Rc::new(1.into())), Rc::new(false.into())))
                        .push_back(MapItemValue::create(Rc::new("b".into()), apply_tapl_succ(LookupValue::create("blocked")), Rc::new(false.into())))
                    )
                )
            );
        // { "a": 2 ; "b": (succ blocked) }
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    MapValue::create(
                Rc::new(
                        DidValue::List(
                            Vector::new()
                            .push_back(MapItemValue::create(Rc::new("a".into()), Rc::new(2.into()), Rc::new(false.into())))
                            .push_back(MapItemValue::create(Rc::new("b".into()), apply_tapl_succ(LookupValue::create("blocked")), Rc::new(false.into())))
                        )
                    )
                )
            )
        );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_map_parallel_blocked_first_value() {
        // { "a": (succ blocked) |; "b": (succ 2) }
        let program = MapValue::create(
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(MapItemValue::create(Rc::new("a".into()), apply_tapl_succ(LookupValue::create("blocked")), Rc::new(true.into())))
                        .push_back(MapItemValue::create(Rc::new("b".into()), apply_tapl_succ(Rc::new(2.into())), Rc::new(false.into())))
                    )
                )
            );
        // { "a": (succ blocked) |; "b": 3 }
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    MapValue::create(
                        Rc::new(
                            DidValue::List(
                                Vector::new()
                                .push_back(MapItemValue::create(Rc::new("a".into()), apply_tapl_succ(LookupValue::create("blocked")), Rc::new(true.into())))
                                .push_back(MapItemValue::create(Rc::new("b".into()), Rc::new(3.into()), Rc::new(false.into())))
                            )
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_map_parallel_blocked_second_value() {
        // { "a": (succ 1) |; "b": (succ blocked) }
        let program = MapValue::create(
                Rc::new(
                    DidValue::List(
                        Vector::new()
                        .push_back(MapItemValue::create(Rc::new("a".into()), apply_tapl_succ(Rc::new(1.into())), Rc::new(true.into())))
                        .push_back(MapItemValue::create(Rc::new("b".into()), apply_tapl_succ(LookupValue::create("blocked")), Rc::new(false.into())))
                    )
                )
            );
        // { "a": 2 |; "b": (succ blocked) }
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    MapValue::create(
                Rc::new(
                        DidValue::List(
                            Vector::new()
                            .push_back(MapItemValue::create(Rc::new("a".into()), Rc::new(2.into()), Rc::new(true.into())))
                            .push_back(MapItemValue::create(Rc::new("b".into()), apply_tapl_succ(LookupValue::create("blocked")), Rc::new(false.into())))
                        )
                    )
                )
            )
        );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_map_pattern_empty() {
        // match { x: true } with {} -> 1 _ -> 2
        let program = MatchValue::create(
            DidValue::new_map_constant().with_map_value("x".to_string(), Rc::new(true.into())),
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(
                        RuleValue::create(
                            MapValue::create(
                                Rc::new(
                                    DidValue::List(
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
    fn test_map_pattern_two() {
        // match { x: true, y: 1 } with { x: true, y: y} -> y _ -> 2
        let program = MatchValue::create(
            DidValue::new_map_constant()
                .with_map_value("x".to_string(), Rc::new(true.into()))
                .with_map_value("y".to_string(), Rc::new(1.into())),
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(
                        RuleValue::create(
                            MapValue::create(
                                Rc::new(
                                    DidValue::List(
                                        Vector::new()
                                        .push_back(MapItemValue::create(Rc::new("x".into()), Rc::new(true.into()), Rc::new(true.into())))
                                        .push_back(MapItemValue::create(Rc::new("y".into()), LookupValue::create("y"), Rc::new(false.into())))
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
    fn test_map_pattern_no_match() {
        // match { x: true, y: 1 } with { x: false, y: y} -> y _ -> 2
        let program = MatchValue::create(
            DidValue::new_map_constant()
                .with_map_value("x".to_string(), Rc::new(true.into()))
                .with_map_value("y".to_string(), Rc::new(1.into())),
            Rc::new(
                DidValue::List(
                    Vector::new()
                    .push_back(
                        RuleValue::create(
                            MapValue::create(
                                Rc::new(
                                    DidValue::List(
                                        Vector::new()
                                        .push_back(MapItemValue::create(Rc::new("x".into()), Rc::new(false.into()), Rc::new(true.into())))
                                        .push_back(MapItemValue::create(Rc::new("y".into()), LookupValue::create("y"), Rc::new(false.into())))
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
        // 2
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(2.into())));
        assert_eq!(expected, result);
    }

}