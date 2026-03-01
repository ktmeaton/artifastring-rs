use clap::Parser;
use std::path::PathBuf;

/// Convert an actions file to wav
#[derive(Clone, Debug, Parser)]
#[clap(verbatim_doc_comment)]
#[clap(arg_required_else_help = true)]
pub struct Args {
    /// Input actions file.
    #[clap(short = 'i', long, required = true)]
    pub input: PathBuf,

    /// Output wav file.
    #[clap(short = 'o', long, required = true)]
    pub output: PathBuf,
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

impl Args {
    pub fn new() -> Self {
        Args {
            input: PathBuf::new(),
            output: PathBuf::new(),
        }
    }
}