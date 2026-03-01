pub mod actions2wav;

use clap::{Parser, Subcommand, ValueEnum};
use log::LevelFilter;
use std::default::Default;


// ----------------------------------------------------------------------------
// CLI Entry Point
// ----------------------------------------------------------------------------

#[derive(Clone, Parser, Debug)]
#[clap(name = "artifastring", trailing_var_arg = true)]
#[clap(author, version)]
#[clap(verbatim_doc_comment)]
#[clap(arg_required_else_help = true)]
pub struct Cli {
    #[clap(subcommand)]
    // subcommand (actions2wav)
    pub command: Command,

    // ------------------------------------------------------------------------
    // Global Options

    /// Output verbosity level.
    #[clap(short = 'v', long)]
    #[clap(value_enum, default_value_t = Verbosity::default())]
    #[clap(hide_possible_values = false)]
    pub verbosity: Verbosity,
}

/// artifastring CLI commands (actions2wav).
#[derive(Clone, Subcommand, Debug)]
#[clap(verbatim_doc_comment)]
pub enum Command {
    /// Convert an actions file to wav
    #[command(name = "actions2wav")]
    Actions2Wav(Box<actions2wav::Args>),
}


// -----------------------------------------------------------------------------
// Verbosity
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Default, ValueEnum)]
pub enum Verbosity {
    #[default]
    Info,
    Warn,
    Debug,
    Error,
}

impl Verbosity {
    pub fn to_level_filter(&self) -> LevelFilter {
        match self {
            Verbosity::Info => LevelFilter::Info,
            _ => LevelFilter::Info,
        }
    }
}

impl std::fmt::Display for Verbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Convert to lowercase for RUST_LOG env var compatibility
        let lowercase = format!("{:?}", self).to_lowercase();
        write!(f, "{lowercase}")
    }
}
