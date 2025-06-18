// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::rc::Rc;
use crate::data::DidValue;
use crate::machine::rewrite_machine::{RewriteContext, RewriteErr, RewriteMachine, RewriteOk };
use super::*;

pub fn std_double() -> Rc<DidValue> {
    let mut double_map: HashTrieMap<String, Rc<DidValue>> = HashTrieMap::new();
    double_map.insert_mut("min".to_string(), std_double_min());
    double_map.insert_mut("max".to_string(), std_double_max());
    double_map.insert_mut("add".to_string(), std_double_add());
    double_map.insert_mut("div".to_string(), std_double_div());
    double_map.insert_mut("mul".to_string(), std_double_mul());
    double_map.insert_mut("rem".to_string(), std_double_rem());
    double_map.insert_mut("sub".to_string(), std_double_sub());
    double_map.insert_mut("pow".to_string(), std_double_pow());
    double_map.insert_mut("abs".to_string(), std_double_abs());
    double_map.insert_mut("neg".to_string(), std_double_neg());
    Rc::new(DidValue::Map(Sort::Constant, double_map)).as_hash()
}

fn double_unary_args(context: &RewriteContext) -> UnaryArgs<f64> {
    let Some(lhs) = context.bindings.try_map_value(&"lhs".to_string()) else {
        return UnaryArgs::Exception(&"expected 'lhs'");
    };
    let Some(lhs_double) = lhs.try_double() else {
        return UnaryArgs::Exception(&"expected 'lhs' as double");
    };
    UnaryArgs::Args(lhs_double)
}

fn double_binary_args(context: &RewriteContext) -> BinaryArgs<f64, f64> {
    let Some(lhs) = context.bindings.try_map_value(&"lhs".to_string()) else {
        return BinaryArgs::Exception("expected 'lhs'".to_string());
    };
    let Some(lhs_double) = lhs.try_double() else {
        return BinaryArgs::Exception("expected 'lhs' as double".to_string());
    };
    let Some(rhs) = context.bindings.try_map_value(&"rhs".to_string()) else {
        return BinaryArgs::Exception("expected 'rhs'".to_string());
    };
    let Some(rhs_double) = rhs.try_double() else {
        return BinaryArgs::Exception("expected 'rhs' as double".to_string());
    };
    BinaryArgs::Args(lhs_double, rhs_double)
}

fn std_double_min() -> Rc<DidValue> {
    Rc::new(DidValue::Double(f64::MIN))
}

fn std_double_max() -> Rc<DidValue> {
    Rc::new(DidValue::Double(f64::MAX))
}

fn std_double_add() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.add(rhs)
    fn std_double_add_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match double_binary_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                let result = lhs.add(rhs);
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::double::add".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_double_add_rewrite)), "lhs", "rhs")
}

fn std_double_div() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.checked_div(rhs)
    fn std_double_div_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match double_binary_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                let result = lhs.div(rhs);
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::double::div".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_double_div_rewrite)), "lhs", "rhs")
}

fn std_double_mul() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.mul(rhs)
    fn std_double_mul_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match double_binary_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                let result = lhs.mul(rhs);
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::double::mul".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_double_mul_rewrite)), "lhs", "rhs")
}

fn std_double_rem() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.rem(rhs)
    fn std_double_rem_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match double_binary_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                let result = lhs.rem(rhs);
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::double::rem".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_double_rem_rewrite)), "lhs", "rhs")
}

fn std_double_sub() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.sub(rhs)
    fn std_double_sub_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match double_binary_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                let result = lhs.sub(rhs);
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::double::sub".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_double_sub_rewrite)), "lhs", "rhs")
}

fn std_double_pow() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.powf(rhs)
    fn std_double_pow_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match double_binary_args(&context) {
            BinaryArgs::Args(lhs, rhs) => {
                let result = lhs.powf(rhs);
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            BinaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::double::pow".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_double_pow_rewrite)), "lhs", "rhs")
}

fn std_double_abs() -> Rc<DidValue> {
    // fun lhs -> lhs.abs()
    fn std_double_abs_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match double_unary_args(&context) {
            UnaryArgs::Args(lhs) => {
                let result = lhs.abs();
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            UnaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    unary_op(Rc::new(DidValue::Rewrite("std::double::abs".to_string(), vec!["lhs".to_string()], std_double_abs_rewrite)))
}

fn std_double_neg() -> Rc<DidValue> {
    // fun lhs -> lhs.neg()
    fn std_double_neg_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        match double_unary_args(&context) {
            UnaryArgs::Args(lhs) => {
                let result = lhs.neg();
                Result::Ok(RewriteOk::Term(Rc::new(result.into())))
            },
            UnaryArgs::Exception(msg) => {
                Result::Err(RewriteErr::Exception(Rc::new(msg.into())))
            }
        }
    }
    unary_op(Rc::new(DidValue::Rewrite("std::double::neg".to_string(), vec!["lhs".to_string()], std_double_neg_rewrite)))
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
    fn test_policy_double() {
        let std_document = policy_std();
        let std_did = std_document.try_document_did().unwrap();
        let min_term = 
            DereferenceValue::create(policy_url(std_did, vec!["double", "min"]));
        let max_term = 
            DereferenceValue::create(policy_url(std_did, vec!["double", "max"]));
        let add_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["double", "add"])), 
                Rc::new(DidValue::Double(1.1)), 
                Rc::new(DidValue::Double(2.2))
            );
        let div_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["double", "div"])), 
                Rc::new(DidValue::Double(12.6)), 
                Rc::new(DidValue::Double(2.0))
            );
        let mul_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["double", "mul"])), 
                Rc::new(DidValue::Double(12.5)), 
                Rc::new(DidValue::Double(2.4))
            );
        let rem_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["double", "rem"])), 
                Rc::new(DidValue::Double(12.2)), 
                Rc::new(DidValue::Double(5.1))
            );
        let sub_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["double", "sub"])), 
                Rc::new(DidValue::Double(12.8)), 
                Rc::new(DidValue::Double(5.2))
            );
        let pow_term = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["double", "pow"])), 
                Rc::new(DidValue::Double(2.25)), 
                Rc::new(DidValue::Double(3.1))
            );
        let abs_term = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["double", "abs"])), 
                Rc::new((-12.3).into())
            );
        let neg_term = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["double", "neg"])), 
                Rc::new((12.1).into())
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
                            .push_back(Rc::new(DidValue::Double(f64::MIN)))
                            .push_back(Rc::new(DidValue::Double(f64::MAX)))
                            .push_back(Rc::new(DidValue::Double(1.1 + 2.2)))
                            .push_back(Rc::new(DidValue::Double(12.6 / 2.0)))
                            .push_back(Rc::new(DidValue::Double(12.5 * 2.4)))
                            .push_back(Rc::new(DidValue::Double(12.2 % 5.1)))
                            .push_back(Rc::new(DidValue::Double(12.8 - 5.2)))
                            .push_back(Rc::new(DidValue::Double((2.25_f64).powf(3.1_f64))))
                            .push_back(Rc::new(DidValue::Double(12.3)))
                            .push_back(Rc::new(DidValue::Double(-12.1)))
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

}