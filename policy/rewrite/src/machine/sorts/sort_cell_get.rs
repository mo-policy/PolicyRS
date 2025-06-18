// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

pub struct CellGetValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> CellGetValue<'a> {
    pub fn cell(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"cell".to_string()).unwrap()) }

    pub fn create(cell: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("cell".to_string(), cell);
        Rc::new(DidValue::Map(Sort::CellGet, ht))
    }
}

pub fn rewrite_cell_get(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let cell_get = CellGetValue { value: &context.term };
    let cell_get_context = context.with_term(Rc::clone(&cell_get.cell()));
    match rewrite_term(machine, cell_get_context)? {
        RewriteOk::Term(cell_get_result) => {
            if let Some(cell) = cell_get_result.try_cell() {
                Result::Ok(RewriteOk::Term(cell.borrow().as_hash()))
            } else {
                panic!("expected cell")
            }
        },
        RewriteOk::Blocked(cell_get_blocked) => {
            Result::Ok(
                RewriteOk::Blocked(
                    cell_get.value.with_map_value(
                        "cell".to_string(), 
                        cell_get_blocked
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
    use sort_application::ApplicationValue;
    use sort_lookup::LookupValue;
    use sort_cell::CellValue;

    #[test]
    fn test_cell_get() {
        // !(ref (succ 1))
        let program = 
            CellGetValue::create(
                CellValue::create(apply_tapl_succ(Rc::new(1.into())))
            );
        // 2
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(2.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_cell_get_blocked() {
        // !(ref (succ blocked))
        let program = 
            CellGetValue::create(
                CellValue::create(
                    apply_tapl_succ(LookupValue::create("blocked"))
                )
            );
        // !(ref (succ blocked))
        let result = run_rewrite(program.clone());
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    CellGetValue::create(
                        CellValue::create(
                            ApplicationValue::create(
                                tapl_succ(), 
                                LookupValue::create("blocked")
                            )
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

}