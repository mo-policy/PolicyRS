// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::path::Path;
use std::{path::PathBuf, rc::Rc};
use rewrite::machine::docket::{ docket_rewrite, docket_parse};
use rewrite::{data::DidValue, machine::rewrite_machine::RewriteMachine, machine::uniform::policy_std};
use rewrite::machine::resolver::DidResolver;

#[test]
fn test_std_docket() {
    let mut docket_dir = env!("CARGO_MANIFEST_DIR").to_string();
    docket_dir.push_str("/uniform_tests/docket_std");
    let mut docket = docket_parse(Path::new(&docket_dir)).unwrap();
    let std_doc = policy_std();
    docket.insert_mut((PathBuf::new(), std_doc));
    let mut machine = RewriteMachine::new();
    machine.docket = docket;
    docket_rewrite(&mut machine);

    let math_did = "did:policy:test:std:math".parse().unwrap();
    let math_result = machine.docket.resolve(&math_did).unwrap();
    let policy_result = math_result.try_map_value(&"policy".to_string()).unwrap();

    //println!("{policy_result:?}");

    let policy_expected = 
        DidValue::new_map_constant()
        .with_map_value("add_dd".to_string(), Rc::new((1.1 + 2.2).into()))
        .with_map_value("add_di".to_string(), Rc::new((1.1 + (2 as f64)).into()))
        .with_map_value("add_id".to_string(), Rc::new(((1 as f64) + 2.2).into()))
        .with_map_value("add_ii".to_string(), Rc::new((1 + 2).into()))

        .with_map_value("sub_dd".to_string(), Rc::new((1.1 - 2.2).into()))
        .with_map_value("sub_di".to_string(), Rc::new((1.1 - (2 as f64)).into()))
        .with_map_value("sub_id".to_string(), Rc::new(((1 as f64) - 2.2).into()))
        .with_map_value("sub_ii".to_string(), Rc::new((1 - 2).into()))

        .with_map_value("mul_dd".to_string(), Rc::new((1.1 * 2.2).into()))
        .with_map_value("mul_di".to_string(), Rc::new((1.1 * (2 as f64)).into()))
        .with_map_value("mul_id".to_string(), Rc::new(((1 as f64) * 2.2).into()))
        .with_map_value("mul_ii".to_string(), Rc::new((1 * 2).into()))

        .with_map_value("div_dd".to_string(), Rc::new((1.1 / 2.2).into()))
        .with_map_value("div_di".to_string(), Rc::new((1.1 / (2 as f64)).into()))
        .with_map_value("div_id".to_string(), Rc::new(((1 as f64) / 2.2).into()))
        .with_map_value("div_ii".to_string(), Rc::new((1 / 2).into()))

        .with_map_value("rem_dd".to_string(), Rc::new((1.1 % 2.2).into()))
        .with_map_value("rem_di".to_string(), Rc::new((1.1 % (2 as f64)).into()))
        .with_map_value("rem_id".to_string(), Rc::new(((1 as f64) % 2.2).into()))
        .with_map_value("rem_ii".to_string(), Rc::new((1 % 2).into()))

        .with_map_value("complex_1".to_string(), Rc::new((1 + 2 * 3 / 4 % 5).into()));

    assert_eq!(policy_expected, *policy_result);
}


