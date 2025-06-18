// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;
use crate::machine::sorts::sort_function::FunctionValue;

pub struct ApplicationValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> ApplicationValue<'a> {
    pub fn fun(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"fun".to_string()).unwrap()) }
    pub fn arg(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"arg".to_string()).unwrap()) }

    pub fn create(fun: Rc<DidValue>, arg: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("fun".to_string(), fun)
            .insert("arg".to_string(), arg);
        Rc::new(DidValue::Map(Sort::Application, ht))
    }
}

pub fn rewrite_application(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let application = ApplicationValue { value: &context.term };
    let fun_context = context.with_term(Rc::clone(&application.fun()));
    match rewrite_term(machine, fun_context)? {
        RewriteOk::Term(fun_result) => {
            let fun_sort = term_sort(&fun_result);
            if let Sort::Function = fun_sort {
                let arg_context = context.with_term(Rc::clone(&application.arg()));
                match rewrite_term(machine, arg_context)? {
                    RewriteOk::Term(arg_result) => {
                        let function_value = FunctionValue { value: &fun_result };
                        let bindings = 
                            match function_value.bindings() {
                                Some(b) => b,
                                None => DidValue::new_map_constant(),
                            };
                        let binding_context = 
                            RewriteContext {
                                base: Rc::clone(&context.base),
                                term: function_value.pattern(),
                                bindings,
                                policies: Rc::clone(&context.policies),
                            };
                        match term_bind(machine, binding_context, Rc::clone(&arg_result))? {
                            MatchOk::Bindings(app_bindings) => {
                                let app_context = 
                                    RewriteContext {
                                        base: Rc::clone(&context.base),
                                        term: Rc::clone(&function_value.term()),
                                        bindings: app_bindings,
                                        policies: Rc::clone(&context.policies),
                                    };
                                match rewrite_term(machine, app_context)? {
                                    RewriteOk::Blocked(blocked_term) => {
                                        Result::Ok(
                                            RewriteOk::Blocked(
                                                ApplicationValue::create(
                                                    FunctionValue::create(
                                                        function_value.pattern(), 
                                                        blocked_term, 
                                                        function_value.bindings(),
                                                        function_value.free()
                                                    ), 
                                                    arg_result
                                                ).as_hash()
                                            )
                                        )
                                    },
                                    result => Result::Ok(result),
                                }
                            },
                            MatchOk::NoMatch => panic!(),
                        }
                    },
                    RewriteOk::Blocked(arg_blocked) => {
                        Result::Ok(
                            RewriteOk::Blocked(
                                ApplicationValue::create(
                                    fun_result, 
                                    arg_blocked
                                ).as_hash()
                            )
                        )
                    },
                }
            } else {
                Result::Err(RewriteErr::Exception(Rc::new("Not a function".into())))
            }
        },
        RewriteOk::Blocked(fun_blocked) => {
            Result::Ok(
                RewriteOk::Blocked(
                    application.value.with_map_value(
                        "fun".to_string(), 
                        fun_blocked
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
    use sort_application::ApplicationValue;
    use sort_function::FunctionValue;

    #[test]
    fn test_application() {
        // (fun x -> x) 1
        let program = ApplicationValue::create(
            FunctionValue::create(
                LookupValue::create("x"), 
                LookupValue::create("x"), 
                None,
                None
            ), 
            Rc::new(1.into())
        );
        let result = run_rewrite(program);
        let expected = Result::Ok(RewriteOk::Term(Rc::new(1.into())));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_application_blocked_function() {
        // blocked 1
        let program = ApplicationValue::create(
            LookupValue::create("blocked"),
            Rc::new(1.into())
        );
        let result = run_rewrite(program.clone());
        // blocked 1
        let expected = Result::Ok(RewriteOk::Blocked(program));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_application_blocked_arg() {
        // (fun x -> x) blocked
        let program = ApplicationValue::create(
            FunctionValue::create(
                LookupValue::create("x"), 
                LookupValue::create("x"), 
                None,
                None
            ), 
            LookupValue::create("blocked")
        );
        let result = run_rewrite(program.clone());
        let expected = Result::Ok(RewriteOk::Blocked(program));
        assert_eq!(expected, result);
    }

}
