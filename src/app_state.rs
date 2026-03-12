use strum_macros::Display;

#[derive(Display)]
pub enum TimerState {
    Running,
    Stopped,
}

pub struct AppState {
    pub timer_state: TimerState,
    last_start_at_millis: Option<u32>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            timer_state: TimerState::Stopped,
            last_start_at_millis: None,
        }
    }
}