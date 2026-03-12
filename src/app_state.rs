use std::time::{Instant, Duration};
use strum_macros::Display;

#[derive(Display)]
pub enum TimerState {
    Running,
    Stopped,
}

pub struct AppState {
    pub timer_state: TimerState,
    last_start_at_millis: Option<Instant>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            timer_state: TimerState::Stopped,
            last_start_at_millis: None,
        }
    }
}

impl AppState {
    pub fn start(&mut self) {
        self.timer_state = TimerState::Running;
        self.last_start_at_millis = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.timer_state = TimerState::Stopped;
        self.last_start_at_millis = None;
    }

    pub fn get_elapsed_time(&self) -> Option<Duration> {
        match self.last_start_at_millis {
            None => None,
            Some(instant) => Some(instant.elapsed()),
        }
    }
}