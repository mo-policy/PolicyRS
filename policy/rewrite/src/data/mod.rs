// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::cmp::Ordering;
use std::{cell::RefCell, rc::Rc};
use std::time::{SystemTime, UNIX_EPOCH};
use std::hash::Hash;
use std::fmt::Debug;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use digest::DynDigest;
use rpds::{ HashTrieMap, HashTrieSet, List, Vector };
use serde::{ser::{SerializeMap, SerializeSeq}, Serialize};
use sha2::Sha512_256;
use sort::Sort;
use uri::DidUri;
use crate::machine::rewrite_machine::{RewriteContext, RewriteMachine, RewriteOk, RewriteErr};

pub mod uri;
pub mod sort;

#[derive(Clone, Eq, PartialEq, PartialOrd, Hash)]
pub enum DidAux {
    Digest { kind: String, data: Vec<u8> },
    Encoding { kind: String, data: String },
    Location { start: (usize, usize), end: (usize, usize) },
}

#[derive(Clone)]
pub enum DidValue { 
    Map(Sort, HashTrieMap<String, Rc<DidValue>>),
    List(Vector<Rc<DidValue>>),
    Set(HashTrieSet<Rc<DidValue>>),
    Tuple(Vector<Rc<DidValue>>),
    DateTime(SystemTime),
    String(String),
    Integer(i64),
    Double(f64),
    Boolean(bool),
    Null,
    Uri(Box<DidUri>),
    Cell(RefCell<Rc<DidValue>>),
    Rewrite(String, Vec<String>, fn (machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr>),
    Aux(List<DidAux>, Rc<DidValue>),
}

impl Debug for DidValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(self).unwrap();
        f.write_str(&json)
    }
}

impl PartialOrd for DidValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Map(lsort, l0), Self::Map(rsort, r0)) => {
                if lsort < rsort { Some(Ordering::Less) } else 
                if lsort > rsort { Some(Ordering::Greater) } else {
                    let mut l0_size = l0.size();
                    let mut r0_size = r0.size();
                    if let Sort::Function = lsort {
                        if let Some(lb) = l0.get("bindings") {
                            if let Some(lbv) = lb.try_map() {
                                if lbv.size() == 0 { 
                                    l0_size -= 1
                                }
                            }
                        }
                        if let Some(lf) = l0.get("free") {
                            if let Some(lfv) = lf.try_list() {
                                if lfv.len() == 0 { 
                                    l0_size -= 1
                                }
                            }
                        }
                        if let Some(rb) = r0.get("bindings") {
                            if let Some(rbv) = rb.try_map() {
                                if rbv.size() == 0 { 
                                    r0_size -= 1
                                }
                            }
                        }
                        if let Some(rf) = r0.get("free") {
                            if let Some(rfv) = rf.try_list() {
                                if rfv.len() == 0 { 
                                    r0_size -= 1
                                }
                            }
                        }
                    }
                    if l0_size < r0_size {
                        Some(Ordering::Less)
                    } else if l0_size > r0_size {
                        Some(Ordering::Greater)
                    } else {
                        for l0_key in l0.keys() {
                            if let Sort::Function = lsort {
                                // None == {}, and {} == None
                                if l0_key == "bindings" {
                                    let l0_bindings = l0.get("bindings");
                                    let r0_bindings = r0.get("bindings");
                                    if l0_bindings.is_none() {
                                        if r0_bindings.is_some() {
                                            let rbv = r0_bindings.unwrap();
                                            let Some(rb) = rbv.try_map() else {
                                                return Some(Ordering::Less)
                                            };
                                            if rb.size() > 0 {
                                                return Some(Ordering::Greater)
                                            }
                                        }
                                        continue;
                                    }
                                    if r0_bindings.is_none() {
                                        return Some(Ordering::Greater)
                                    } else {
                                        let lbv = l0_bindings.unwrap();
                                        let rbv = r0_bindings.unwrap();
                                        let Some(lb) = lbv.try_map() else {
                                            return Some(Ordering::Greater)
                                        };
                                        let Some(rb) = rbv.try_map() else {
                                            return Some(Ordering::Less)
                                        };
                                        if lb.size() == 0 && rb.size() == 0 {
                                            continue;
                                        }
                                    }
                                } else if l0_key == "free" {
                                    let l0_free = l0.get("free");
                                    let r0_free = r0.get("free");
                                    if l0_free.is_none() {
                                        if r0_free.is_some() {
                                            let rfv = r0_free.unwrap();
                                            let Some(rf) = rfv.try_list() else {
                                                return Some(Ordering::Less)
                                            };
                                            if rf.len() > 0 {
                                                return Some(Ordering::Greater)
                                            }
                                        }
                                        continue;
                                    }
                                    if r0_free.is_none() {
                                        return Some(Ordering::Greater)
                                    } else {
                                        let lfv = l0_free.unwrap();
                                        let rfv = r0_free.unwrap();
                                        let Some(lf) = lfv.try_list() else {
                                            return Some(Ordering::Greater)
                                        };
                                        let Some(rf) = rfv.try_list() else {
                                            return Some(Ordering::Less)
                                        };
                                        if lf.len() == 0 && rf.len() == 0 {
                                            continue;
                                        }
                                    }
                                }
                            }
                            if r0.contains_key(l0_key) {
                                let l0_value = l0.get(l0_key).unwrap();
                                let r0_value = r0.get(l0_key).unwrap();
                                match l0_value.partial_cmp(r0_value) {
                                    Some(Ordering::Equal) => continue,
                                    x => return x
                                }
                            } else {
                                return Some(Ordering::Greater)
                            }
                        }
                        Some(Ordering::Equal)
                    }
                }
            },
            (Self::List(l0), Self::List(r0)) => {
                if l0.len() < r0.len() {
                    Some(Ordering::Less)
                } else if l0.len() > r0.len() {
                    Some(Ordering::Greater)
                } else {
                    let mut index: usize = 0;
                    for l0_item in l0 {
                        match l0_item.partial_cmp(r0.get(index).unwrap()) {
                            Some(Ordering::Equal) => {
                                index += 1;
                                continue
                            },
                            x => return x
                        }
                    }
                    Some(Ordering::Equal)
                }
            },
            (Self::Set(l0), Self::Set(r0)) => {
                if l0.size() < r0.size() {
                    Some(Ordering::Less)
                } else if l0.size() > r0.size() {
                    Some(Ordering::Greater)
                } else {
                    for l0_item in l0 {
                        if !r0.contains(l0_item) {
                            return Some(Ordering::Greater)
                        }
                    }
                    Some(Ordering::Equal)
                }
            },
            (Self::Tuple(l0), Self::Tuple(r0)) => l0.partial_cmp(r0),
            (Self::DateTime(l0), Self::DateTime(r0)) => l0.partial_cmp(r0),
            (Self::String(l0), Self::String(r0)) => l0.partial_cmp(r0),
            (Self::Integer(l0), Self::Integer(r0)) => l0.partial_cmp(r0),
            (Self::Double(l0), Self::Double(r0)) => l0.partial_cmp(r0),
            (Self::Boolean(l0), Self::Boolean(r0)) => l0.partial_cmp(r0),
            (Self::Null, Self::Null) => Some(Ordering::Equal),
            (Self::Uri(l0), Self::Uri(r0)) => l0.partial_cmp(r0),
            (Self::Cell(l0), Self::Cell(r0)) => l0.partial_cmp(r0),
            (Self::Rewrite(l0, _, _), Self::Rewrite(r0, _, _)) => l0.partial_cmp(r0),
            (Self::Aux(lh, l0), Self::Aux(rh, r0)) => {
                if lh == rh {
                    Some(Ordering::Equal)
                } else {
                    l0.partial_cmp(r0)
                }
            },
            (Self::Aux(_, l0), r0) => (&**l0).partial_cmp(r0),
            (l0, Self::Aux(_, r0)) => l0.partial_cmp(&**r0),
            _ => None,
        }
    }
}

impl PartialEq for DidValue {
    fn eq(&self, other: &Self) -> bool {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}

impl Eq for DidValue { }

impl Hash for DidValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            DidValue::Aux(_, v) => return v.hash(state),
            _ => {},
        }
        core::mem::discriminant(self).hash(state);
        match self {
            DidValue::Map(sort, v) => { 
                sort.hash(state);
                for (k, kv) in v {
                    if let Sort::Function = sort {
                        if k == "bindings" {
                            if let Some(bv) = kv.try_map() {
                                if bv.size() == 0 {
                                    continue;
                                }
                            }
                        } else if k == "free" {
                            if let Some(fv) = kv.try_list() {
                                if fv.len() == 0 {
                                    continue;
                                }
                            }
                        }
                    }
                    k.hash(state);
                    kv.hash(state);
                }
            }
            DidValue::List(v) => v.hash(state),
            DidValue::Set(v) => {
                for sv in v {
                    sv.hash(state);
                }
            },
            DidValue::Tuple(v) => v.hash(state),
            DidValue::DateTime(v) => v.hash(state),
            DidValue::String(v) => v.hash(state),
            DidValue::Integer(v) => v.hash(state),
            DidValue::Double(v) => {
                let vs = format!("{:.1$}", v, 8);
                vs.hash(state);
            },
            DidValue::Boolean(v) => v.hash(state),
            DidValue::Null => {},
            DidValue::Uri(v) => v.hash(state),
            DidValue::Cell(v) => v.borrow().hash(state),
            DidValue::Rewrite(name, _, _) => name.hash(state),
            DidValue::Aux(_, _) => unreachable!(),
        }
    }
}

impl Serialize for DidValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            match self {
                DidValue::Map(sort, v) => {
                    let mut map = serializer.serialize_map(Some(v.size()))?;
                    if let Sort::Constant = sort { } else {
                        map.serialize_entry(&"$sort", &sort.to_string())?;

                    }
                    let mut keys: Vec<&String> = v.keys().collect();
                    keys.sort();
                    for key in keys {
                        let value = v.get(key).unwrap();
                        map.serialize_entry(key, &**value)?;
                    }
                    map.end()
                },
                DidValue::List(v) => {
                    let mut seq = serializer.serialize_seq(Some(v.len()))?;
                    for lv in v {
                        seq.serialize_element(&**lv)?;
                    }
                    seq.end()
                },
                DidValue::Set(v) => {
                    let mut seq = serializer.serialize_seq(Some(v.size()))?;
                    for sv in v {
                        seq.serialize_element(&**sv)?;
                    }
                    seq.end()
                },
                DidValue::Tuple(v) => {
                    let mut seq = serializer.serialize_seq(Some(v.len()))?;
                    for lv in v {
                        seq.serialize_element(&**lv)?;
                    }
                    seq.end()
                },
                DidValue::DateTime(v) => v.serialize(serializer),
                DidValue::String(v) => serializer.serialize_str(v.as_str()),
                DidValue::Integer(v) => serializer.serialize_i64(*v),
                DidValue::Double(v) => serializer.serialize_f64(*v),
                DidValue::Boolean(v) => serializer.serialize_bool(*v),
                DidValue::Null => serializer.serialize_unit(),
                DidValue::Uri(v) => serializer.serialize_str(v.to_string().as_str()),
                DidValue::Cell(v) => v.borrow().serialize(serializer),
                DidValue::Rewrite(id, _, _) => {
                    // serialize as map
                    let mut map = serializer.serialize_map(Some(2))?;
                    map.serialize_entry(&"sort".to_string(), &"Rewrite".to_string())?;
                    map.serialize_entry(&"id".to_string(), id)?;
                    map.end()
                },
                DidValue::Aux(_, value) => {
                    value.serialize(serializer)
                    // serialize as map
                    // let mut map = serializer.serialize_map(Some(2))?;
                    // let url_encoded: String = URL_SAFE_NO_PAD.encode(hash);
                    // map.serialize_entry(&"hash".to_string(), &url_encoded)?;
                    // map.serialize_entry(&"value".to_string(), &**value)?;
                    // map.end()
                },
            }
    }
}


impl DidValue {
    pub fn new_map_constant() -> Rc<DidValue> {
        Rc::new(DidValue::Map(Sort::Constant, HashTrieMap::new()))
    }
    pub fn as_policy_did(self: &Rc<DidValue>, aka: Option<Rc<DidValue>>) -> Rc<DidValue> {
        let policy_value = self.as_hash();
        let did: DidUri = format!("did:policy:{0}", policy_value.hash_str()).parse().unwrap();
        let doc = 
            DidValue::new_map_constant()
            .with_map_value("id".to_string(), Rc::new(DidValue::Uri(Box::new(did))))
            .with_map_value("policy".to_string(), Rc::clone(&policy_value));
        if let Some(aka_value) = aka {
            doc.with_map_value("alsoKnownAs".to_string(), aka_value)
        } else {
            doc
        }
    }
    pub fn as_hash(self: &Rc<DidValue>) -> Rc<DidValue> {
        if let DidValue::Aux(aux_vec, _) = &**self {
            let digest_aux = 
                aux_vec
                .iter()
                .find(|x| 
                    match x {
                        DidAux::Digest { kind, .. } => {
                            kind == "Sha512_256"
                        },
                        _ => false,
                    }
                );
            if digest_aux.is_some() {
                return Rc::clone(self);
            } else {
                todo!("as_hash, todo compute hash and add to aux.");
            }
        } else {
            let mut hasher = Box::new(Sha512_256::default()) as Box<dyn DynDigest>;
            // Sort:DidType:Value
            match &**self {
                DidValue::Map(sort, map) => {
                    hasher.update(sort.to_string().as_bytes());
                    hasher.update(":Map:{".as_bytes());
                    let mut keys: Vec<&String> = map.keys().collect();
                    keys.sort();
                    for k in keys {
                        hasher.update(k.as_bytes());
                        hasher.update(":".as_bytes());
                        if let Some(vhash) = map.get(k).unwrap().as_hash().try_hash() {
                            hasher.update(&vhash);
                            hasher.update(",".as_bytes());
                        } else {
                            unreachable!()
                        }
                    }
                    hasher.update("}".as_bytes());
                    let hash = hasher.finalize_reset().to_vec();
                    let aux = DidAux::Digest { kind: "Sha512_256".to_string(), data: hash };
                    let aux_list = List::new().push_front(aux);
                    Rc::new(DidValue::Aux(aux_list, Rc::clone(self)))
                },
                DidValue::List(items) => {
                    hasher.update("Constant:List:[".as_bytes());
                    for item in items {
                        if let Some(vhash) = item.as_hash().try_hash() {
                            hasher.update(&vhash);
                            hasher.update(",".as_bytes());
                        } else {
                            unreachable!()
                        }
                    }
                    hasher.update("]".as_bytes());
                    let hash = hasher.finalize_reset().to_vec();
                    let aux = DidAux::Digest { kind: "Sha512_256".to_string(), data: hash };
                    let aux_list = List::new().push_front(aux);
                    Rc::new(DidValue::Aux(aux_list, Rc::clone(self)))
                },
                DidValue::Set(elements) => {
                    hasher.update("Constant:Set:{".as_bytes());
                    for element in elements {
                        if let Some(vhash) = element.as_hash().try_hash() {
                            hasher.update(&vhash);
                            hasher.update(",".as_bytes());
                        } else {
                            unreachable!()
                        }
                    }
                    hasher.update("}".as_bytes());
                    let hash = hasher.finalize_reset().to_vec();
                    let aux = DidAux::Digest { kind: "Sha512_256".to_string(), data: hash };
                    let aux_list = List::new().push_front(aux);
                    Rc::new(DidValue::Aux(aux_list, Rc::clone(self)))
                },
                DidValue::Tuple(items) => {
                    hasher.update("Constant:Tuple:(".as_bytes());
                    for item in items {
                        if let Some(vhash) = item.as_hash().try_hash() {
                            hasher.update(&vhash);
                            hasher.update(",".as_bytes());
                        } else {
                            unreachable!()
                        }
                    }
                    hasher.update(")".as_bytes());
                    let hash = hasher.finalize_reset().to_vec();
                    let aux = DidAux::Digest { kind: "Sha512_256".to_string(), data: hash };
                    let aux_list = List::new().push_front(aux);
                    Rc::new(DidValue::Aux(aux_list, Rc::clone(self)))
                },
                DidValue::DateTime(dt) => {
                    // this needs work
                    hasher.update("Constant:DateTime:".as_bytes());
                    let nanos = dt.duration_since(UNIX_EPOCH).unwrap().as_nanos().to_be_bytes();
                    hasher.update(&nanos);
                    let hash = hasher.finalize_reset().to_vec();
                    let aux = DidAux::Digest { kind: "Sha512_256".to_string(), data: hash };
                    let aux_list = List::new().push_front(aux);
                    Rc::new(DidValue::Aux(aux_list, Rc::clone(self)))
                },
                DidValue::String(s) => {
                    hasher.update("Constant:String:".as_bytes());
                    hasher.update(s.as_bytes());
                    let hash = hasher.finalize_reset().to_vec();
                    let aux = DidAux::Digest { kind: "Sha512_256".to_string(), data: hash };
                    let aux_list = List::new().push_front(aux);
                    Rc::new(DidValue::Aux(aux_list, Rc::clone(self)))
                },
                DidValue::Integer(i) => {
                    hasher.update("Constant:Integer:".as_bytes());
                    let bytes = i.to_be_bytes();
                    hasher.update(&bytes);
                    let hash = hasher.finalize_reset().to_vec();
                    let aux = DidAux::Digest { kind: "Sha512_256".to_string(), data: hash };
                    let aux_list = List::new().push_front(aux);
                    Rc::new(DidValue::Aux(aux_list, Rc::clone(self)))
                },
                DidValue::Double(d) => {
                    hasher.update("Constant:Double:".as_bytes());
                    let bytes = d.to_be_bytes();
                    hasher.update(&bytes);
                    let hash = hasher.finalize_reset().to_vec();
                    let aux = DidAux::Digest { kind: "Sha512_256".to_string(), data: hash };
                    let aux_list = List::new().push_front(aux);
                    Rc::new(DidValue::Aux(aux_list, Rc::clone(self)))
                },
                DidValue::Boolean(b) => {
                    hasher.update("Constant:Boolean:".as_bytes());
                    if *b {
                        hasher.update("true".as_bytes());
                    } else {
                        hasher.update("false".as_bytes());
                    }
                    let hash = hasher.finalize_reset().to_vec();
                    let aux = DidAux::Digest { kind: "Sha512_256".to_string(), data: hash };
                    let aux_list = List::new().push_front(aux);
                    Rc::new(DidValue::Aux(aux_list, Rc::clone(self)))
                },
                DidValue::Null => {
                    hasher.update("Constant:Null".as_bytes());
                    let hash = hasher.finalize_reset().to_vec();
                    let aux = DidAux::Digest { kind: "Sha512_256".to_string(), data: hash };
                    let aux_list = List::new().push_front(aux);
                    Rc::new(DidValue::Aux(aux_list, Rc::clone(self)))
                },
                DidValue::Uri(v) => {
                    hasher.update("Constant:Did:".as_bytes());
                    hasher.update(v.to_string().as_bytes());
                    let hash = hasher.finalize_reset().to_vec();
                    let aux = DidAux::Digest { kind: "Sha512_256".to_string(), data: hash };
                    let aux_list = List::new().push_front(aux);
                    Rc::new(DidValue::Aux(aux_list, Rc::clone(self)))
                },
                DidValue::Cell(ref_cell) => {
                    if let Some(h) = ref_cell.borrow().as_hash().try_hash() {
                        let aux = DidAux::Digest { kind: "Sha512_256".to_string(), data: h.clone() };
                        let aux_list = List::new().push_front(aux);
                        Rc::new(DidValue::Aux(aux_list, Rc::clone(self)))
                    } else {
                        unreachable!()
                    }
                },
                DidValue::Rewrite(name, _, _) => {
                    hasher.update("Constant:Rewrite:".as_bytes());
                    hasher.update(name.as_bytes());
                    let hash = hasher.finalize_reset().to_vec();
                    let aux = DidAux::Digest { kind: "Sha512_256".to_string(), data: hash };
                    let aux_list = List::new().push_front(aux);
                    Rc::new(DidValue::Aux(aux_list, Rc::clone(self)))
            },
                DidValue::Aux(_, _) => unreachable!(),
            }
        }
    }

    pub fn try_document_did(&self) -> Option<&Box<DidUri>> {
        self.try_map_value(&"id".to_string()).unwrap().try_uri()
    }

    pub fn try_map_value(&self, key: &String) -> Option<&Rc<DidValue>> {
        match self {
            DidValue::Map(_, v) => {
                v.get(key)
            },
            DidValue::Aux(_, v) => {
                v.try_map_value(key)
            },
            _ => None,
        }
    }
    pub fn try_map(&self) -> Option<&HashTrieMap<String, Rc<DidValue>>> {
        match self {
            DidValue::Map(_, v) => { Some(v) },
            DidValue::Aux(_, v) => {
                v.try_map()
            },
            _ => None,
        }
    }
    pub fn with_map_value(&self, key: String, value: Rc<DidValue>) -> Rc<DidValue> {
        match self {
            DidValue::Map(sort, v) => { 
                let ht = v.insert(key, value);
                Rc::new(DidValue::Map(*sort, ht))
            },
            DidValue::Aux(_, v) => {
                v.with_map_value(key, value)
            },
            _ => panic!(),
        }
    }
    pub fn try_list(&self) -> Option<&Vector<Rc<DidValue>>> {
        match self {
            DidValue::List(v) => { Some(v) },
            DidValue::Aux(_, v) => {
                v.try_list()
            },
            _ => None,
        }
    }
    pub fn try_set(&self) -> Option<&HashTrieSet<Rc<DidValue>>> {
        match self {
            DidValue::Set(v) => { Some(v) },
            DidValue::Aux(_, v) => {
                v.try_set()
            },
            _ => None,
        }
    }
    pub fn try_tuple(&self) -> Option<&Vector<Rc<DidValue>>> {
        match self {
            DidValue::Tuple(v) => { Some(v) },
            DidValue::Aux(_, v) => {
                v.try_tuple()
            },
            _ => None,
        }
    }
    pub fn try_datetime(&self) -> Option<&SystemTime> {
        match self {
            DidValue::DateTime(v) => { Some(v) },
            DidValue::Aux(_, v) => {
                v.try_datetime()
            },
            _ => None,
        }
    }
    pub fn try_string(&self) -> Option<&String> {
        match self {
            DidValue::String(v) => { Some(v) },
            DidValue::Aux(_, v) => {
                v.try_string()
            },
            _ => None,
        }
    }
    pub fn try_integer(&self) -> Option<i64> {
        match self {
            DidValue::Integer(v) => { Some(*v) },
            DidValue::Aux(_, v) => {
                v.try_integer()
            },
            _ => None,
        }
    }
    pub fn try_double(&self) -> Option<f64> {
        match self {
            DidValue::Double(v) => { Some(*v) },
            DidValue::Aux(_, v) => {
                v.try_double()
            },
            _ => None,
        }
    }
    pub fn try_bool(&self) -> Option<bool> { 
        match self {
            DidValue::Boolean(v) => { Some(*v) },
            DidValue::Aux(_, v) => {
                v.try_bool()
            },
            _ => None,
        }
    }
    pub fn try_null(&self) -> Option<()> { 
        match self {
            DidValue::Null => { Some(()) },
            DidValue::Aux(_, v) => {
                v.try_null()
            },
            _ => None,
        }
    }
    pub fn try_uri(&self) -> Option<&Box<DidUri>> { 
        match self {
            DidValue::Uri(v) => { Some(v) },
            DidValue::Aux(_, v) => {
                v.try_uri()
            },
            _ => None,
        }
    }
    pub fn try_cell(&self) -> Option<&RefCell<Rc<DidValue>>> { 
        match self {
            DidValue::Cell(v) => { Some(v) },
            DidValue::Aux(_, v) => {
                v.try_cell()
            },
            _ => None,
        }
    }
    pub fn try_rewrite(&self) -> Option<(&String, &Vec<String>, &fn (machine: &mut RewriteMachine, context: RewriteContext) -> Result<RewriteOk, RewriteErr>)> { 
        match self {
            DidValue::Rewrite(name, free, rw) => { 
                Some((name, free, rw)) 
            },
            DidValue::Aux(_, v) => {
                v.try_rewrite()
            },
            _ => None,
        }
    }
    pub fn try_hash(&self) -> Option<&Vec<u8>> { 
        match self {
            DidValue::Aux(aux_vec, _) => {
                let digest_aux = 
                    aux_vec
                    .iter()
                    .find(|x| 
                        match x {
                            DidAux::Digest { kind, .. } => {
                                kind == "Sha512_256"
                            },
                            _ => false,
                        }
                    );
                if let Some(DidAux::Digest { data, .. }) = digest_aux {
                    Some(data)
                } else {
                    None
                }
            },
            _ => None,
        }
    }
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn hash_str(&self)-> String {
        if let Some(hash) = self.try_hash() {
            URL_SAFE_NO_PAD.encode(hash)
        } else {
            panic!()
        }
    }

    pub fn try_hash_str(&self)-> Option<String> {
        if let Some(hash) = self.try_hash() {
            Some(URL_SAFE_NO_PAD.encode(hash))
        } else {
            None
        }
    }
}

impl From<SystemTime> for DidValue {
    fn from(value: SystemTime) -> Self {
        DidValue::DateTime(value)
    }
}

impl From<String> for DidValue {
    fn from(value: String) -> Self {
        DidValue::String(value)
    }
}

impl From<&str> for DidValue {
    fn from(value: &str) -> Self {
        DidValue::String(value.to_string())
    }
}

impl From<i64> for DidValue {
    fn from(value: i64) -> Self {
        DidValue::Integer(value)
    }
}

impl From<f64> for DidValue {
    fn from(value: f64) -> Self {
        DidValue::Double(value)
    }
}

impl From<bool> for DidValue {
    fn from(value: bool) -> Self {
        DidValue::Boolean(value)
    }
}
