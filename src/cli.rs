use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// A representation of the CLI. This struct contains any parameters that are
/// shared in common among all of the subcommands. Subcommand-specific
/// parameters must be palced in the `Command` struct.
#[derive(Debug, Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

/// A representation of the subcommand to run. Each variant encapsulates the
/// subcommand-specific parameters.
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Fetch a single citation's metadata, independent of any environment,
    /// including any `Tome.toml` or `Tome.lock` that may be present in the
    /// directory.
    Fetch {
        identifier: String,
        parser: PathBuf,
        fetcher: PathBuf,
    },
}
