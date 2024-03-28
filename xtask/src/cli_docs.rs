use std::{env, fs};
use std::path::PathBuf;
use std::process::Command;

use crate::cli::{Docs};
use crate::{DynError, project_root};

impl Docs {
    pub(crate) fn run(self) -> Result<(), DynError> {
        docs_clean()?;
        docs_run()?;

        Ok(())
    }
}

fn docs_clean() -> Result<(), DynError> {
    let _ = fs::remove_dir_all(&dist_doc_dir());
    fs::create_dir_all(&dist_doc_dir())?;

    Ok(())
}

fn docs_run() -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["doc", "--all-features"])
        .status()?;

    if !status.success() {
        Err("Command 'cargo doc --all-features' failed")?;
    }

    Ok(())
}

fn dist_doc_dir() -> PathBuf {
    project_root().join("target/doc")
}