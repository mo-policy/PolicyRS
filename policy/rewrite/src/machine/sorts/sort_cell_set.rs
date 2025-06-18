// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

pub struct CellSetValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> CellSetValue<'a> {
    pub fn cell(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"cell".to_string()).unwrap()) }
    pub fn value(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"value".to_string()).unwrap()) }

    pub fn create(cell: Rc<DidValue>, value: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("cell".to_string(), cell)
            .insert("value".to_string(), value);
        Rc::new(DidValue::Map(Sort::CellSet, ht))
    }
}

pub fn rewrite_cell_set(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let cell_set = CellSetValue { value: &context.term };
    let cell_set_context = context.with_term(Rc::clone(&cell_set.cell()));
    match rewrite_term(machine, cell_set_context)? {
        RewriteOk::Term(cell_set_result) => {
            if let Some(cell) = cell_set_result.try_cell() {
                let value_context = context.with_term(Rc::clone(&cell_set.value()));
                match rewrite_term(machine, value_context)? {
                    RewriteOk::Term(value_result) => {
                        cell.replace(Rc::clone(&value_result));
                        Result::Ok(RewriteOk::Term(Rc::new(DidValue::Null).as_hash()))
                    },
                    RewriteOk::Blocked(value_blocked) => {
                        Result::Ok(
                            RewriteOk::Blocked(
                                CellSetValue::create(
                                    cell_set_result, 
                                    value_blocked
                                ).as_hash()
                            )
                        )
                    }
                }
            } else {
                panic!("expected cell")
            }
        },
        RewriteOk::Blocked(cell_set_blocked) => {
            Result::Ok(
                RewriteOk::Blocked(
                    cell_set.value.with_map_value(
                        "cell".to_string(), 
                        cell_set_blocked
                    ).as_hash()
                )
            )
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    use sort_lookup::LookupValue;
    use sort_cell::CellValue;
    use sort_cell_get::CellGetValue;
    use sort_let::LetValue;
    use sort_list::ListItemValue;
    use sort_sequence::SequenceValue;
    use std::cell::RefCell;
    use rpds::Vector;

    #[test]
    fn test_cell_set() {
        // let x = ref succ 1 in x := succ !x; !x end
        let program = 
            LetValue::create(
                false, 
                LookupValue::create("x"), 
                CellValue::create(
                    apply_tapl_succ(Rc::new(1.into()))
                ), 
                SequenceValue::create(
                    Rc::new(
                        DidValue::List(
                            Vector::new()
                            .push_back(
                                ListItemValue::create(
                                    CellSetValue::create(
                                        LookupValue::create("x"), 
                                        apply_tapl_succ(
                                            CellGetValue::create(
                                                LookupValue::create("x")
                                            )
                                        )
                                    ), 
                                    Rc::new(false.into())
                                ) 
                            ).push_back(
                                ListItemValue::create(
                                    CellGetValue::create(
                                        LookupValue::create("x")
                                    ), 
                                    Rc::new(false.into())
                                )
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
    fn test_cell_set_blocked_cell() {
        // ref blocked := 1
        let program = 
        CellSetValue::create(
            CellValue::create(
                LookupValue::create("blocked")
            ), 
            Rc::new(1.into())
        );
        // ref blocked := 1
        let result = run_rewrite(program.clone());
        let expected = Result::Ok(RewriteOk::Blocked(program));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_cell_set_blocked_value() {
        // ref 1 := blocked
        let program = 
            CellSetValue::create(
                CellValue::create(
                    Rc::new(1.into())
                ),
                LookupValue::create("blocked")
            );
        // ref 1 := blocked
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    CellSetValue::create(
                        Rc::new(
                            DidValue::Cell(
                                RefCell::new(
                                    Rc::new(1.into())
                                )
                            )
                        ),
                        LookupValue::create("blocked")
                    )
                )
            );
        assert_eq!(expected, result);
    }

}