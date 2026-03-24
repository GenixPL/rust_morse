use crate::features::feature::Feature;
use crate::timer::timer::Timer;

#[derive(Default)]
pub struct TimerFeature {
    timer: Timer,
}

impl Feature for TimerFeature {
    fn get_name(&self) -> &'static str {
        "Timer"
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

    fn print(&self) -> Vec<&str> {
        vec![
            "Press <f> to start.",
            "Press <s> to stop.",
        ]
    }
}