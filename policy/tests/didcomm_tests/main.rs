// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::path::{Path, PathBuf};
use rewrite::data::uri::DidUri;
use rewrite::machine::resolver::DidResolver;
use rewrite::machine::{rewrite_machine::RewriteMachine, uniform::policy_std};
use rewrite::machine::docket::{ docket_rewrite, docket_parse };
 
// https://github.com/hyperledger/aries-rfcs/blob/main/features/0509-action-menu/README.md
// https://didcomm.org/search/?page=1


/*
[ x ] create standard_map_index, to get item from map
[ x ] write map match pattern
[ x ] create 'as' pattern
*/

#[test]
fn test_didcomm_ping_docket() {
    let mut docket_dir = env!("CARGO_MANIFEST_DIR").to_string();
    docket_dir.push_str("/didcomm_tests/docket_ping");
    let mut docket = docket_parse(Path::new(&docket_dir)).unwrap();
    let std_doc = policy_std();
    docket.insert_mut((PathBuf::new(), std_doc));

    let mut machine = RewriteMachine::new();
    machine.docket = docket;
    docket_rewrite(&mut machine);
    let send_did = "did:policy:didcomm:send".parse().unwrap();
    let send_result = machine.docket.resolve(&send_did).unwrap();
    let send_result_did = send_result.try_document_did().unwrap();
    let receive_did = "did:policy:didcomm:receive".parse().unwrap();
    let receive_result = machine.docket.resolve(&receive_did).unwrap();
    let receive_result_did = receive_result.try_document_did().unwrap();

    let send_expected_did: DidUri = "did:policy:jr6a2n1oBuuwL093ZCFleu2ntodz2wDeLvAQhpdya-o".parse().unwrap();
    let receive_expected_did: DidUri = "did:policy:udZ3uBKXuVuX_ML4aH8xrcRobLRuxZoD6H9awb-DnG4".parse().unwrap();

    assert_eq!(send_expected_did, **send_result_did);
    assert_eq!(receive_expected_did, **receive_result_did);
}

