// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

use super::sort_fix::FixValue;
use super::sort_function::FunctionValue;
pub struct LetValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> LetValue<'a> {
    pub fn recursive(&self) -> Rc<DidValue> { Rc::clone(self.value.try_map_value(&"recursive".to_string()).unwrap()) }
    pub fn pattern(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"pattern".to_string()).unwrap()) }
    pub fn term(&self) -> Rc<DidValue>      { Rc::clone(self.value.try_map_value(&"term".to_string()).unwrap()) }
    pub fn in_term(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"in".to_string()).unwrap()) }

    pub fn create(recursive: bool, pattern: Rc<DidValue>, term: Rc<DidValue>, in_term: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("recursive".to_string(), Rc::new(recursive.into()))
            .insert("pattern".to_string(), pattern)
            .insert("term".to_string(), term)
            .insert("in".to_string(), in_term);
        Rc::new(DidValue::Map(Sort::Let, ht))
    }
}

pub fn rewrite_let(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let mut recursive = false;
    let liv = LetValue { value: &context.term };
    let let_in_value = 
        if let Some(rv) = liv.recursive().try_bool() {
            if rv {
                // let rec x = t1 in t2 end 
                // let x = fix( fun x -> t1 ) in t2
                recursive = true;
                LetValue {
                    value: &LetValue::create(
                        false, 
                        Rc::clone(&liv.pattern()), 
                        FixValue::create(
                            FunctionValue::create(
                                Rc::clone(&liv.pattern()), 
                                Rc::clone(&liv.term()), 
                                None,
                                None
                            )
                        ), 
                        liv.in_term()
                    )
                }
            } else {
                LetValue { value: &context.term }
            }
        } else {
            return Result::Err(RewriteErr::Exception(Rc::new("Let.recursive not Boolean".into())));
        };
    let term_context = context.with_term(Rc::clone(&let_in_value.term()));
    match rewrite_term(machine, term_context)? {
        RewriteOk::Term(term_result) => {
            let pattern_context = context.with_term(Rc::clone(&let_in_value.pattern()));
            match term_bind(machine, pattern_context, Rc::clone(&term_result))? {
                MatchOk::Bindings(let_bindings) => {
                    let in_term_context = 
                        RewriteContext {
                            base: Rc::clone(&context.base),
                            term: Rc::clone(&let_in_value.in_term()),
                            bindings: let_bindings,
                            policies: Rc::clone(&context.policies),
                        };
                    match rewrite_term(machine, in_term_context)? {
                        RewriteOk::Blocked(in_term_blocked) => {
                            Result::Ok(
                                RewriteOk::Blocked(
                                    LetValue::create(
                                        recursive, 
                                        liv.pattern(), 
                                        term_result,
                                        in_term_blocked
                                    ).as_hash()
                                )
                            )
                        },
                        result => Result::Ok(result),
                    }
                },
                MatchOk::NoMatch => { panic!() },
            }
        },
        RewriteOk::Blocked(term_blocked) => {
            Result::Ok(
                RewriteOk::Blocked(
                    let_in_value.value.with_map_value(
                        "term".to_string(), 
                        term_blocked
                    ).as_hash()
                )
            )
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::machine::sorts::sort_list::ListItemValue;
    use crate::machine::sorts::sort_tuple::TupleValue;

    use super::*;
    use super::super::*;
    use rpds::Vector;
    use sort_application::ApplicationValue;
    use sort_lookup::LookupValue;
    use sort_if::IfValue;

    #[test]
    fn test_let_in() {
        // let x = 1 in x end
        let program = LetValue::create(
            false.into(), 
            LookupValue::create("x"), 
            Rc::new(1.into()), 
            LookupValue::create("x")
        );
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(1.into())));
        assert_eq!(expected, result)
    }

    #[test]
    fn test_let_in_blocked_term() {
        // let x = blocked in x end
        let program = LetValue::create(
            false.into(), 
            LookupValue::create("x"), 
            LookupValue::create("blocked"), 
            LookupValue::create("x")
        );
        let result = run_rewrite(program.clone());
        let expected = Result::Ok(RewriteOk::Blocked(program));
        assert_eq!(expected, result)
    }

    #[test]
    fn test_let_in_blocked_in_term() {
        // let x = succ 1 in blocked end
        let program = LetValue::create(
            false.into(), 
            LookupValue::create("x"), 
            apply_tapl_succ(Rc::new(1.into())), 
            LookupValue::create("blocked")
        );
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    LetValue::create(
                        false.into(), 
                        LookupValue::create("x"), 
                        Rc::new(2.into()), 
                        LookupValue::create("blocked")
                    )
                )
            );
        assert_eq!(expected, result);
    }
 
    #[test]
    fn test_let_rec_in() {
        //  let rec f = 
        //      fun x -> 
        //          if x then 
        //              f false 
        //          else 
        //              1 
        //  in 
        //      f true 
        //  end
        let program = 
            LetValue::create(
                true,
                LookupValue::create("f"),
                FunctionValue::create(
                    LookupValue::create("x"), 
                    IfValue::create(
                        LookupValue::create("x"), 
                        ApplicationValue::create(
                            LookupValue::create("f"), 
                            Rc::new(false.into())
                        ),
                        Rc::new(1.into())),
                    None,
                    None
                ),
                ApplicationValue::create(
                    LookupValue::create("f"), 
                    Rc::new(true.into())
                )
            );
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(1.into())));

        println!("result: {0}", serde_json::to_string_pretty(&result).unwrap());
        println!("expected: {0}", serde_json::to_string_pretty(&expected).unwrap());

        assert_eq!(expected, result);
    }

    #[test]
    fn test_let_rec_in_blocked() {
        //  let rec f = 
        //      fun x -> blocked 
        //  in 
        //      f true 
        //  end
        let program = 
            LetValue::create(
                true,
                LookupValue::create("f"),
                FunctionValue::create(
                    LookupValue::create("x"), 
                    LookupValue::create("blocked"),
                    None,
                    None
                ),
                ApplicationValue::create(
                    LookupValue::create("f"), 
                    Rc::new(true.into())
                )
            );
        //  let rec f = 
        //      fun x -> blocked 
        //  in 
        //      (fun x -> blocked) true
        //  end
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    LetValue::create(
                        true,
                        LookupValue::create("f"),
                        FunctionValue::create(
                            LookupValue::create("x"), 
                            LookupValue::create("blocked"),
                            None,
                            Some(Rc::new(DidValue::List(Vector::new().push_back(Rc::new("blocked".into())))))
                        ),
                        ApplicationValue::create(
                            FunctionValue::create(
                                LookupValue::create("x"), 
                                LookupValue::create("blocked"), 
                                None,
                                Some(Rc::new(DidValue::List(Vector::new().push_back(Rc::new("blocked".into())))))
                            ), 
                            Rc::new(true.into())
                        )
                    )
                )
            );
        assert_eq!(expected, result);
    }

    #[test]
    fn let_tuple() {
        // let (sender, world) = (`did:mo:sender`, `did:policy:world`) in sender
        let program = 
            LetValue::create(
                false, 
                TupleValue::create(
                    Rc::new(
                        DidValue::Tuple(
                            Vector::new()
                            .push_back(ListItemValue::create(LookupValue::create("sender"), Rc::new(false.into())))
                            .push_back(ListItemValue::create(LookupValue::create("world"), Rc::new(false.into())))
                        )
                    )
                ), 
                TupleValue::create(
                    Rc::new(
                        DidValue::Tuple(
                            Vector::new()
                            .push_back(ListItemValue::create(Rc::new(DidValue::Uri(Box::new("did:mo:sender".parse().unwrap()))), Rc::new(false.into())))
                            .push_back(ListItemValue::create(Rc::new(DidValue::Uri(Box::new("did:policy:world".parse().unwrap()))), Rc::new(false.into())))
                        )
                    )
                ), 
                LookupValue::create("sender")
            );
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(DidValue::Uri(Box::new("did:mo:sender".parse().unwrap())))));
        assert_eq!(expected, result)
       }
}