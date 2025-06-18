// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::rc::Rc;
use crate::data::DidValue;
use crate::machine::rewrite_machine::{RewriteContext, RewriteErr, RewriteMachine, RewriteOk };
use super::*;

pub fn std_math() -> Rc<DidValue> {
    let mut math_map: HashTrieMap<String, Rc<DidValue>> = HashTrieMap::new();
    math_map.insert_mut("add".to_string(), std_math_add());
    math_map.insert_mut("div".to_string(), std_math_div());
    math_map.insert_mut("mul".to_string(), std_math_mul());
    math_map.insert_mut("rem".to_string(), std_math_rem());
    math_map.insert_mut("sub".to_string(), std_math_sub());
    math_map.insert_mut("pow".to_string(), std_math_pow());
    math_map.insert_mut("abs".to_string(), std_math_abs());
    math_map.insert_mut("neg".to_string(), std_math_neg());
    Rc::new(DidValue::Map(Sort::Constant, math_map)).as_hash()
}

fn std_math_add() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.add(rhs)
    fn std_math_add_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        let Some(lhs) = context.bindings.try_map_value(&"lhs".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("expected 'lhs'".into())));
        };
        let Some(rhs) = context.bindings.try_map_value(&"rhs".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("expected 'rhs'".into())));
        };
        if let Some(lhs_integer) = lhs.try_integer() {
            if let Some(rhs_integer) = rhs.try_integer() {
                let result = lhs_integer.add(rhs_integer);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            } else if let Some(rhs_double) = rhs.try_double() {
                let result = (lhs_integer as f64).add(rhs_double);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            }
        } else if let Some(lhs_double) = lhs.try_double() {
            if let Some(rhs_double) = rhs.try_double() {
                let result = lhs_double.add(rhs_double);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            } else if let Some(rhs_integer) = rhs.try_integer() {
                let result = lhs_double.add(rhs_integer as f64);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            }
        }
        Result::Err(RewriteErr::Exception(Rc::new("unexpected types for 'lhs' or 'rhs'".into())))
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::math::add".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_math_add_rewrite)), "lhs", "rhs")
}

fn std_math_div() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.checked_div(rhs)
    fn std_math_div_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        let Some(lhs) = context.bindings.try_map_value(&"lhs".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("expected 'lhs'".into())));
        };
        let Some(rhs) = context.bindings.try_map_value(&"rhs".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("expected 'rhs'".into())));
        };
        if let Some(lhs_integer) = lhs.try_integer() {
            if let Some(rhs_integer) = rhs.try_integer() {
                let result = lhs_integer.div(rhs_integer);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            } else if let Some(rhs_double) = rhs.try_double() {
                let result = (lhs_integer as f64).div(rhs_double);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            }
        } else if let Some(lhs_double) = lhs.try_double() {
            if let Some(rhs_double) = rhs.try_double() {
                let result = lhs_double.div(rhs_double);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            } else if let Some(rhs_integer) = rhs.try_integer() {
                let result = lhs_double.div(rhs_integer as f64);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            }
        }
        Result::Err(RewriteErr::Exception(Rc::new("unexpected types for 'lhs' or 'rhs'".into())))
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::math::div".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_math_div_rewrite)), "lhs", "rhs")
}

fn std_math_mul() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.mul(rhs)
    fn std_math_mul_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        let Some(lhs) = context.bindings.try_map_value(&"lhs".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("expected 'lhs'".into())));
        };
        let Some(rhs) = context.bindings.try_map_value(&"rhs".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("expected 'rhs'".into())));
        };
        if let Some(lhs_integer) = lhs.try_integer() {
            if let Some(rhs_integer) = rhs.try_integer() {
                let result = lhs_integer.mul(rhs_integer);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            } else if let Some(rhs_double) = rhs.try_double() {
                let result = (lhs_integer as f64).mul(rhs_double);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            }
        } else if let Some(lhs_double) = lhs.try_double() {
            if let Some(rhs_double) = rhs.try_double() {
                let result = lhs_double.mul(rhs_double);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            } else if let Some(rhs_integer) = rhs.try_integer() {
                let result = lhs_double.mul(rhs_integer as f64);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            }
        }
        Result::Err(RewriteErr::Exception(Rc::new("unexpected types for 'lhs' or 'rhs'".into())))
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::math::mul".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_math_mul_rewrite)), "lhs", "rhs")
}

fn std_math_rem() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.rem(rhs)
    fn std_math_rem_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        let Some(lhs) = context.bindings.try_map_value(&"lhs".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("expected 'lhs'".into())));
        };
        let Some(rhs) = context.bindings.try_map_value(&"rhs".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("expected 'rhs'".into())));
        };
        if let Some(lhs_integer) = lhs.try_integer() {
            if let Some(rhs_integer) = rhs.try_integer() {
                let result = lhs_integer.rem(rhs_integer);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            } else if let Some(rhs_double) = rhs.try_double() {
                let result = (lhs_integer as f64).rem(rhs_double);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            }
        } else if let Some(lhs_double) = lhs.try_double() {
            if let Some(rhs_double) = rhs.try_double() {
                let result = lhs_double.rem(rhs_double);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            } else if let Some(rhs_integer) = rhs.try_integer() {
                let result = lhs_double.rem(rhs_integer as f64);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            }
        }
        Result::Err(RewriteErr::Exception(Rc::new("unexpected types for 'lhs' or 'rhs'".into())))
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::math::rem".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_math_rem_rewrite)), "lhs", "rhs")
}

fn std_math_sub() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.sub(rhs)
    fn std_math_sub_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        let Some(lhs) = context.bindings.try_map_value(&"lhs".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("expected 'lhs'".into())));
        };
        let Some(rhs) = context.bindings.try_map_value(&"rhs".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("expected 'rhs'".into())));
        };
        if let Some(lhs_integer) = lhs.try_integer() {
            if let Some(rhs_integer) = rhs.try_integer() {
                let result = lhs_integer.sub(rhs_integer);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            } else if let Some(rhs_double) = rhs.try_double() {
                let result = (lhs_integer as f64).sub(rhs_double);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            }
        } else if let Some(lhs_double) = lhs.try_double() {
            if let Some(rhs_double) = rhs.try_double() {
                let result = lhs_double.sub(rhs_double);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            } else if let Some(rhs_integer) = rhs.try_integer() {
                let result = lhs_double.sub(rhs_integer as f64);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            }
        }
        Result::Err(RewriteErr::Exception(Rc::new("unexpected types for 'lhs' or 'rhs'".into())))
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::math::sub".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_math_sub_rewrite)), "lhs", "rhs")
}

fn std_math_pow() -> Rc<DidValue> {
    // fun lhs -> fun rhs -> lhs.powf(rhs)
    fn std_math_pow_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        let Some(lhs) = context.bindings.try_map_value(&"lhs".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("expected 'lhs'".into())));
        };
        let Some(rhs) = context.bindings.try_map_value(&"rhs".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("expected 'rhs'".into())));
        };
        if let Some(lhs_integer) = lhs.try_integer() {
            if let Some(rhs_integer) = rhs.try_integer() {
                let result = lhs_integer.pow(rhs_integer as u32);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            } else if let Some(rhs_double) = rhs.try_double() {
                let result = (lhs_integer as f64).powf(rhs_double);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            }
        } else if let Some(lhs_double) = lhs.try_double() {
            if let Some(rhs_double) = rhs.try_double() {
                let result = lhs_double.powf(rhs_double);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            } else if let Some(rhs_integer) = rhs.try_integer() {
                let result = lhs_double.powf(rhs_integer as f64);
                return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
            }
        }
        Result::Err(RewriteErr::Exception(Rc::new("unexpected types for 'lhs' or 'rhs'".into())))
    }
    rewrite_two(Rc::new(DidValue::Rewrite("std::math::pow".to_string(), vec!["lhs".to_string(), "rhs".to_string()], std_math_pow_rewrite)), "lhs", "rhs")
}

fn std_math_abs() -> Rc<DidValue> {
    // fun lhs -> lhs.abs()
    fn std_math_abs_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        let Some(lhs) = context.bindings.try_map_value(&"lhs".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("expected 'lhs'".into())));
        };
        if let Some(lhs_integer) = lhs.try_integer() {
            let result = lhs_integer.abs();
            return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
        } else if let Some(lhs_double) = lhs.try_double() {
            let result = lhs_double.abs();
            return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
        }
        Result::Err(RewriteErr::Exception(Rc::new("unexpected type for 'lhs'".into())))
    }
    unary_op(Rc::new(DidValue::Rewrite("std::math::abs".to_string(), vec!["lhs".to_string()], std_math_abs_rewrite)))
}

fn std_math_neg() -> Rc<DidValue> {
    // fun lhs -> lhs.neg()
    fn std_math_neg_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        let Some(lhs) = context.bindings.try_map_value(&"lhs".to_string()) else {
            return Result::Err(RewriteErr::Exception(Rc::new("expected 'lhs'".into())));
        };
        if let Some(lhs_integer) = lhs.try_integer() {
            let result = lhs_integer.neg();
            return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
        } else if let Some(lhs_double) = lhs.try_double() {
            let result = lhs_double.neg();
            return Result::Ok(RewriteOk::Term(Rc::new(result.into())));
        }
        Result::Err(RewriteErr::Exception(Rc::new("unexpected type for 'lhs'".into())))
    }
    unary_op(Rc::new(DidValue::Rewrite("std::math::neg".to_string(), vec!["lhs".to_string()], std_math_neg_rewrite)))
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
    fn test_policy_math() {
        let std_document = policy_std();
        let std_did = std_document.try_document_did().unwrap();
        let add_term_dd = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "add"])), 
                Rc::new(DidValue::Double(1.1)), 
                Rc::new(DidValue::Double(2.2))
            );
        let add_term_di = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "add"])), 
                Rc::new(DidValue::Double(1.1)), 
                Rc::new(DidValue::Integer(2))
            );
        let add_term_id = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "add"])), 
                Rc::new(DidValue::Integer(1)), 
                Rc::new(DidValue::Double(2.2))
            );
        let add_term_ii = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "add"])), 
                Rc::new(DidValue::Integer(1)), 
                Rc::new(DidValue::Integer(2))
            );
        let div_term_dd = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "div"])), 
                Rc::new(DidValue::Double(12.6)), 
                Rc::new(DidValue::Double(2.0))
            );
        let div_term_di = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "div"])), 
                Rc::new(DidValue::Double(12.6)), 
                Rc::new(DidValue::Integer(2))
            );
        let div_term_id = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "div"])), 
                Rc::new(DidValue::Integer(12)), 
                Rc::new(DidValue::Double(2.0))
            );
        let div_term_ii = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "div"])), 
                Rc::new(DidValue::Integer(12)), 
                Rc::new(DidValue::Integer(2))
            );
        let mul_term_dd = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "mul"])), 
                Rc::new(DidValue::Double(12.5)), 
                Rc::new(DidValue::Double(2.4))
            );
        let mul_term_di = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "mul"])), 
                Rc::new(DidValue::Double(12.5)), 
                Rc::new(DidValue::Integer(2))
            );
        let mul_term_id = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "mul"])), 
                Rc::new(DidValue::Integer(12)), 
                Rc::new(DidValue::Double(2.4))
            );
        let mul_term_ii = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "mul"])), 
                Rc::new(DidValue::Integer(12)), 
                Rc::new(DidValue::Integer(2))
            );
        let rem_term_dd = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "rem"])), 
                Rc::new(DidValue::Double(12.2)), 
                Rc::new(DidValue::Double(5.1))
            );
        let rem_term_di = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "rem"])), 
                Rc::new(DidValue::Double(12.2)), 
                Rc::new(DidValue::Integer(5))
            );
        let rem_term_id = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "rem"])), 
                Rc::new(DidValue::Integer(12)), 
                Rc::new(DidValue::Double(5.1))
            );
        let rem_term_ii = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "rem"])), 
                Rc::new(DidValue::Integer(12)), 
                Rc::new(DidValue::Integer(5))
            );
        let sub_term_dd = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "sub"])), 
                Rc::new(DidValue::Double(12.8)), 
                Rc::new(DidValue::Double(5.2))
            );
        let sub_term_di = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "sub"])), 
                Rc::new(DidValue::Double(12.8)), 
                Rc::new(DidValue::Integer(5))
            );
        let sub_term_id = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "sub"])), 
                Rc::new(DidValue::Integer(12)), 
                Rc::new(DidValue::Double(5.2))
            );
        let sub_term_ii = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "sub"])), 
                Rc::new(DidValue::Integer(12)), 
                Rc::new(DidValue::Integer(5))
            );
        let pow_term_dd = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "pow"])), 
                Rc::new(DidValue::Double(2.25)), 
                Rc::new(DidValue::Double(3.1))
            );
        let pow_term_di = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "pow"])), 
                Rc::new(DidValue::Double(2.25)), 
                Rc::new(DidValue::Integer(3))
            );
        let pow_term_id = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "pow"])), 
                Rc::new(DidValue::Integer(2)), 
                Rc::new(DidValue::Double(3.1))
            );
        let pow_term_ii = 
            apply_two(
                DereferenceValue::create(policy_url(std_did, vec!["math", "pow"])), 
                Rc::new(DidValue::Integer(2)), 
                Rc::new(DidValue::Integer(3))
            );
        let abs_term_d = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["math", "abs"])), 
                Rc::new((-12.3).into())
            );
        let abs_term_i = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["math", "abs"])), 
                Rc::new((-12).into())
            );
        let neg_term_d = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["math", "neg"])), 
                Rc::new((12.1).into())
            );
        let neg_term_i = 
            apply_one(
                DereferenceValue::create(policy_url(std_did, vec!["math", "neg"])), 
                Rc::new(12.into())
            );
        let program = 
            TupleValue::create(
                Rc::new(
                    DidValue::Tuple(
                        Vector::new()
                        .push_back(ListItemValue::create(add_term_dd, Rc::new(false.into())))
                        .push_back(ListItemValue::create(add_term_di, Rc::new(false.into())))
                        .push_back(ListItemValue::create(add_term_id, Rc::new(false.into())))
                        .push_back(ListItemValue::create(add_term_ii, Rc::new(false.into())))
                        .push_back(ListItemValue::create(div_term_dd, Rc::new(false.into())))
                        .push_back(ListItemValue::create(div_term_di, Rc::new(false.into())))
                        .push_back(ListItemValue::create(div_term_id, Rc::new(false.into())))
                        .push_back(ListItemValue::create(div_term_ii, Rc::new(false.into())))
                        .push_back(ListItemValue::create(mul_term_dd, Rc::new(false.into())))
                        .push_back(ListItemValue::create(mul_term_di, Rc::new(false.into())))
                        .push_back(ListItemValue::create(mul_term_id, Rc::new(false.into())))
                        .push_back(ListItemValue::create(mul_term_ii, Rc::new(false.into())))
                        .push_back(ListItemValue::create(rem_term_dd, Rc::new(false.into())))
                        .push_back(ListItemValue::create(rem_term_di, Rc::new(false.into())))
                        .push_back(ListItemValue::create(rem_term_id, Rc::new(false.into())))
                        .push_back(ListItemValue::create(rem_term_ii, Rc::new(false.into())))
                        .push_back(ListItemValue::create(sub_term_dd, Rc::new(false.into())))
                        .push_back(ListItemValue::create(sub_term_di, Rc::new(false.into())))
                        .push_back(ListItemValue::create(sub_term_id, Rc::new(false.into())))
                        .push_back(ListItemValue::create(sub_term_ii, Rc::new(false.into())))
                        .push_back(ListItemValue::create(pow_term_dd, Rc::new(false.into())))
                        .push_back(ListItemValue::create(pow_term_di, Rc::new(false.into())))
                        .push_back(ListItemValue::create(pow_term_id, Rc::new(false.into())))
                        .push_back(ListItemValue::create(pow_term_ii, Rc::new(false.into())))
                        .push_back(ListItemValue::create(abs_term_d, Rc::new(false.into())))
                        .push_back(ListItemValue::create(abs_term_i, Rc::new(false.into())))
                        .push_back(ListItemValue::create(neg_term_d, Rc::new(false.into())))
                        .push_back(ListItemValue::create(neg_term_i, Rc::new(false.into())))
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
                            .push_back(Rc::new(DidValue::Double(1.1 + 2.2)))
                            .push_back(Rc::new(DidValue::Double(1.1 + (2 as f64))))
                            .push_back(Rc::new(DidValue::Double((1 as f64) + 2.2)))
                            .push_back(Rc::new(DidValue::Integer(1 + 2)))

                            .push_back(Rc::new(DidValue::Double(12.6 / 2.0)))
                            .push_back(Rc::new(DidValue::Double(12.6 / (2 as f64))))
                            .push_back(Rc::new(DidValue::Double((12 as f64) / 2.0)))
                            .push_back(Rc::new(DidValue::Integer(12 / 2)))

                            .push_back(Rc::new(DidValue::Double(12.5 * 2.4)))
                            .push_back(Rc::new(DidValue::Double(12.5 * (2 as f64))))
                            .push_back(Rc::new(DidValue::Double((12 as f64) * 2.4)))
                            .push_back(Rc::new(DidValue::Integer(12 * 2)))

                            .push_back(Rc::new(DidValue::Double(12.2 % 5.1)))
                            .push_back(Rc::new(DidValue::Double(12.2 % (5 as f64))))
                            .push_back(Rc::new(DidValue::Double((12 as f64) % 5.1)))
                            .push_back(Rc::new(DidValue::Integer(12 % 5)))

                            .push_back(Rc::new(DidValue::Double(12.8 - 5.2)))
                            .push_back(Rc::new(DidValue::Double(12.8 - (5 as f64))))
                            .push_back(Rc::new(DidValue::Double((12 as f64) - 5.2)))
                            .push_back(Rc::new(DidValue::Integer(12 - 5)))

                            .push_back(Rc::new(DidValue::Double((2.25_f64).powf(3.1_f64))))
                            .push_back(Rc::new(DidValue::Double((2.25_f64).powf(3 as f64))))
                            .push_back(Rc::new(DidValue::Double((2 as f64).powf(3.1_f64))))
                            .push_back(Rc::new(DidValue::Integer((2_i64).pow(3_u32))))

                            .push_back(Rc::new(DidValue::Double(12.3)))
                            .push_back(Rc::new(DidValue::Integer(12)))

                            .push_back(Rc::new(DidValue::Double(-12.1)))
                            .push_back(Rc::new(DidValue::Integer(-12)))
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }
}