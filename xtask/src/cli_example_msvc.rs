use std::{fs};
use std::path::PathBuf;

use crate::cli::{Example, TargetType};
use crate::{cargo_build, copy_assets, DynError, project_root};

impl Example {
    pub(crate) fn run_msvc(self) -> Result<(), DynError> {
        let status = match self.release {
            true => cargo_build(TargetType::Msvc, "release", self.name.as_str())?,
            false => cargo_build(TargetType::Msvc, "dev", self.name.as_str())?,
        };

        if !status.success() {
            Err("Command 'cargo build' failed")?;
        }


        let name_str = self.name.as_str();
        let executable = format!("{name_str}.exe");

        let _ = fs::create_dir_all(&docs_msvc_dir(self.release))?;
        let _ = fs::copy(
            dist_msvc_dir(self.release).join(executable.as_str()),
            docs_msvc_dir(self.release).join(executable.as_str()),
        );

        if !self.no_assets {
            copy_assets(docs_msvc_dir(self.release));
        }

        Ok(())
    }
}

pub fn docs_msvc_dir(release: bool) -> PathBuf {
    project_root()
        .join("docs/msvc_examples/")
        .join(match release {
            true => "release",
            false => "debug",
        })
}

fn dist_msvc_dir(release: bool) -> PathBuf {
    project_root()
        .join("target/x86_64-pc-windows-msvc")
        .join(match release {
            true => "release",
            false => "debug",
        })
        .join("examples")
}
