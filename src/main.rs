use clap::Parser;
use color_eyre::eyre::{Context, eyre};
use std::process::Command;

mod cli;
use crate::cli::Cli;

fn main() -> color_eyre::eyre::Result<()> {
    let _ = color_eyre::install();
    let cli = Cli::parse();
    match cli.command {
        cli::Command::Fetch {
            identifier,
            parser,
            fetcher,
        } => {
            let parsed = Command::new(&parser)
                .arg(&identifier)
                .output()
                .map(|res| String::from_utf8(res.stdout))
                .map_err(|e| eyre!(e))?
                .map_err(|e| eyre!(e))
                .wrap_err_with(|| {
                    format!(
                        "Failed to parse identifier {} using {}",
                        identifier,
                        parser.as_os_str().to_string_lossy(),
                    )
                })?;
            let fetched = Command::new(&fetcher)
                .arg(&parsed)
                .output()
                .map_err(|e| eyre!(e))
                .wrap_err_with(|| {
                    format!(
                        "Failed to fetch identifier {} using {}",
                        identifier,
                        fetcher.as_os_str().to_string_lossy(),
                    )
                })?;

            println!("{fetched:?}");
        }
    }
    Ok(())
}
