use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use crate::cli::{Example, TargetType};
use crate::{cargo_build, copy_assets, gz_file, project_root, wasm_bindgen, wasm_opt, DynError};

impl Example {
    pub(crate) fn run_web(self) -> Result<(), DynError> {
        let status = match self.release {
            true => cargo_build(TargetType::Web, "release", self.name.as_str())?,
            false => cargo_build(TargetType::Web, "dev", self.name.as_str())?,
        };

        if !status.success() {
            Err("Command 'cargo build' failed")?;
        }

        let name_str = self.name.as_str();
        let wasm = format!("{name_str}.wasm");

        let input = dist_web_dir(self.release)
            .join(wasm.as_str())
            .to_string_lossy()
            .into_owned();
        let output = docs_web_dir(self.release, name_str)
            .to_string_lossy()
            .into_owned();

        let status = wasm_bindgen(input.as_str(), output.as_str(), !self.release)?;
        if !status.success() {
            Err("Command 'wasm-bindgen' failed")?;
        }

        if self.release {
            let wasm_file = format!("{name_str}_bg.wasm");
            let inout = docs_web_dir(self.release, name_str)
                .join(wasm_file.as_str())
                .to_string_lossy()
                .into_owned();

            let status = wasm_opt(inout.as_str(), inout.as_str())?;
            if !status.success() {
                Err("Command 'wasm-opt' failed")?;
            }

            if self.gzip {
                let wasm_gz = format!("{wasm_file}.gz");
                let wasm_gz_output = docs_web_dir(self.release, name_str)
                    .join(wasm_gz)
                    .to_string_lossy()
                    .into_owned();

                let js_gz_input = docs_web_dir(self.release, name_str)
                    .join(format!("{name_str}.js"))
                    .to_string_lossy()
                    .into_owned();
                let js_gz_output = docs_web_dir(self.release, name_str)
                    .join(format!("{name_str}.js.gz"))
                    .to_string_lossy()
                    .into_owned();

                gz_file(inout.as_str(), wasm_gz_output.as_str());
                gz_file(js_gz_input.as_str(), js_gz_output.as_str());
            }
        }

        let example_file = format!("{name_str}.html");

        let file_in = File::open(res_dir().join("example.html"))?;
        let reader = BufReader::new(file_in);

        let mut output_lines = Vec::new();
        for line in reader.lines() {
            let mut line = line?;

            line = line.replace("{{ EXAMPLE }}", self.name.as_str());
            output_lines.push(line);
        }

        let mut file_out =
            File::create(docs_web_dir(self.release, "").join(example_file.as_str()))?;

        for line in &output_lines {
            writeln!(file_out, "{}", line)?;
        }

        if !self.no_assets {
            copy_assets(docs_web_dir(self.release, ""))
        }

        Ok(())
    }
}

pub fn docs_web_dir(release: bool, name: &str) -> PathBuf {
    project_root()
        .join("docs/web_examples/")
        .join(match release {
            true => "release",
            false => "debug",
        })
        .join("examples")
        .join(name)
}

fn dist_web_dir(release: bool) -> PathBuf {
    project_root()
        .join("target/wasm32-unknown-unknown")
        .join(match release {
            true => "release",
            false => "debug",
        })
        .join("examples")
}

pub fn res_dir() -> PathBuf {
    project_root().join("xtask/res")
}
