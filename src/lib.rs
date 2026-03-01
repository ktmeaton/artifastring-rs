pub mod actions2wav;
pub mod cli;

use color_eyre::eyre::{eyre, Report, Result};
use std::str::FromStr;

#[derive(Debug)]
pub enum Action {
    Bow,
    BowAccelerate,
    Finger,
    Pluck,
    Release,
    Wait
}

impl FromStr for Action {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s {
            "a" => Self::BowAccelerate,
            "b" => Self::Bow,
            "f" => Self::Finger,
            "p" => Self::Pluck,
            "r" => Self::Release,
            "w" => Self::Wait,
            _ => Err(eyre!("Failed to convert '{s}' to action."))?,
        };

        Ok(action)
    }
}