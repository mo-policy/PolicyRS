// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use crate::data::DidValue;
use crate::data::sort::Sort;
use crate::machine::rewrite_machine::{RewriteContext, RewriteErr, RewriteMachine, RewriteOk };
use super::*;

pub fn std_comparison() -> Rc<DidValue> {
    let mut comparison_map: HashTrieMap<String, Rc<DidValue>> = HashTrieMap::new();
    comparison_map.insert_mut("eq".to_string(), std_comparison_eq());
    comparison_map.insert_mut("ne".to_string(), std_comparison_ne());
    comparison_map.insert_mut("gt".to_string(), std_comparison_gt());
    comparison_map.insert_mut("lt".to_string(), std_comparison_lt());
    comparison_map.insert_mut("ge".to_string(), std_comparison_ge());
    comparison_map.insert_mut("le".to_string(), std_comparison_le());
    comparison_map.insert_mut("not".to_string(), std_comparison_not());
    Rc::new(DidValue::Map(Sort::Constant, comparison_map)).as_hash()
}

fn comparison_args(context: &RewriteContext) -> BinaryArgs<&Rc<DidValue>, &Rc<DidValue>> {
    let Some(lhs) = context.bindings.try_map_value(&"lhs".to_string()) else {
        return BinaryArgs::Exception("expected 'lhs'".to_string());
    };
    let Some(rhs) = context.bindings.try_map_value(&"rhs".to_string()) else {
        return BinaryArgs::Exception("expected 'rhs'".to_string());
    };
    BinaryArgs::Args(lhs, rhs)
}

fn std_comparison_eq() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.eq(rhs)
    fn std_comparison_eq_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match comparison_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                let result = lhs.eq(rhs);
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::comparison::eq".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_comparison_eq_rewrite)), "lhs", "rhs")
}

fn std_comparison_ne() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.ne(rhs)
    fn std_comparison_ne_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match comparison_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                let result = lhs.ne(rhs);
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::comparison::ne".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_comparison_ne_rewrite)), "lhs", "rhs")
}

fn std_comparison_gt() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.gt(rhs)
    fn std_comparison_gt_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match comparison_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                let result = lhs.gt(rhs);
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::comparison::gt".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_comparison_gt_rewrite)), "lhs", "rhs")
}

fn std_comparison_lt() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.lt(rhs)
    fn std_comparison_lt_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match comparison_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                let result = lhs.lt(rhs);
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::comparison::lt".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_comparison_lt_rewrite)), "lhs", "rhs")
}

fn std_comparison_ge() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.ge(rhs)
    fn std_comparison_ge_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match comparison_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                let result = lhs.ge(rhs);
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::comparison::ge".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_comparison_ge_rewrite)), "lhs", "rhs")
}

fn std_comparison_le() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.le(rhs)
    fn std_comparison_le_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match comparison_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                let result = lhs.le(rhs);
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::comparison::le".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_comparison_le_rewrite)), "lhs", "rhs")
}

fn std_comparison_not() -> Rc<DidValue> {
    // fun b -> b.not()
    fn std_comparison_not_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        let b_value = context.bindings.try_map_value(&"b".to_string()).ok_or(RewriteErr::Exception(Rc::new("Arg 'b' not found".into())))?;
        let b = b_value.try_bool().ok_or(RewriteErr::Exception(Rc::new("Arg 'b' not bool".into())))?;
        Result::Ok(RewriteOk::Term(Rc::new((!b).into())))
    }
    rewrite_one(Rc::new(DidValue::Rewrite("std::comparison::not".to_string(), vec!["b".to_string()], std_comparison_not_rewrite)), "b")
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use rpds::{List, Vector};
    use super::*;
    use crate::machine::sorts::sort_dereference::DereferenceValue;
    use crate::machine::sorts::sort_list::ListItemValue;
    use crate::machine::sorts::sort_tuple::TupleValue;
    use crate::machine::uniform::apply_two;
    use crate::machine::rewrite_machine::{rewrite_term, RewriteOk };

    #[test]
    fn test_policy_comparison() {
        let std_document = policy_std();
        let std_did = std_document.try_document_did().unwrap();
        let eq_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["comparison", "eq"])), 
                Rc::new(1.into()), 
                Rc::new(1.into())
            );
        let ne_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["comparison", "ne"])), 
                Rc::new(1.into()), 
                Rc::new(2.into())
            );
        let gt_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["comparison", "gt"])), 
                Rc::new(2.into()), 
                Rc::new(1.into())
            );
        let lt_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["comparison", "lt"])), 
                Rc::new(1.into()), 
                Rc::new(2.into())
            );
        let ge_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["comparison", "ge"])), 
                Rc::new(1.into()), 
                Rc::new(1.into())
            );
        let le_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["comparison", "le"])), 
                Rc::new(1.into()), 
                Rc::new(1.into())
            );
        let not_term = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["comparison", "not"])), 
                Rc::new(false.into())
            );

        let program = 
            TupleValue::create(
                Rc::new(
                    DidValue::Tuple(
                        Vector::new()
                        .push_back(ListItemValue::create(eq_term, Rc::new(false.into())))            
                        .push_back(ListItemValue::create(ne_term, Rc::new(false.into())))            
                        .push_back(ListItemValue::create(gt_term, Rc::new(false.into())))            
                        .push_back(ListItemValue::create(lt_term, Rc::new(false.into())))            
                        .push_back(ListItemValue::create(ge_term, Rc::new(false.into())))            
                        .push_back(ListItemValue::create(le_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(not_term, Rc::new(false.into())))
                    )
                )
            );
        let mut machine = RewriteMachine::new();
        let context = RewriteContext {
            base: Rc::clone(&program),
            term: program,
            bindings: DidValue::new_map_constant(),
            policies: Rc::new(List::new()),
        };
        machine.docket.insert_mut((PathBuf::new(), std_document));
        let result = rewrite_term(&mut machine, context);
        let expected = 
            Result::Ok(
                RewriteOk::Term(
                    Rc::new(
                        DidValue::Tuple(
                            Vector::new()
                            .push_back(Rc::new(true.into()))
                            .push_back(Rc::new(true.into()))
                            .push_back(Rc::new(true.into()))
                            .push_back(Rc::new(true.into()))
                            .push_back(Rc::new(true.into()))
                            .push_back(Rc::new(true.into()))
                            .push_back(Rc::new(true.into()))
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

}
