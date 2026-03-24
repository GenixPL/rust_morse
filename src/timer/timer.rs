use std::time::{Duration, Instant};
use strum_macros::Display;

#[derive(Display)]
pub enum TimerState {
    Running,
    Stopped,
}

pub struct Timer {
    state: TimerState,
    last_start_at_millis: Option<Instant>,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            state: TimerState::Stopped,
            last_start_at_millis: None,
        }
    }
}

impl Timer {
    pub fn get_state(&self) -> &TimerState {
        &self.state
    }

    pub fn start(&mut self) {
        self.state = TimerState::Running;
        self.last_start_at_millis = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.state = TimerState::Stopped;
        self.last_start_at_millis = None;
    }

    pub fn get_elapsed_time(&self) -> Duration {
        match self.last_start_at_millis {
            None => Duration::default(),
            Some(instant) => instant.elapsed(),
        }
    }
}