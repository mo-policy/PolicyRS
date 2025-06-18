// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::cell::RefCell;
use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

pub struct CellValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> CellValue<'a> {
    pub fn value(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"value".to_string()).unwrap()) }

    pub fn create(value: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("value".to_string(), value);
        Rc::new(DidValue::Map(Sort::Cell, ht))
    }
}

pub fn rewrite_cell(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let cell = CellValue { value: &context.term };
    let value_context = context.with_term(Rc::clone(&cell.value()));
    match rewrite_term(machine, value_context)? {
        RewriteOk::Term(value_result) => {
            let cell = RefCell::new(value_result);
            Result::Ok(RewriteOk::Term(Rc::new(DidValue::Cell(cell)).as_hash()))
        },
        RewriteOk::Blocked(value_blocked) => {
            Result::Ok(
                RewriteOk::Blocked(
                    cell.value.with_map_value(
                        "value".to_string(), 
                        value_blocked
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

    #[test]
    fn test_cell() {
        // ref (succ 1)
        let program = CellValue::create(apply_tapl_succ(Rc::new(1.into())));
        // 2
        let result = run_rewrite(program);
        let cell: RefCell<Rc<DidValue>> = RefCell::new(Rc::new(2.into()));
        let expected = Result::Ok(RewriteOk::Term(Rc::new(DidValue::Cell(cell))));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_cell_blocked() {
        // ref (succ blocked)
        let program = 
            CellValue::create(
                apply_tapl_succ(LookupValue::create("blocked"))
            );
        // ref (succ blocked)
        let result = run_rewrite(program.clone());
        let expected = Result::Ok(RewriteOk::Blocked(program));
        assert_eq!(expected, result);
    }
}