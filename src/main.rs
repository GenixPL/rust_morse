use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::*;
use crossterm::event::*;
use std::thread;
use std::time::Duration;
use crate::app_runner::AppRunner;
use crate::app_state::*;

mod app_state;
mod app_runner;

fn main() -> color_eyre::Result<()> {
    let mut app_runner: AppRunner = AppRunner::default();

    color_eyre::install()?;
    run(|terminal| app_runner.run(terminal))?;
    Ok(())
}








