// Copyright (c) Mobile Ownership, mobileownership.org.  All Rights Reserved.  See LICENSE.md in the project root for license information.

use std::{env::current_dir, path::PathBuf};

use rewrite::machine::docket::{docket_load, docket_rewrite, docket_save};
use rewrite::machine::uniform::policy_std;

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let docket_dir = current_dir()?;
    println!("Running docket: {docket_dir:?}");

    let mut machine = docket_load(&docket_dir.as_path())?;
    let std_doc = policy_std();
    machine.docket.insert_mut((PathBuf::new(), std_doc));
    docket_rewrite(&mut machine);

    docket_save(&mut machine, &docket_dir);

    Ok(())
}

/*

#[cfg(test)]
mod tests {
    use std::{env::{current_dir, set_current_dir}, path::Path};

    use super::run;

    // this "test" is used to run a folder in the debugger, it isn't really a test.
    #[test]
    fn test_polly_run() {
        let original_dir = current_dir().unwrap();

        let docket_path = Path::new("/home/glenpi/projects/hello");
        set_current_dir(&docket_path).unwrap();
        run().unwrap();

        set_current_dir(&original_dir).unwrap();
    }

}

*/