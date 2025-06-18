// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::fs::{read_to_string, File};
use std::io::Error;
use std::path::Path;
use std::rc::Rc;
use std::{collections::HashMap, time::SystemTime};
use rpds::Vector;

use crate::data::DidValue;
use crate::parse::parse_constant_str;

#[derive(Debug, Clone, PartialEq)]
pub struct DidComm {
    messages: HashMap<Rc<DidValue>, Vec<(u128, Rc<DidValue>)>>,
    reserved: HashMap<u128, Rc<DidValue>>,
    last_id: u128,
}

impl DidComm {
    pub fn new() -> DidComm {
        DidComm {
            messages: HashMap::new(),
            reserved: HashMap::new(),
            last_id: 0_u128,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        /*
        [
            {
                "channel": channel,
                "messages": [
                    {
                        "id": "id",
                        "message": message,
                    }
                ]
            }
        ]
        */
        let mut channel_list: Vector<Rc<DidValue>> = Vector::new();
        for (channel, msgs) in &self.messages {
            let mut msg_list: Vector<Rc<DidValue>> = Vector::new();
            for (id, msg) in msgs {
                let msg_item = DidValue::new_map_constant()
                    .with_map_value("id".to_string(), Rc::new(id.to_string().into()))
                    .with_map_value("message".to_string(), Rc::clone(msg));
                msg_list.push_back_mut(msg_item);
            }
            let channel_item = DidValue::new_map_constant()
                .with_map_value("channel".to_string(), Rc::clone(channel))
                .with_map_value("messages".to_string(), Rc::new(DidValue::List(msg_list)));
            channel_list.push_back_mut(channel_item);
        }
        let comm_value = DidValue::List(channel_list);
        let comm_file = File::create(path)?;
        serde_json::to_writer_pretty(comm_file, &comm_value)?;
        Ok(())
    }

    pub fn restore_from_path(path: &Path) -> Result<DidComm, Error> {
        let comm_text = read_to_string(&path)?;
        DidComm::restore_from_string(&comm_text)
    }

    pub fn restore_from_string(text: &String) -> Result<DidComm, Error> {
        let comm_value = parse_constant_str(&text);
        let mut messages: HashMap<Rc<DidValue>, Vec<(u128, Rc<DidValue>)>> = HashMap::new();
        let mut last_id: u128 = 0_u128;
        let channel_list = comm_value.try_list().unwrap();
        for channel_item in channel_list {
            let channel = channel_item.try_map_value(&"channel".to_string()).unwrap();
            let messages_value = channel_item.try_map_value(&"messages".to_string()).unwrap();
            let messages_list = messages_value.try_list().unwrap();
            let mut message_vec: Vec<(u128, Rc<DidValue>)> = Vec::new();
            for message_item in messages_list {
                let id_value = message_item.try_map_value(&"id".to_string()).unwrap();
                let id_string = id_value.try_string().unwrap();
                last_id = id_string.parse().unwrap();
                let message = message_item.try_map_value(&"message".to_string()).unwrap();
                message_vec.push((last_id, Rc::clone(message)));
            }
            messages.insert(Rc::clone(channel), message_vec);
        }
        Ok(
            DidComm {
                messages,
                reserved: HashMap::new(),
                last_id,
            }
        )
    }

    fn next_id(self: &mut Self) {
        let d = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let n = d.as_nanos();
        if n > self.last_id {
            self.last_id = n;
        } else {
            self.last_id += 1;
        }
    }

    pub fn send(self: &mut Self, channel: Rc<DidValue>, message: Rc<DidValue>) -> String {
        self.next_id();
        //println!("send channel: {channel:?}");
        match self.messages.get_mut(&channel) {
            Some(v) => {
                v.push((self.last_id, Rc::clone(&message)));
            },
            None => {
                self.messages.insert(Rc::clone(&channel), vec![(self.last_id, Rc::clone(&message))]);
            },
        }
        self.last_id.to_string()
    }

    pub fn reserve(self: &mut Self, channel: Rc<DidValue>, id: &Option<String>) -> Option<(String, Rc<DidValue>)> {
        let idv: u128 = 
            match id {
                Some(ids) =>  u128::from_str_radix(ids.as_str(), 10).unwrap(),
                None => 0_u128,
            };
        //println!("reserve channel: {channel:?}");
        match self.messages.get(&channel) {
            Some(v) => {
                for (m_id, t) in v {
                    if *m_id > idv {
                        match self.reserved.insert(*m_id, Rc::clone(&channel)) {
                            None => {
                                return Some((m_id.to_string(), Rc::clone(t)));
                            },
                            _ => {},
                        }
                    }
                };
                None
            },
            None => None,
        }
    }

    pub fn release(self: &mut Self, id: &Option<String>) -> bool {
        match id {
            Some(s) => {
                let idv: u128 = u128::from_str_radix(s.as_str(), 10).unwrap();
                self.reserved.remove(&idv).is_some()
            }
            None => false,
        }
    }

    pub fn receive(self: &mut Self, id: &Option<String>) -> bool {
        match id {
            Some(s) => {
                let idv: u128 = u128::from_str_radix(s.as_str(), 10).unwrap();
                match self.reserved.remove(&idv) {
                    Some(channel) => {
                        match self.messages.get_mut(&channel) {
                            Some(v) => {
                                let mut index: usize = 0;
                                for (m_id, ..) in v.iter() {
                                    if *m_id == idv {
                                        if v.len() > 1 {
                                            v.remove(index);
                                            return true;
                                        } else {
                                            if self.messages.remove(&channel).is_some() {
                                                return true;
                                            } else {
                                                panic!("expected channel");
                                            }
                                        }
                                    }
                                    index += 1;
                                };
                                panic!("message expected")
                            }
                            None => panic!("channel expected")
                        }
                    },
                    None => { false }
                }
            }
            None => false,
        }
    }
}


#[cfg(test)]
mod tests {
    use std::{env::temp_dir, fs::remove_file};
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_comm_send_did() {
        let mut mc = DidComm::new();
        let channel = Rc::new(1.into());
        let message = Rc::new(1.into());
        let id1 = mc.send(Rc::clone(&channel), Rc::clone(&message));
        let id2 = mc.send(channel, message);
        assert_eq!(mc.messages.len(), 1);
        assert!(id1 < id2);
    }

    #[test]
    fn test_comm_reserve_did() {
        let mut mc = DidComm::new();
        let channel = Rc::new(1.into());
        let message = Rc::new(1.into());
        let id = mc.send(channel, message);
        match mc.reserve(Rc::new(1.into()), &None) {
            Some((r_id, r_msg)) => {
                assert_eq!(r_id, id);
                assert_eq!(r_msg, Rc::new(1.into()));
            }
            None => assert!(false, "expected some")
        }
    }

    #[test]
    fn test_comm_release_did() {
        let mut mc = DidComm::new();
        let id = mc.send(Rc::new(1.into()), Rc::new(1.into()));
        mc.send(Rc::new(1.into()), Rc::new(2.into()));
        mc.send(Rc::new(2.into()), Rc::new(2.into()));

        match mc.reserve(Rc::new(1.into()), &None) {
            Some((r_id, r_msg)) => {
                assert_eq!(r_id, id);
                assert_eq!(r_msg, Rc::new(1.into()));
                assert!(mc.release(&Some(r_id)));
                assert_eq!(mc.messages.len(), 2)
            }
            None => assert!(false, "expected some")
        }
    }

    #[test]
    fn test_comm_receive_did() {
        let mut mc = DidComm::new();
        mc.send(Rc::new(1.into()), Rc::new(1.into()));
        mc.send(Rc::new(1.into()), Rc::new(2.into()));
        let id = mc.send(Rc::new(2.into()), Rc::new(2.into()));

        match mc.reserve(Rc::new(2.into()), &None) {
            Some((r_id, r_msg)) => {
                assert_eq!(r_id, id);
                assert_eq!(r_msg, Rc::new(2.into()));
                assert!(mc.receive(&Some(r_id)));
                assert_eq!(mc.messages.len(), 1);
            }
            None => assert!(false, "expected some")
        }
    }

    #[test]
    fn test_comm_save_restore() {
        let mut file_path = temp_dir();
        let file_name = format!("{}.txt", Uuid::new_v4());
        file_path.push(file_name);
        
        let mut mc = DidComm::new();
        let channel = Rc::new(1.into());
        let message = Rc::new(1.into());
        let _id1 = mc.send(Rc::clone(&channel), Rc::clone(&message));
        let _id2 = mc.send(channel, message);
    
        mc.save(&file_path).unwrap();
        let restored = DidComm::restore_from_path(&file_path).unwrap();
        assert_eq!(mc, restored);

        remove_file(file_path).unwrap();
    }

/*
    #[test]
    fn test_comm_debug() {
        let path = Path::new("path to comm.pol");
        //let comm_text = read_to_string(&path).unwrap();
        //let comm_value = parse_constant_str(&comm_text);
        //println!("{comm_value:?}");

        let comm = DidComm::restore_from_path(&path).unwrap();
        println!("DidComm: {comm:?}");

        //let xx = comm.reserve(Rc::new(2.into()), &None);
        //println!("reserve: {xx:?}");

        let src= Path::new("path to /src dir");
        let docket = docket_parse(src).unwrap();
        let mut machine = RewriteMachine { comm, docket };
        docket_rewrite(&mut machine);
    }
*/
}