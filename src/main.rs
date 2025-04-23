#![allow(dead_code, unused_variables, unused_imports)]
use anyhow::{anyhow, Result};
use data::{Data, Metadata, Section, Wrapper};
use log::{error, info, LevelFilter};
use simple_logger::SimpleLogger;
use std::{
    collections::BTreeMap,
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
};
use structopt::StructOpt;

mod data;
mod html;
mod parser;

#[derive(Debug, Clone, StructOpt)]
#[structopt(
    about = "Convert a resume from markdown to some easily parsable format. Currently supports toml, yaml, and json."
)]
struct Opt {
    /// Increase verbosity
    #[structopt(short, long)]
    verbose: bool,

    /// Input file
    #[structopt(short, long, default_value = "resume.md")]
    input: PathBuf,

    /// Output file
    #[structopt(short, long, default_value = "resume.toml")]
    output: PathBuf,
}

fn main() {
    let options = Opt::from_args();

    let logger = SimpleLogger::new().with_colors(true);
    if options.verbose {
        logger.with_level(LevelFilter::Debug).init().unwrap();
    } else {
        logger.with_level(LevelFilter::Info).init().unwrap();
    }

    if let Err(e) = run(options) {
        error!("{e}");
    } else {
        info!("Success!");
    }
}

fn run(options: Opt) -> Result<()> {
    let (metadata, data) = parser::parse(&read_to_string(options.input)?)?;
    let output_str = options.output.to_string_lossy();
    if output_str.ends_with(".toml") {
        let mut file = File::create(&options.output).expect("Can't open output");
        file.write_all(toml::to_string(&Wrapper { metadata, data })?.as_bytes())?;
    } else if output_str.ends_with(".yaml") {
        let mut file = File::create(&options.output).expect("Can't open output");
        file.write_all(serde_yaml::to_string(&Wrapper { metadata, data })?.as_bytes())?;
    } else if output_str.ends_with(".json") {
        let mut file = File::create(&options.output).expect("Can't open output");
        file.write_all(serde_json::to_string(&Wrapper { metadata, data })?.as_bytes())?;
    } else if output_str.ends_with(".html") {
        let html = html::create_html(Wrapper { metadata, data })?;
        let mut file = File::create(&options.output).expect("Can't open output");
        file.write_all(html.as_bytes())?;
    } else {
        return Err(anyhow!(
            "Invalid output format. Valid formats: toml, yaml, json."
        ));
    }
    Ok(())
}
