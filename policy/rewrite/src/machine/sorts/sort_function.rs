// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::{HashTrieMap, Vector};

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;
pub struct FunctionValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> FunctionValue<'a> {
    pub fn pattern(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"pattern".to_string()).unwrap()) }
    pub fn term(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"term".to_string()).unwrap()) }
    pub fn bindings(&self) -> Option<Rc<DidValue>>   { 
        let Some(bindings_value) = self.value.try_map_value(&"bindings".to_string()) else {
            return None
        };
        Some(Rc::clone(bindings_value))
    }
    pub fn free(&self) -> Option<Rc<DidValue>>   { 
        let Some(free_value) = self.value.try_map_value(&"free".to_string()) else {
            return None
        };
        Some(Rc::clone(free_value))
    }

    pub fn create(pattern: Rc<DidValue>, term: Rc<DidValue>, bindings: Option<Rc<DidValue>>, free: Option<Rc<DidValue>>) -> Rc<DidValue> {
        let mut ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("pattern".to_string(), pattern)
            .insert("term".to_string(), term);
        if let Some(b) = bindings {
            ht.insert_mut("bindings".to_string(), b);
        };
        if let Some(f) = free {
            ht.insert_mut("free".to_string(), f);
        };
        Rc::new(DidValue::Map(Sort::Function, ht))
    }
}

pub fn rewrite_function(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let function_value = FunctionValue { value: &context.term };
    let mut changed = false;
    let mut has_bindings_and_free = true;
    let mut function_bindings: HashTrieMap<String, Rc<DidValue>> = 
        if function_value.bindings().is_none() {
            has_bindings_and_free = false;
            HashTrieMap::new()
        } else {
            function_value.bindings().unwrap().try_map().unwrap().clone()
        };
    let mut function_free: Vector<Rc<DidValue>> = 
        if function_value.free().is_none() {
            has_bindings_and_free = false;
            Vector::new()
        } else {
            function_value.free().unwrap().try_list().unwrap().clone()
        };
    if !has_bindings_and_free {
        // generate closure
        let all_free = free_names(machine, &context.term, HashTrieMap::new(), HashTrieMap::new());
        for name in all_free.keys() {
            if let Some(binding) = context.bindings.try_map_value(name) {
                function_bindings.insert_mut(name.clone(), Rc::clone(binding));
            } else {
                function_free.push_back_mut(Rc::new(name.clone().into()));
            }
        }
        let function_with_bindings = 
            function_value.value.with_map_value(
                "bindings".to_string(), 
                Rc::new(DidValue::Map(Sort::Constant, function_bindings))
            ).with_map_value(
                "free".to_string(), 
                Rc::new(DidValue::List(function_free))
            );
        Result::Ok(RewriteOk::Term(function_with_bindings.as_hash()))
    } else {
        let mut next_free: Vector<Rc<DidValue>> = Vector::new();
        for fv in function_free.iter() {
            let Some(name) = fv.try_string() else {
                panic!()
            };
            assert!(!function_bindings.contains_key(name));
            if let Some(bv) = context.bindings.try_map_value(name) {
                function_bindings.insert_mut(name.clone(), Rc::clone(bv));
                changed = true;
            } else {
                next_free.push_back_mut(Rc::clone(fv));
            }
        }
        if changed {
            Result::Ok(
                RewriteOk::Term(
                    function_value.value.with_map_value(
                    "bindings".to_string(), 
                    Rc::new(DidValue::Map(Sort::Constant, function_bindings))
                    ).with_map_value(
                        "free".to_string(), 
                        Rc::new(DidValue::List(next_free))
                    ).as_hash()
                )
            )
        } else {
            Result::Ok(RewriteOk::Term(context.term))
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    use sort_let::LetValue;
    use sort_lookup::LookupValue;

    #[test]
    fn test_function() {
        // fun x -> x
        let program = FunctionValue::create(
            LookupValue::create("x"),
            LookupValue::create("x"),
            None,
            None
        );
        // fun x -> x [ bindings: {} ]
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    FunctionValue::create(
                        LookupValue::create("x"),
                        LookupValue::create("x"),
                        Some(DidValue::new_map_constant()),
                        None
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn test_function_closure() {
        // let y = 1 in fun x -> y
        let program = 
            LetValue::create(
                false, 
                LookupValue::create("y"), 
                Rc::new(1.into()), 
                FunctionValue::create(
                    LookupValue::create("x"),
                    LookupValue::create("y"),
                    None,
                    None
                )
            );
        // fun x -> y [bindings: {y: 1} ]
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    FunctionValue::create(
                        LookupValue::create("x"),
                        LookupValue::create("y"),
                        Some(Rc::new(DidValue::Map(Sort::Constant, HashTrieMap::new().insert("y".to_string(), Rc::new(1.into()))))),
                        None
                    )
                )
            );
        assert_eq!(expected, result);
    }

}
