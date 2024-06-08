use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crate::cli::{Example, Examples};
use crate::cli_example_web::docs_web_dir;
use crate::{copy_assets, project_root, DynError};

impl Examples {
    pub(crate) fn run_web(self) -> Result<(), DynError> {
        copy_assets(docs_web_dir("assets"));

        let mut doc_body = String::from("<ul>\n");

        let examples_path = project_root()
            .join("examples")
            .to_string_lossy()
            .into_owned();

        let target = self.target;
        let release = self.release;
        let gzip = self.gzip;

        self.list_files(examples_path.as_str())?
            .for_each(|example| {
                let name = example.file_stem().unwrap().to_str().unwrap().to_owned();
                let name_str = name.as_str().to_owned();

                let example = Example {
                    name,
                    target,
                    release,
                    no_assets: true,
                    gzip,
                };

                let _ = example.run();

                let url = format!("examples/{name_str}.html");
                let image = format!("examples/images/{name_str}.jpg");
                let tmp = format!("\n<li><a href=\"{url}\"><div class=\"example-image\"><img src=\"{image}\" alt=\"{name_str}\"></div><div class=\"example-link\">{name_str}</div></a></li>");

                doc_body.push_str(tmp.as_str());
            });

        doc_body.push_str("\n</ul>");

        let file_in = File::open(crate::cli_example_web::res_dir().join("docs.html"))?;
        let reader = BufReader::new(file_in);

        let mut output_lines = Vec::new();
        for line in reader.lines() {
            let mut line = line?;

            line = line.replace("{{ BODY }}", doc_body.as_str());
            output_lines.push(line);
        }

        let mut file_out = File::create(docs_web_dir("../").join("index.html"))?;

        for line in &output_lines {
            writeln!(file_out, "{}", line)?;
        }

        Ok(())
    }
}
