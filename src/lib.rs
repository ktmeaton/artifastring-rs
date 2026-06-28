pub mod actions2wav;
pub mod cli;
pub mod constants;
pub mod convolution;
pub mod instrument;
pub mod monowav;
pub mod string;

use color_eyre::eyre::{eyre, Report, Result};
use std::str::FromStr;
use convolution::*;
use instrument::*;
use monowav::*;
use string::*;

#[derive(Debug, Default, PartialEq)]
pub enum ActionType {
    #[default]
    Off,
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
#[allow(dead_code)]
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

        // Validate
        if let Some(p) = position {
            if p < 0.0 || p > 1.0 {
                return Err(eyre!("Position must be between 0.0 and 1.0"))
            }
        }

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