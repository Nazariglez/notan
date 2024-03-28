mod cli;

mod cli_docs;
mod cli_example;
mod cli_example_msvc;
mod cli_example_web;
mod cli_examples;
mod cli_examples_msvc;
mod cli_examples_web;

use std::env;
use std::fs::File;
use std::io::{BufReader, copy, Error};
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use flate2::Compression;
use flate2::write::GzEncoder;
use cli::{Cli, CliCmd};
use crate::cli::TargetType;

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let flags = Cli::from_env_or_exit();

    match flags.subcommand {
        CliCmd::Docs(cmd) => cmd.run()?,
        CliCmd::Example(cmd) => cmd.run()?,
        CliCmd::Examples(cmd) => cmd.run()?,
    }

    Ok(())
}

fn project_root() -> PathBuf {
    let dir =
        env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());

    PathBuf::from(dir).parent().unwrap().to_owned()
}

fn assets_dir() -> PathBuf {
    project_root().join("examples/assets")
}

fn copy_assets(to: PathBuf) {
    let options = fs_extra::dir::CopyOptions::new()
        .overwrite(true)
        .copy_inside(true);

    let mut paths = Vec::new();
    paths.push(assets_dir().as_path().to_owned());

    let _ = fs_extra::copy_items(&paths, to, &options);
}

fn cargo_build(target: TargetType, profile: &str, name: &str) -> Result<ExitStatus, Error> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    // We want this to be configurable?
    let features = "glyph,egui,text,extra,audio,links,drop_files,clipboard,save_file,texture_to_file";
    let features_arg = format!("--features={features}");

    match target {
        TargetType::Msvc => Command::new(cargo)
            .current_dir(project_root())
            .args(&["build", "--target", "x86_64-pc-windows-msvc", "--profile", profile, "--example", name, features_arg.as_str()])
            .status(),

        TargetType::Web => Command::new(cargo)
            .current_dir(project_root())
            .env("RUSTFLAGS", "--cfg=web_sys_unstable_apis")
            .args(&["build", "--target", "wasm32-unknown-unknown", "--profile", profile, "--example", name, features_arg.as_str()])
            .status()
    }
}

fn wasm_bindgen(input: &str, output: &str, debug: bool) -> Result<ExitStatus, Error> {
    Command::new("wasm-bindgen")
        .current_dir(project_root())
        .args(&[
            [input, "--out-dir", output, "--no-modules", "--browser"].as_slice(),
            match debug {
                true => &["--keep-debug", "--debug"].as_slice(),
                false => &[].as_slice(),
            }
        ].concat())
        .status()
}

fn wasm_opt(input: &str, output: &str) -> Result<ExitStatus, Error> {
    Command::new("wasm-opt")
        .current_dir(project_root())
        .args(&["-O", "-o", input, output])
        .status()
}

fn gz_file(input: &str, output: &str) {
    let mut input_wasm = BufReader::new(File::open(input).unwrap());
    let output_wasm = File::create(output).unwrap();

    let mut encoder_wasm = GzEncoder::new(output_wasm, Compression::default());

    copy(&mut input_wasm, &mut encoder_wasm).unwrap();
    encoder_wasm.finish().unwrap();
}
