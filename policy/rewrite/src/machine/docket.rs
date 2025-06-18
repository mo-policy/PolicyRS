// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::fs::{read_dir, read_to_string};
use std::{fs::File, rc::Rc};
use std::path::{Path, PathBuf};
use std::io::{Error, Write};
use rpds::HashTrieSet;
use crate::machine::comm::DidComm;
use crate::parse::export::export_term;
use crate::parse::parse_str;
use crate::data::DidValue;
use crate::machine::rewrite_machine::{rewrite_term, RewriteContext, RewriteMachine, RewriteOk};

pub fn docket_rewrite(machine: &mut RewriteMachine) {
    let mut done;
    loop {
        done = true;
        let mut next_docket: HashTrieSet<(PathBuf, Rc<DidValue>)> = machine.docket.clone();
        let items: Vec<(PathBuf, Rc<DidValue>)> = machine.docket.iter().map(|x| x.clone()).collect();
        for (path, doc) in items {
            let hash_term = Rc::clone(&doc).as_hash();
            let hash_before = hash_term.try_hash().unwrap();
            let context = RewriteContext::new(Rc::clone(&hash_term), Rc::clone(&hash_term));
            match rewrite_term(machine, context) {
                Ok(RewriteOk::Term(result)) |
                Ok(RewriteOk::Blocked(result)) => {
                    let hash_result = result.try_hash().unwrap();
                    if hash_before != hash_result {
                        done = false;
                        next_docket.remove_mut(&(path.clone(), doc));
                        next_docket.insert_mut((path, result));
                    }
                },
                Err(x) => {
                    println!("err: {x:?}");
                    panic!()
                },
            }
        }
        if done { break }
        machine.docket = next_docket;
    }
}

pub fn docket_save(machine: &mut RewriteMachine, dir: &Path) {
    // save docket files
    for (path, doc) in &machine.docket {
        if *path != PathBuf::new() {
            println!("Save to: {path:?}");

            //let doc_js = serde_json::to_string_pretty(&**doc).unwrap();
            let mut doc_file = File::create(path).unwrap();
            let mut doc_pol = String::new();
            export_term(&mut doc_pol, doc).unwrap();
            write!(doc_file, "{doc_pol}").unwrap()
        }
    }
    // save comm if not empty
    let comm_path = dir.join("comm").join("comm.pol");
    machine.comm.save(&comm_path).unwrap();
}

pub fn docket_parse(src: &Path) -> Result<HashTrieSet<(PathBuf, Rc<DidValue>)>, Error> {
    let policy_files = read_dir(src)?;
    let mut docket: HashTrieSet<(PathBuf, Rc<DidValue>)> = HashTrieSet::new();
    for policy_file in policy_files {
        let entry = policy_file?;
        let path = entry.path();
        let text = read_to_string(&path)?;
        let program = parse_str(text.as_str());
        docket.insert_mut((path, program.as_hash()));
    }
    Ok(docket)
}

pub fn docket_load(dir: &Path) -> Result<RewriteMachine, Error> {
    let src_path = dir.join("src");
    println!("Loading docket from {src_path:?}");
    let docket = docket_parse(&src_path)?;

    let comm_path = dir.join("comm").join("comm.pol");
    let comm = if comm_path.exists() {
        println!("Restoring comm from {comm_path:?}");
        DidComm::restore_from_path(&comm_path)?
    } else {
        println!("Creating new comm");
        DidComm::new()
    };
    Ok(RewriteMachine { comm, docket })
}


#[cfg(test)]
mod tests {
    use std::{env::temp_dir, fs::{create_dir, remove_dir_all}};

    use uuid::Uuid;

    use crate::data::uri::DidUri;
    use crate::machine::resolver::DidResolver;
    use super::*;

    #[test]
    fn test_docket_one() {
        let mut machine = RewriteMachine::new();
        let aka_did: DidUri = "did:policy:test_docket_one".parse().unwrap();
        let aka_value = DidValue::Set(HashTrieSet::new().insert(Rc::new(DidValue::Uri(Box::new(aka_did.clone())))));
        let doc = Rc::new(DidValue::Integer(1)).as_policy_did(Some(Rc::new(aka_value)));
        machine.docket.insert_mut(("test_docket_one".into(), Rc::clone(&doc)));
        docket_rewrite(&mut machine);
        let result_doc = machine.docket.resolve(&aka_did).unwrap();
        assert_eq!(doc, result_doc)
    }

    #[test]
    fn test_docket_two() {
        let mut machine = RewriteMachine::new();
        let doc1_did: DidUri = "did:policy:test_docket_two:1".parse().unwrap();
        let doc1_aka = DidValue::Set(HashTrieSet::new().insert(Rc::new(DidValue::Uri(Box::new(doc1_did.clone())))));
        let doc1 = Rc::new(DidValue::Integer(1)).as_policy_did(Some(Rc::new(doc1_aka)));
        let doc2_did: DidUri = "did:policy:test_docket_two:2".parse().unwrap();
        let doc2_aka = DidValue::Set(HashTrieSet::new().insert(Rc::new(DidValue::Uri(Box::new(doc2_did.clone())))));
        let doc2 = Rc::new(DidValue::Integer(2)).as_policy_did(Some(Rc::new(doc2_aka)));
        machine.docket.insert_mut((PathBuf::new(), Rc::clone(&doc1)));
        machine.docket.insert_mut((PathBuf::new(), Rc::clone(&doc2)));
        docket_rewrite(&mut machine);
        let result_doc1 = machine.docket.resolve(&doc1_did).unwrap();
        let result_doc2 = machine.docket.resolve(&doc2_did).unwrap();
        assert_eq!(doc1, result_doc1);
        assert_eq!(doc2, result_doc2);
    }

    #[test]
    fn test_docket_load() {
        let temp_path = temp_dir();
        let docket_path = temp_path.join(Uuid::new_v4().to_string());
        create_dir(&docket_path).unwrap();

        // create comm
        let comm_dir_path = &docket_path.join("comm");
        create_dir(&comm_dir_path).unwrap();
        let comm_path = comm_dir_path.join("comm.pol");
        let mut mc = DidComm::new();
        let channel = Rc::new(1.into());
        let message = Rc::new(1.into());
        let _id1 = mc.send(Rc::clone(&channel), Rc::clone(&message));
        let _id2 = mc.send(channel, message);
        mc.save(&comm_path).unwrap();

        // create one src file
        let src_dir_path = &docket_path.join("src");
        create_dir(&src_dir_path).unwrap();
        let src_path = src_dir_path.join("test.pol");
        let mut src_file = File::create(src_path).unwrap();
        write!(src_file, "{}", "1 + 1").unwrap();

        let machine = docket_load(&docket_path).unwrap();
        assert_eq!(machine.comm, mc);
        assert_eq!(machine.docket.size(), 1);

        remove_dir_all(docket_path).unwrap();
    }

}