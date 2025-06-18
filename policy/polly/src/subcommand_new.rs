// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::fs::{create_dir, File};
use std::io::Write;
use std::process::Command;

/*

polly new [name]
    like cargo new [name]

        /name
            /comm
            /debug
            /src
                hello.pol
            /.git
                ...
            .gitignore
                "/degug"

// hello.pol
begin
    let (sender, world) = (`did:mo:sender`, `did:policy:world`) in
        send (sender, "hello") on world;
        receive on sender with
        | msg -> msg
    end
end


// world.pol
begin
    let world = `did:policy:world` in 
        receive on world with 
        | (sender, msg) -> 
            begin
                send 42 on sender;
                (sender, msg)
            end
    end
end


*/

pub(crate) fn run(name: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating docket `{name}`");

    // create [name] dir as child of current dir
    create_dir(name)?;

    // create [name]/comm dir
    let comm = format!("{name}/comm");
    create_dir(&comm)?;

    // create [name]/src dir
    let src = format!("{name}/src");
    create_dir(&src)?;

    // create [name]/debug dir
    let debug = format!("{name}/debug");
    create_dir(&debug)?;

    // create [name]/.gitignore file
    let gitignore = format!("{name}/.gitignore");
    let mut gitignore_file = File::create(gitignore)?;
    let git_ignore_text = r#"/debug"#;
    writeln!(gitignore_file, "{git_ignore_text}")?;

    // create [name]/src/hello.pol file
    let src_hello = format!("{src}/hello.pol");
    let mut hello_pol_file = File::create(src_hello)?;
    let hello_text = r#"let (sender, world) = (`did:mo:sender`, `did:policy:world`) in
    send (sender, "hello") on world;
    receive on sender with
    | msg -> msg
end
"#;
    writeln!(hello_pol_file, "{hello_text}")?;

    // create [name]/src/world.pol file
    let src_world = format!("{src}/world.pol");
    let mut world_pol_file = File::create(src_world)?;
    let world_text = r#"let world = `did:policy:world` in 
    receive on world with 
    | (sender, msg) -> 
        begin
            send 42 on sender;
            (sender, msg)
        end
end
"#;
        writeln!(world_pol_file, "{world_text}")?;
    
    // initialize git repository
    Command::new(&"git")
    .arg(&"init")
    .current_dir(name)
    .output()?;

    Ok(())
}