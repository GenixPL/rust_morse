use crate::features::feature::Feature;
use crate::timer::timer::{Timer};

#[derive(Default)]
pub struct TimerFeature {
    timer: Timer,
}

impl Feature for TimerFeature {
    fn get_name(&self) -> &'static str {
        "Timer"
    }

    fn get_command(&self) -> &'static str {
        "t"
    }

    fn handle_text(&mut self, text: String) {
        let parts: Vec<&str> = text.split(' ').collect();
        let first_part = parts[0];
        match first_part {
            "f" => self.timer.start(),
            "s" => self.timer.stop(),
            &_ => {}
        }
    }

    fn print(&self) -> Vec<String> {
        vec![
            format!("State: {}", self.timer.get_state()),
            format!("Time: {:?}", self.timer.get_elapsed_time()),
            "Press <f> to start.".to_string(),
            "Press <s> to stop.".to_string(),
        ]
    }
}