pub mod actions2wav;
pub mod cli;
pub mod constants;
pub mod convolution;
pub mod instrument;
pub mod monowav;
pub mod string;

use color_eyre::eyre::{eyre, Report, Result};
use std::str::FromStr;
use constants::*;
use convolution::*;
use instrument::*;
use monowav::*;
use string::*;

#[derive(Debug, PartialEq)]
pub enum ActionType {
    Bow,
    BowAccelerate,
    Finger,
    Pluck,
    Release,
    Wait
}

impl FromStr for ActionType {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s {
            "a" => Self::BowAccelerate,
            "b" => Self::Bow,
            "f" => Self::Finger,
            "p" => Self::Pluck,
            "r" => Self::Release,
            "w" => Self::Wait,
            _ => Err(eyre!("Failed to convert '{s}' to action"))?,
        };

        Ok(action)
    }
}

#[derive(Debug)]
pub enum StringNumber {
    One,
    Two,
    Three,
    Four
}

impl FromStr for StringNumber {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s {
            "0" => Self::One,
            "1" => Self::Two,
            "2" => Self::Three,
            "3" => Self::Four,
            _ => Err(eyre!("Failed to convert '{s}' to string number"))?,
        };

        Ok(action)
    }
}

#[derive(Debug)]
pub struct Action {
    action_type: ActionType,
    seconds: f32,
    string_number: Option<StringNumber>,
    position: Option<f32>,
    force: Option<f32>,
    velocity: Option<f32>,
    acceleration: Option<f32>,
}

impl Default for Action {
    fn default() -> Self {
        Self {
            action_type: ActionType::Wait,
            seconds: 0.0,
            string_number: None,
            position: None,
            force: None,
            velocity: None,
            acceleration: None,
        }
    }
}


impl Action {
    fn from_array(a: &[String]) -> Result<Self, Report> {

        let action_type = ActionType::from_str(&a[0])?;
        let seconds = a[1].parse::<f32>()?;
        let string_number = match action_type {
            ActionType::Wait => None,
            _ => Some(StringNumber::from_str(&a[2])?),
        };
        let position = match action_type {
            ActionType::Finger | ActionType::Pluck | ActionType::Bow | ActionType::BowAccelerate => Some(a[3].parse::<f32>()?),
            _ => None,
        };
        let force = match action_type {
            ActionType::Pluck | ActionType::Bow | ActionType::BowAccelerate => Some(a[4].parse::<f32>()?),
            _ => None,
        };
        let velocity = match action_type {
            ActionType::Bow | ActionType::BowAccelerate => Some(a[5].parse::<f32>()?),
            _ => None,
        };
        let acceleration = match action_type {
            ActionType::BowAccelerate => Some(a[6].parse::<f32>()?),
            _ => None,
        };
        Ok(Action { action_type, seconds, string_number, position, force, velocity, acceleration, ..Default::default() })
    }
}