use crate::cli::{Example, Examples};
use crate::{copy_assets, DynError, project_root};
use crate::cli_example_msvc::docs_msvc_dir;

impl Examples {
    pub(crate) fn run_msvc(self) -> Result<(), DynError> {
        copy_assets(docs_msvc_dir(self.release).join("assets"));

        let examples_path = project_root().join("examples").to_string_lossy().into_owned();

        let target = self.target;
        let release = self.release;
        let gzip = self.gzip;

        self.list_files(examples_path.as_str())?
            .for_each(|example| {
                let name = example.file_stem().unwrap().to_str().unwrap().to_owned();
                // eprintln!("{}", name);
                let example = Example {
                    name,
                    target,
                    release,
                    no_assets: true,
                    gzip,
                };

                let _ = example.run();
            });

        Ok(())
    }
}
