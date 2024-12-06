use std::fs;
use std::ops::Range;

use color_eyre::Result;
use serde::Deserialize;

pub fn get() -> Result<Config> {
    let raw = fs::read_to_string("input.toml")?;
    let parsed = toml::from_str(&raw)?;
    Ok(parsed)
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(clippy::struct_field_names)]
pub struct Config {
    pub task_1: Task1,
    pub task_2: Task2,
    pub task_3: Task3,
}

#[derive(Deserialize)]
pub struct Task1 {
    pub interval: Range<f64>,
    pub tolerance: f64,
}

#[derive(Deserialize)]
pub struct Task2 {
    pub interval: Range<f64>,
    pub tolerance: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Task3 {
    pub initial_guess: f64,
    pub learning_rate: f64,
    pub iterations: usize,
}
