// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use crate::data::DidValue;
use crate::machine::rewrite_machine::{RewriteContext, RewriteErr, RewriteMachine, RewriteOk };
use super::*;

pub fn std_integer() -> Rc<DidValue> {
    let mut integer_map: HashTrieMap<String, Rc<DidValue>> = HashTrieMap::new();
    integer_map.insert_mut("min".to_string(), std_integer_min());
    integer_map.insert_mut("max".to_string(), std_integer_max());
    integer_map.insert_mut("add".to_string(), std_integer_add());
    integer_map.insert_mut("div".to_string(), std_integer_div());
    integer_map.insert_mut("mul".to_string(), std_integer_mul());
    integer_map.insert_mut("rem".to_string(), std_integer_rem());
    integer_map.insert_mut("sub".to_string(), std_integer_sub());
    integer_map.insert_mut("pow".to_string(), std_integer_pow());
    integer_map.insert_mut("shl".to_string(), std_integer_shl());
    integer_map.insert_mut("shr".to_string(), std_integer_shr());
    integer_map.insert_mut("abs".to_string(), std_integer_abs());
    integer_map.insert_mut("neg".to_string(), std_integer_neg());
    Rc::new(DidValue::Map(Sort::Constant, integer_map)).as_hash()
}

fn integer_unary_args(context: &RewriteContext) -> UnaryArgs<i64> {
    let Some(lhs) = context.bindings.try_map_value(&"lhs".to_string()) else {
        return UnaryArgs::Exception(&"expected 'lhs'");
    };
    let Some(lhs_integer) = lhs.try_integer() else {
        return UnaryArgs::Exception(&"expected 'lhs' as integer");
    };
    UnaryArgs::Args(lhs_integer)
}

fn integer_binary_args(context: &RewriteContext) -> BinaryArgs<i64, i64> {
    let Some(lhs) = context.bindings.try_map_value(&"lhs".to_string()) else {
        return BinaryArgs::Exception("expected 'lhs'".to_string());
    };
    let Some(lhs_integer) = lhs.try_integer() else {
        return BinaryArgs::Exception("expected 'lhs' as integer".to_string());
    };
    let Some(rhs) = context.bindings.try_map_value(&"rhs".to_string()) else {
        return BinaryArgs::Exception("expected 'rhs'".to_string());
    };
    let Some(rhs_integer) = rhs.try_integer() else {
        return BinaryArgs::Exception("expected 'rhs' as integer".to_string());
    };
    BinaryArgs::Args(lhs_integer, rhs_integer)
}

fn std_integer_min() -> Rc<DidValue> {
    Rc::new(DidValue::Integer(i64::MIN))
}

fn std_integer_max() -> Rc<DidValue> {
    Rc::new(DidValue::Integer(i64::MAX))
}

fn std_integer_add() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.checked_add(rhs)
    fn std_integer_add_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match integer_binary_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                match lhs.checked_add(rhs) {
                    Some(result) => Result::Ok(RewriteOk::Term(Rc::new(result.into()))),
                    None => Result::Err(RewriteErr::Exception(Rc::new("overflow on add".into()))),
                }
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::integer::add".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_integer_add_rewrite)), "lhs", "rhs")
}

fn std_integer_div() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.checked_div(rhs)
    fn std_integer_div_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match integer_binary_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                match lhs.checked_div(rhs) {
                    Some(result) => Result::Ok(RewriteOk::Term(Rc::new(result.into()))),
                    None => Result::Err(RewriteErr::Exception(Rc::new("overflow on div".into()))),
                }
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::integer::div".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_integer_div_rewrite)), "lhs", "rhs")
}

fn std_integer_mul() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.checked_mul(rhs)
    fn std_integer_mul_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match integer_binary_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                match lhs.checked_mul(rhs) {
                    Some(result) => Result::Ok(RewriteOk::Term(Rc::new(result.into()))),
                    None => Result::Err(RewriteErr::Exception(Rc::new("overflow on mul".into()))),
                }
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::integer::mul".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_integer_mul_rewrite)), "lhs", "rhs")
}

fn std_integer_rem() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.checked_rem(rhs)
    fn std_integer_rem_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match integer_binary_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                match lhs.checked_rem(rhs) {
                    Some(result) => Result::Ok(RewriteOk::Term(Rc::new(result.into()))),
                    None => Result::Err(RewriteErr::Exception(Rc::new("overflow on rem".into()))),
                }
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::integer::rem".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_integer_rem_rewrite)), "lhs", "rhs")
}

fn std_integer_sub() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.checked_sub(rhs)
    fn std_integer_sub_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match integer_binary_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                match lhs.checked_sub(rhs) {
                    Some(result) => Result::Ok(RewriteOk::Term(Rc::new(result.into()))),
                    None => Result::Err(RewriteErr::Exception(Rc::new("overflow on sub".into()))),
                }
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::integer::sub".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_integer_sub_rewrite)), "lhs", "rhs")
}

fn std_integer_pow() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.checked_pow(rhs)
    fn std_integer_pow_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match integer_binary_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                match lhs.checked_pow(rhs as u32) {
                    Some(result) => Result::Ok(RewriteOk::Term(Rc::new(result.into()))),
                    None => Result::Err(RewriteErr::Exception(Rc::new("overflow on pow".into()))),
                }
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::integer::pow".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_integer_pow_rewrite)), "lhs", "rhs")
}

fn std_integer_shl() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.checked_shl(rhs)
    fn std_integer_shl_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match integer_binary_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                match lhs.checked_shl(rhs as u32) {
                    Some(result) => Result::Ok(RewriteOk::Term(Rc::new(result.into()))),
                    None => Result::Err(RewriteErr::Exception(Rc::new("overflow on shl".into()))),
                }
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::integer::shl".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_integer_shl_rewrite)), "lhs", "rhs")
}

fn std_integer_shr() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.checked_shr(rhs)
    fn std_integer_shr_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match integer_binary_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                match lhs.checked_shr(rhs as u32) {
                    Some(result) => Result::Ok(RewriteOk::Term(Rc::new(result.into()))),
                    None => Result::Err(RewriteErr::Exception(Rc::new("overflow on shr".into()))),
                }
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::integer::shr".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_integer_shr_rewrite)), "lhs", "rhs")
}

fn std_integer_abs() -> Rc<DidValue> {
    // fun lhs -> lhs.checked_abs()
    fn std_integer_abs_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match integer_unary_args(&context) {
            UnaryArgs::Args(lhs) => {
                match lhs.checked_abs() {
                    Some(result) => Result::Ok(RewriteOk::Term(Rc::new(result.into()))),
                    None => Result::Err(RewriteErr::Exception(Rc::new("error on abs".into()))),
                }
            },
            UnaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    unary_op(Rc::new(DidValue::Rewrite("std::integer::abs".to_string(), vec!["lhs".to_string()], std_integer_abs_rewrite)))
}

fn std_integer_neg() -> Rc<DidValue> {
    // fun lhs -> lhs.checked_neg()
    fn std_integer_neg_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match integer_unary_args(&context) {
            UnaryArgs::Args(lhs) => {
                match lhs.checked_neg() {
                    Some(result) => Result::Ok(RewriteOk::Term(Rc::new(result.into()))),
                    None => Result::Err(RewriteErr::Exception(Rc::new("error on neg".into()))),
                }
            },
            UnaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    unary_op(Rc::new(DidValue::Rewrite("std::integer::neg".to_string(), vec!["lhs".to_string()], std_integer_neg_rewrite)))
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
    use crate::machine::rewrite_machine::rewrite_term;

    #[test]
    fn test_policy_integer() {
        let std_document = policy_std();
        let std_did = std_document.try_document_did().unwrap();
        let min_term = 
            DereferenceValue::create(policy_url(std_did, vec!["integer", "min"]));
        let max_term = 
            DereferenceValue::create(policy_url(std_did, vec!["integer", "max"]));
        let add_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["integer", "add"])), 
                Rc::new(DidValue::Integer(1)), 
                Rc::new(DidValue::Integer(2))
            );
        let div_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["integer", "div"])), 
                Rc::new(DidValue::Integer(12)), 
                Rc::new(DidValue::Integer(2))
            );
        let mul_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["integer", "mul"])), 
                Rc::new(DidValue::Integer(12)), 
                Rc::new(DidValue::Integer(2))
            );
        let rem_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["integer", "rem"])), 
                Rc::new(DidValue::Integer(12)), 
                Rc::new(DidValue::Integer(5))
            );
        let sub_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["integer", "sub"])), 
                Rc::new(DidValue::Integer(12)), 
                Rc::new(DidValue::Integer(5))
            );
        let pow_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["integer", "pow"])), 
                Rc::new(DidValue::Integer(2)), 
                Rc::new(DidValue::Integer(3))
            );
        let shl_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["integer", "shl"])), 
                Rc::new(DidValue::Integer(1)), 
                Rc::new(DidValue::Integer(3))
            );
        let shr_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["integer", "shr"])), 
                Rc::new(DidValue::Integer(3)), 
                Rc::new(DidValue::Integer(1))
            );
        let abs_term = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["integer", "abs"])), 
                Rc::new(DidValue::Integer(-12))
            );
        let neg_term = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["integer", "neg"])), 
                Rc::new(DidValue::Integer(12))
            );
        let program = 
            TupleValue::create(
                Rc::new(
                    DidValue::Tuple(
                        Vector::new()
                        .push_back(ListItemValue::create(min_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(max_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(add_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(div_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(mul_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(rem_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(sub_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(pow_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(shl_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(shr_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(abs_term, Rc::new(false.into())))
                        .push_back(ListItemValue::create(neg_term, Rc::new(false.into())))
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
                            .push_back(Rc::new(DidValue::Integer(i64::MIN)))
                            .push_back(Rc::new(DidValue::Integer(i64::MAX)))
                            .push_back(Rc::new(DidValue::Integer(1 + 2)))
                            .push_back(Rc::new(DidValue::Integer(12 / 2)))
                            .push_back(Rc::new(DidValue::Integer(12 * 2)))
                            .push_back(Rc::new(DidValue::Integer(12 % 5)))
                            .push_back(Rc::new(DidValue::Integer(12 - 5)))
                            .push_back(Rc::new(DidValue::Integer(8)))
                            .push_back(Rc::new(DidValue::Integer(8)))
                            .push_back(Rc::new(DidValue::Integer(1)))
                            .push_back(Rc::new(DidValue::Integer(12)))
                            .push_back(Rc::new(DidValue::Integer(-12)))
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

}