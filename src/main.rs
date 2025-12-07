use clap::Parser;
use color_eyre::Section;
use color_eyre::eyre::Context;
use color_eyre::eyre::{Result, eyre};
use std::{path::PathBuf, process::Command};

mod cli;
use crate::cli::Cli;

fn main() -> Result<()> {
    let _ = color_eyre::install();
    let cli = Cli::parse();
    match cli.command {
        cli::Command::Fetch {
            identifier,
            parser,
            fetcher,
        } => {
            let parsed = parse_identifier(&identifier, &parser)?;
            let fetched = fetch_parsed(&parsed, &fetcher)
                .suggestion(format!("Original identifier is {identifier}"));
            println!("{fetched:?}");
        }
    }
    Ok(())
}

fn parse_identifier(identifier: &str, parser: &PathBuf) -> Result<String> {
    Command::new(parser)
        .arg(identifier)
        .output()
        .map(|res| String::from_utf8(res.stdout))
        .map_err(|e| eyre!(e))?
        .map_err(|e| eyre!(e))
        .wrap_err(format!(
            "Failed to parse identifier {} using {}",
            identifier,
            parser.as_os_str().to_string_lossy(),
        ))
}

fn fetch_parsed(parsed: &str, fetcher: &PathBuf) -> Result<String> {
    Command::new(fetcher)
        .arg(parsed)
        .output()
        .map(|res| String::from_utf8(res.stdout))
        .map_err(|e| eyre!(e))?
        .map_err(|e| eyre!(e))
        .wrap_err(format!(
            "Failed to fetch parsed identifier {} using {}",
            parsed,
            fetcher.as_os_str().to_string_lossy(),
        ))
}
