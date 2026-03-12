use std::thread;
use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Direction, Layout};
use crate::app_state::AppState;

#[derive(Default)]
pub struct AppRunner {
    app_state: AppState,
}

impl AppRunner {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            // Wait up to 16ms (approx 60fps) for an event
            if poll(Duration::from_millis(16))? {
                if let Event::Key(key) = read()? {
                    if key.kind == KeyEventKind::Press {
                        // handle key
                        println!("{:?}", key)
                    }
                }
            }

            thread::sleep(Duration::from_millis(500))
        }
    }

    fn render(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(frame.area());


        frame.render_widget("Press F to start timer", layout[0]);
        frame.render_widget("Press S to start timer", layout[1]);
        frame.render_widget(format!("Timer state: {}", self.app_state.timer_state), layout[2]);
        frame.render_widget("Time:", layout[3]);
    }
}