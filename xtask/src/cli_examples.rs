use std::fs;
use std::path::PathBuf;

use crate::cli::{Examples, TargetType};
use crate::DynError;

impl Examples {
    pub(crate) fn run(self) -> Result<(), DynError> {
        match self.target {
            TargetType::Msvc => self.run_msvc()?,
            TargetType::Web => self.run_web()?,
        }

        Ok(())
    }

    pub(crate) fn list_files(self, path: &str) -> Result<impl Iterator<Item = PathBuf>, DynError> {
        let entries = fs::read_dir(path)?;
        let dir_entries: Vec<_> = entries
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let metadata = entry.metadata().ok()?;
                let is_file = metadata.is_file();
                let is_hidden = entry
                    .file_name()
                    .to_str()
                    .map(|name| name.starts_with('.'))
                    .unwrap_or_default();
                let is_valid = is_file && !is_hidden;
                if is_valid {
                    Some(entry)
                } else {
                    None
                }
            })
            .map(|entry| entry.path())
            .collect();

        Ok(dir_entries.into_iter())
    }
}
