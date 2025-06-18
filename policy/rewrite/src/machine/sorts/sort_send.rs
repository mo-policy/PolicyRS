// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::rc::Rc;
use rpds::HashTrieMap;

use crate::machine::rewrite_machine::*;
use crate::data::DidValue;
use crate::data::sort::Sort;

pub struct SendValue<'a> { pub value: &'a Rc<DidValue> }

impl<'a> SendValue<'a> {
    pub fn channel(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"channel".to_string()).unwrap()) }
    pub fn message(&self) -> Rc<DidValue>   { Rc::clone(self.value.try_map_value(&"message".to_string()).unwrap()) }

    pub fn create(channel: Rc<DidValue>, message: Rc<DidValue>) -> Rc<DidValue> {
        let ht: HashTrieMap<String, Rc<DidValue>> = 
            HashTrieMap::new()
            .insert("channel".to_string(), channel)
            .insert("message".to_string(), message);
        Rc::new(DidValue::Map(Sort::Send, ht))
    }
}

pub fn rewrite_send(machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr> {
    let send_value = SendValue { value: &context.term };
    let channel_context = context.with_term(Rc::clone(&send_value.channel()));
    match rewrite_term(machine, channel_context)? {
        RewriteOk::Term(channel_result) => {
            let message_context = context.with_term(Rc::clone(&send_value.message()));
            match rewrite_term(machine, message_context)? {
                RewriteOk::Term(message_result) => {
                    machine.comm.send(channel_result, message_result);
                    Result::Ok(RewriteOk::Term(Rc::new(DidValue::Null).as_hash()))
                },
                RewriteOk::Blocked(message_blocked) => {
                    Result::Ok(
                        RewriteOk::Blocked(
                            SendValue::create(
                                channel_result, 
                                message_blocked
                            ).as_hash()
                        )
                    )
                },
            }
        },
        RewriteOk::Blocked(channel_blocked) => {
            Result::Ok(
                RewriteOk::Blocked(
                    send_value.value.with_map_value(
                        "channel".to_string(), 
                        channel_blocked
                    )
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
    fn test_send() {
        // send 1 on 1
        let program = 
            SendValue::create(
                Rc::new(1.into()),
                Rc::new(1.into())
            );
        let result = run_rewrite(program);
        // null
        let expected = Result::Ok(RewriteOk::Term(Rc::new(DidValue::Null)));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_send_channel_blocked() {
        // send (succ 1) on blocked
        let program = 
            SendValue::create(
                LookupValue::create("blocked"),
                apply_tapl_succ(Rc::new(1.into()))
            );
        let result = run_rewrite(program.clone());
        // send (succ 1) on blocked
        let expected = Result::Ok(RewriteOk::Blocked(program));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_send_message_blocked() {
        // send blocked on (succ 1)
        let program = 
            SendValue::create(
                apply_tapl_succ(Rc::new(1.into())),
                LookupValue::create("blocked")
            );
        // send blocked on 2
        let result = run_rewrite(program);
        let expected = 
            Result::Ok(
                RewriteOk::Blocked(
                    SendValue::create(
                        Rc::new(2.into()),
                        LookupValue::create("blocked")
                    )
                )
            );
        assert_eq!(expected, result);
    }
}
