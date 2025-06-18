// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = env::current_dir()?;
    path.push("src/parse");

    lalrpop::Configuration::new()
    .generate_in_source_tree()
    .process_dir(path)?;

    Ok(())
}