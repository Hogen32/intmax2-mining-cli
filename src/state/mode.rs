use std::fmt::Display;

use clap::{Subcommand, ValueEnum};

#[derive(Subcommand, Debug, Copy, Clone, PartialEq, ValueEnum)]
pub enum RunMode {
    Mining,      // only mining
    Claim,       // only claim
    Exit,        // only withdraw or cancel pending deposits
    Interactive, // interactive mode
}

impl Display for RunMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunMode::Mining => write!(f, "Mining"),
            RunMode::Claim => write!(f, "Claim"),
            RunMode::Exit => write!(f, "Exit"),
            RunMode::Interactive => write!(f, "Interactive"),
        }
    }
}
