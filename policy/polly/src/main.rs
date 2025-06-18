// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::fmt::Debug;

use clap::{Parser, Subcommand};

mod subcommand_new;
mod subcommand_run;

#[derive(Parser, Debug)]
#[command(name = "Polly", version = "1.0", about = "Policy's docket manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new Policy docket
    New {
        /// Name of the policy docket
        name: String,
    },
    /// Run a Policy docket
    Run,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            subcommand_new::run(name)
        }
        Commands::Run => {
            subcommand_run::run()
        }
    }
}
