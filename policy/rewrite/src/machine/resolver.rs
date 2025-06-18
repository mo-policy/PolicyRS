// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::{path::PathBuf, rc::Rc};

use rpds::HashTrieSet;

use crate::data::{uri::DidUri, DidValue, sort::Sort};
use crate::machine::rewrite_machine::term_sort;
pub trait DidResolver {
    fn resolve(&self, did: &DidUri) -> Option<Rc<DidValue>>;
}

impl DidResolver for HashTrieSet<(PathBuf, Rc<DidValue>)> {
    fn resolve(&self, did: &DidUri) -> Option<Rc<DidValue>> {
        let mut resolved_doc: Option<Rc<DidValue>> = None;
        for (_, doc) in self {
            let sort = term_sort(&doc);
            match sort {
                Sort::Constant => {
                    if let Some(id_value) = doc.try_map_value(&"id".to_string()) {
                        if let Some(id_uri) = id_value.try_uri() {
                            if let Some(id_did) = id_uri.get_did() {
                                if id_did == *did {
                                    resolved_doc = Some(Rc::clone(doc));
                                    break;
                                }
                            }
                        }
                    }
                    if let Some(aka_value) = doc.try_map_value(&"alsoKnownAs".to_string()) {
                        if let Some(aka_set) = aka_value.try_set() {
                            for aka_item in aka_set {
                                if let Some(aka_did) = aka_item.try_uri() {
                                    if **aka_did == *did {
                                        resolved_doc = Some(Rc::clone(doc));
                                        break;
                                    }
                                }
                                if let Some(aka_string) = aka_item.try_string() {
                                    if *aka_string == did.to_string() {
                                        resolved_doc = Some(Rc::clone(doc));
                                        break;
                                    }
                                }
                            }
                        }
                    }
                },
                _ => { },
            }
        }
        resolved_doc
    }
}

pub trait DidDereferencer {
    fn dereference(&self, url: &DidUri, base: Option<&Rc<DidValue>>) -> Option<Rc<DidValue>>;
}

impl DidDereferencer for HashTrieSet<(PathBuf, Rc<DidValue>)> {
    
    fn dereference(&self, uri: &DidUri, base: Option<&Rc<DidValue>>) -> Option<Rc<DidValue>> {
        let document = 
            match &uri.get_did() {
                Some(did) => self.resolve(did),
                None => 
                    match base {
                        Some(v) => Some(Rc::clone(v)),
                        None => None,
                    },
            };
        match document {
            Some(doc) => {
                let mut value = Rc::clone(&doc);
                for segment in uri.get_path_segments() {
                    let ps = uri.part_string(segment);
                    let path_text = 
                        if ps.starts_with("/") {
                            ps[1..].to_string()
                        } else {
                            ps
                        };
                    let sort = term_sort(&value);
                    match sort {
                        Sort::Constant => {
                            if let Some(v) = value.try_map_value(&path_text) {
                                value = Rc::clone(v);
                            } else {
                                return None;
                            }
                        },
                        Sort::Map => {
                            // { "items": [ { "key": ..., "value": ... } ] }
                            let items = value.try_map_value(&"items".to_string()).unwrap();
                            for item in items.try_list().unwrap() {
                                let key_value = item.try_map_value(&"key".to_string()).unwrap();
                                let key = key_value.try_string().unwrap();
                                if *key == path_text {
                                    let item_value = item.try_map_value(&"value".to_string()).unwrap();
                                    value = Rc::clone(item_value);
                                    break;
                                }
                            }
                        },
                        _ => {
                            return None;
                        },
                    }
                }
                Some(value)
            }
            None => None
        }
    }
}

