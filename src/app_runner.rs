use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Direction, Layout};
use crate::app_state::AppState;
use crate::audio_handler::audio_handler::*;

pub struct AppRunner {
    app_state: AppState,
    audio_handler: Box<dyn AudioHandler>,
}

impl AppRunner {
    pub fn new(audio_handler: Box<dyn AudioHandler>) -> Self {
        Self {
            app_state: Default::default(),
            audio_handler,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        self.audio_handler.init();
        self.audio_handler.play();

        return Ok(());

        loop {
            terminal.draw(|frame| self.render(frame))?;

            // Wait up to 16ms (approx 60fps) for an event
            if poll(Duration::from_millis(16))? {
                if let Event::Key(key) = read()? {
                    self.input_handler(key);
                }
            }

            if self.app_state.quit {
                return Ok(());
            }
        }
    }

    fn input_handler(&mut self, key_event: KeyEvent) {
        if key_event.kind == KeyEventKind::Press {
            // handle key
            match key_event.code.as_char() {
                None => {}
                Some(char) => {
                    match char {
                        's' => self.app_state.stop(),
                        'f' => self.app_state.start(),
                        'q' => self.app_state.quit(),
                        'p' => self.audio_handler.play(),
                        _ => {}
                    }
                }
            }
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


        frame.render_widget("Press <F> to start timer", layout[0]);
        frame.render_widget("Press <S> to start timer", layout[1]);
        frame.render_widget(format!("Timer state: {}", self.app_state.timer_state), layout[2]);
        frame.render_widget(format!("Time: {:?}", self.app_state.get_elapsed_time()), layout[3]);
    }
}