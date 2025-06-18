// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use std::path::PathBuf;

use rpds::{HashTrieMap, HashTrieSet, List};
use serde::{self, ser::SerializeTupleVariant, Serialize};

use crate::data::DidValue;
use crate::data::sort::Sort;
use crate::machine::sorts::*;
use crate::machine::uniform::policy_std;
use crate::machine::comm::DidComm;

use sort_application::*;
use sort_as::*;
use sort_cell::*;
use sort_cell_set::*;
use sort_cell_get::*;
use sort_dereference::*;
use sort_eval::*;
use sort_fix::*;
use sort_function::*;
use sort_if::*;
use sort_let::*;
use sort_list::*;
use sort_lookup::*;
use sort_loop::*;
use sort_map::*;
use sort_match::*;
use sort_policy::*;
use sort_receive::*;
use sort_send::*;
use sort_sequence::*;
use sort_set::*;
use sort_try::*;
use sort_tuple::*;

pub struct RewriteMachine {
    pub comm: DidComm,
    pub docket: HashTrieSet<(PathBuf, Rc<DidValue>)>,
}

impl RewriteMachine {
    pub fn new() -> RewriteMachine {
        RewriteMachine {
            comm: DidComm::new(),
            docket: HashTrieSet::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RewriteContext {
    pub base: Rc<DidValue>,
    pub term: Rc<DidValue>,
    pub bindings: Rc<DidValue>,
    pub policies: Rc<List<RewriteContext>>,
}

impl RewriteContext {
    pub fn new(base: Rc<DidValue>, term: Rc<DidValue>) -> RewriteContext {
        RewriteContext {
            base,
            term,
            bindings: DidValue::new_map_constant(),
            policies: Rc::new(List::new()),
        }
    }
    
    pub fn with_term(&self, term: Rc<DidValue>) -> RewriteContext {
        RewriteContext {
            base: Rc::clone(&self.base),
            term,
            bindings: Rc::clone(&self.bindings),
            policies: Rc::clone(&self.policies),
        }
    }

    pub fn with_bindings(&self, bindings: Rc<DidValue>) -> RewriteContext {
        RewriteContext {
            base: Rc::clone(&self.base),
            term: Rc::clone(&self.term),
            bindings,
            policies: Rc::clone(&self.policies),
        }
    }

    pub fn with_policies(&self, policies: Rc<List<RewriteContext>>) -> RewriteContext {
        RewriteContext {
            base: Rc::clone(&self.base),
            term: Rc::clone(&self.term),
            bindings: Rc::clone(&self.bindings),
            policies,
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum RewriteOk {
    Term(Rc<DidValue>),
    Blocked(Rc<DidValue>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RewriteErr {
    Exception(Rc<DidValue>),
    PolicyViolation(bool, Rc<DidValue>),
}

impl Serialize for RewriteOk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            match self {
                RewriteOk::Term(v) => {
                    let mut ser_tup = 
                        serializer.serialize_tuple_variant(
                            "RewriteOk", 
                            0, 
                            "Term", 
                            1)?;
                    ser_tup.serialize_field(&**v)?;
                    ser_tup.end()
                },
                RewriteOk::Blocked(v) => {
                    let mut ser_tup = 
                        serializer.serialize_tuple_variant(
                            "RewriteResult", 
                            1, 
                            "Blocked", 
                            1)?;
                    ser_tup.serialize_field(&**v)?;
                    ser_tup.end()
                },
            }
        }
}

impl Serialize for RewriteErr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            match self {
                RewriteErr::Exception(v) => {
                    let mut ser_tup = 
                        serializer.serialize_tuple_variant(
                            "RewriteErr", 
                            0, 
                            "Exception", 
                            1)?;
                    ser_tup.serialize_field(&**v)?;
                    ser_tup.end()
                },
                RewriteErr::PolicyViolation(blocked, v) => {
                    let mut ser_tup = 
                        serializer.serialize_tuple_variant(
                            "RewriteErr", 
                            2, 
                            "PolicyViolation", 
                            1)?;
                        ser_tup.serialize_field(&*blocked)?;
                        ser_tup.serialize_field(&**v)?;
                    ser_tup.end()
                },
            }
        }
}


#[derive(Debug)]
pub enum MatchOk {
    Bindings(Rc<DidValue>),
    NoMatch,
}

pub fn term_sort(term: &DidValue) -> Sort {
    match term {
        DidValue::Map(sort, _) => *sort,
        DidValue::Aux(_, v) => term_sort(v),
        _ => Sort::Constant,
    }
}

pub fn rewrite_term(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    // check context.term against any policies
    if context.policies.len() > 0 {
        for policy_context in &*context.policies {
            let policy_value = PolicyValue { value: &policy_context.term };
            let rules_context = policy_context.with_term(Rc::clone(&context.term));
            match rewrite_rules(machine, rules_context, policy_value.rules())? {
                RulesOk::NoRuleMatched => { },
                RulesOk::MatchedRule(rule_context) => {
                    match rewrite_term(machine, rule_context)? {
                        RewriteOk::Term(rule_term) => {
                            return Result::Err(RewriteErr::PolicyViolation(false, rule_term));
                        },
                        RewriteOk::Blocked(rule_blocked) => {
                            return Result::Err(RewriteErr::PolicyViolation(true, rule_blocked));
                        }
                    }
                },
                RulesOk::BlockedGuard(blocked_rules) => {
                    return Result::Ok(
                        RewriteOk::Blocked(
                            PolicyValue::create(
                                context.term,
                                blocked_rules
                            ).as_hash()
                        )
                    )
                },
            }
        }
    }
    let sort = term_sort(&context.term);
    match sort {
        Sort::Application => rewrite_application(machine, context),
        Sort::As => rewrite_as(machine, context),
        Sort::Cell => rewrite_cell(machine, context),
        Sort::CellGet => rewrite_cell_get(machine, context),
        Sort::CellSet => rewrite_cell_set(machine, context),
        Sort::Constant => {
            if let DidValue::Rewrite(_, _, rw) = &*context.term {
                rw(machine, context)
            } else {
                Result::Ok(RewriteOk::Term(context.term.as_hash()))
            }
        }
        Sort::Dereference => rewrite_dereference(machine, context),
        Sort::Eval => rewrite_eval(machine, context),
        Sort::Fix => rewrite_fix(machine, context),
        Sort::Function => rewrite_function(machine, context),
        Sort::If => rewrite_if(machine, context),
        Sort::Let => rewrite_let(machine, context),
        Sort::List => rewrite_list(machine, context),
        Sort::Lookup => rewrite_lookup(machine, context),
        Sort::Loop => rewrite_loop(machine, context),
        Sort::Map => rewrite_map(machine, context),
        Sort::Match => rewrite_match(machine, context),
        Sort::Policy => rewrite_policy(machine, context),
        Sort::Receive => rewrite_receive(machine, context),
        Sort::Send => rewrite_send(machine, context),
        Sort::Sequence => rewrite_sequence(machine, context),
        Sort::Set => rewrite_set(machine, context),
        Sort::Throw => rewrite_throw(machine, context),
        Sort::Try => rewrite_try(machine, context),
        Sort::Tuple => rewrite_tuple(machine, context),
    }
}



pub fn term_bind(machine: &mut RewriteMachine, context: RewriteContext, value: Rc<DidValue>) -> Result<MatchOk, RewriteErr> {
    let sort = term_sort(&context.term);
    match sort {
        Sort::As => bind_as(machine, context, value),
        Sort::Lookup => bind_lookup(machine, context, value),
        Sort::Map => bind_map(machine, context, value),
        Sort::Tuple => bind_tuple(machine, context, value),
        _ => {
            if context.term == value {
                Result::Ok(MatchOk::Bindings(context.bindings))
            } else {
                Result::Ok(MatchOk::NoMatch)
            }
        },
    }
}

pub fn free_names(machine: &mut RewriteMachine, term: &Rc<DidValue>, bound: HashTrieMap<String, Rc<DidValue>>, free: HashTrieMap<String, Rc<DidValue>>) -> HashTrieMap<String, Rc<DidValue>> {
    let sort = term_sort(term);
    match sort {
        Sort::Application => {
            // fun, arg
            let v = ApplicationValue { value: term };
            let fun_free = free_names(machine, &v.fun(), bound.clone(), free);
            let arg_free = free_names(machine, &v.arg(), bound, fun_free);
            arg_free
        }
        Sort::Cell => {
            // value (is a term)
            let v = CellValue { value: term };
            let value_free = free_names(machine, &v.value(), bound, free);
            value_free
        },
        Sort::CellGet => {
            // cell get
            // !y, y is free
            let v = CellGetValue { value: term };
            let cell_get_free = free_names(machine, &v.cell(), bound, free);
            cell_get_free
        },
        Sort::CellSet => {
            // x := y, x is bound, y is free
            let v = CellSetValue { value: term };
            let cell_free = free_names(machine, &v.value(), bound, free);
            cell_free
        },
        Sort::Constant => {
            if let DidValue::Rewrite(_, rw_free, _) = &**term {
                let mut rewrite_free = free.clone();
                for name in rw_free {
                    if !bound.contains_key(name) {
                        rewrite_free.insert_mut(name.clone(), Rc::new(DidValue::Null));
                    }
                }
                rewrite_free
            } else {
                free
            }
        },
        Sort::Fix => {
            // term
            let v = FixValue { value: term };
            let term_free = free_names(machine, &v.term(), bound, free);
            term_free
        },
        Sort::Function => {
            // fun x -> y, x is bound, y is free
            let v = FunctionValue { value: term };
            let pattern_bound = free_names(machine, &v.pattern(), HashTrieMap::new(), bound);
            let term_free = free_names(machine, &v.term(), pattern_bound, free);
            term_free
        },
        Sort::If => {
            // condition, then, else
            let v = IfValue { value: term };
            let condition_free = free_names(machine, &v.condition(), bound.clone(), free);
            let then_free = free_names(machine, &v.then_term(), bound.clone(), condition_free);
            let else_free = free_names(machine, &v.else_term(), bound, then_free);
            else_free
        }
        Sort::Let => {
            // pattern, term, in
            // let x = y in z, x is bound, y and z are free
            let v = LetValue { value: term };
            let term_free = free_names(machine, &v.term(), bound.clone(), free);
            let pattern_bound = free_names(machine, &v.pattern(), HashTrieMap::new(), bound);
            let in_free = free_names(machine, &v.in_term(), pattern_bound, term_free);
            in_free
        },
        Sort::List => {
            // items
            let v = ListValue { value: term };
            let items_value = v.items();
            let items = items_value.try_list().unwrap();
            let mut items_free = free.clone();
            for item in items {
                let liv = ListItemValue { value: item };
                items_free = free_names(machine, &liv.term(), bound.clone(), items_free);
            }
            items_free
        },
        Sort::Lookup => {
            // x, x is free if not bound
            let v = LookupValue { value: term };
            let name_value = v.name();
            let name = name_value.try_string().unwrap();
            if bound.contains_key(name) {
                free
            } else {
                free.insert(name.clone(), Rc::new(DidValue::Null))
            }
        },
        Sort::Loop => {
            // pattern, iterator, term
            // pattern: null
            // iterator: fun null -> { done: true, value: null }
            // term: null
            // pattern are bound, iterator is function like, term is recursive call
            let v = LoopValue { value: term };
            let pattern_bound = free_names(machine, &v.pattern(), HashTrieMap::new(), bound);
            let iterator_free = free_names(machine, &v.iterator(), pattern_bound.clone(), free);
            let term_free = free_names(machine, &v.term(), pattern_bound, iterator_free);
            term_free
        }
        Sort::Map => {
            // bound and free of items
            let v = MapValue{ value: term };
            let items_value = v.items();
            let items = items_value.try_list().unwrap();
            let mut items_free = free.clone();
            for item in items {
                let liv = MapItemValue { value: item };
                items_free = free_names(machine, &liv.value(), bound.clone(), items_free);
            }
            items_free
        },
        Sort::Match => {
            // match x with | y -> z, y is bound, x and z are free
            // term
            // rules
            //      pattern, guard, term
            let v = MatchValue { value: term };
            let term_free = free_names(machine, &v.term(), bound.clone(), free);
            let rules_value = v.rules();
            let rules = rules_value.try_list().unwrap();
            let mut rules_free = term_free;
            for rule in rules {
                let rv = RuleValue { value: rule };
                let pattern_bound = free_names(machine, &rv.pattern(), HashTrieMap::new(), bound.clone());
                let guard_free = free_names(machine, &rv.guard(), pattern_bound.clone(), rules_free);
                rules_free = free_names(machine, &rv.term(), pattern_bound, guard_free);
            }
            rules_free
        },
        Sort::Policy => {
            // policy x with | y -> z, y is bound, x and z are free
            // term
            // rules
            //      pattern, guard, term
            let v = PolicyValue { value: term };
            let term_free = free_names(machine, &v.term(), bound.clone(), free);
            let rules_value = v.rules();
            let rules = rules_value.try_list().unwrap();
            let mut rules_free = term_free;
            for rule in rules {
                let rv = RuleValue { value: rule };
                let pattern_bound = free_names(machine, &rv.pattern(), HashTrieMap::new(), bound.clone());
                let guard_free = free_names(machine, &rv.guard(), pattern_bound.clone(), rules_free);
                rules_free = free_names(machine, &rv.term(), pattern_bound, guard_free);
            }
            rules_free
        },
        Sort::Receive => {
            // receive on x with | y -> z, y is bound, x and z are free
            // channel
            // rules
            //      pattern, guard, term
            let v = ReceiveValue { value: term };
            let channel_free = free_names(machine, &v.channel(), bound.clone(), free);
            let rules_value = v.rules();
            let rules = rules_value.try_list().unwrap();
            let mut rules_free = channel_free;
            for rule in rules {
                let rv = RuleValue { value: rule };
                let pattern_bound = free_names(machine, &rv.pattern(), HashTrieMap::new(), bound.clone());
                let guard_free = free_names(machine, &rv.guard(), pattern_bound.clone(), rules_free);
                rules_free = free_names(machine, &rv.term(), pattern_bound, guard_free);
            }
            rules_free
        },
        Sort::Send => {
            let v = SendValue { value: term };
            let channel_free = free_names(machine, &v.channel(), bound.clone(), free);
            let message_free = free_names(machine, &v.message(), bound, channel_free);
            message_free
        },
        Sort::Sequence => {
            // items
            let v = SequenceValue { value: term };
            let items_value = v.items();
            let items = items_value.try_list().unwrap();
            let mut items_free = free.clone();
            for item in items {
                let liv = ListItemValue { value: item };
                items_free = free_names(machine, &liv.term(), bound.clone(), items_free);
            }
            items_free
        },
        Sort::Throw => {
            // description
            let v = ThrowValue { value: term };
            let description_free = free_names(machine, &v.description(), bound, free);
            description_free
        },
        Sort::Try => {
            // try x with | y -> z, y is bound, x and z are free
            // term
            // rules
            //      pattern, guard, term
            let v = TryValue { value: term };
            let term_free = free_names(machine, &v.term(), bound.clone(), free);
            let rules_value = v.rules();
            let rules = rules_value.try_list().unwrap();
            let mut rules_free = term_free;
            for rule in rules {
                let rv = RuleValue { value: rule };
                let pattern_bound = free_names(machine, &rv.pattern(), HashTrieMap::new(), bound.clone());
                let guard_free = free_names(machine, &rv.guard(), pattern_bound.clone(), rules_free);
                rules_free = free_names(machine, &rv.term(), pattern_bound, guard_free);
            }
            rules_free
        },
        Sort::Tuple => {
            // bound and free of items
            let v = TupleValue { value: term };
            let items_value = v.items();
            let items = items_value.try_list().unwrap();
            let mut items_free = free.clone();
            for item in items {
                let liv = ListItemValue { value: item };
                items_free = free_names(machine, &liv.term(), bound.clone(), items_free);
            }
            items_free
        },
        _ => free,
    }
}

pub fn tapl_succ() -> Rc<DidValue> {
    fn succ_rewrite(_machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
        if let Some(arg_value) = context.bindings.try_map_value(&"x".to_string()) {
            if let Some(x) = arg_value.try_integer() {
                Result::Ok(RewriteOk::Term(Rc::new((x + 1).into())))
            } else {
                panic!("expected integer")
            }
        } else {
            panic!("expected arg binding")
        }
    }
    // fun x -> (x, succ_rewrite)
    FunctionValue::create(
        LookupValue::create("x"),
        Rc::new(DidValue::Rewrite("tapl_succ".into(), vec!["x".to_string()], succ_rewrite)),
        None,
        None
    )
}

pub fn apply_tapl_succ(x: Rc<DidValue>) -> Rc<DidValue> {
    ApplicationValue::create(
        tapl_succ(), 
        x
    )
}

pub fn run_rewrite(program: Rc<DidValue>) -> Result<RewriteOk, RewriteErr> {
    let mut machine = RewriteMachine::new();
    let context = RewriteContext {
        base: Rc::clone(&program),
        term: program,
        bindings: DidValue::new_map_constant(),
        policies: Rc::new(List::new()),
    };
    rewrite_term(&mut machine, context)
}

pub fn run_rewrite_with_std(program: Rc<DidValue>) -> Result<RewriteOk, RewriteErr> {
    let mut machine = RewriteMachine::new();
    let context = RewriteContext {
        base: Rc::clone(&program),
        term: program,
        bindings: DidValue::new_map_constant(),
        policies: Rc::new(List::new()),
    };
    let std_document = policy_std();
    machine.docket.insert_mut((PathBuf::new(), std_document));
    rewrite_term(&mut machine, context)
}


// println!("result: {0}", serde_json::to_string_pretty(&result).unwrap());
// println!("expected: {0}", serde_json::to_string_pretty(&expected).unwrap());
