use std::sync::mpsc::Receiver;
use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Direction, Layout};
use crate::audio_handler::audio_handler::*;
use crate::audio_recorder::audio_recorder::{AudioRecorder};
use crate::features::feature::Feature;
use crate::features::timer_feature::TimerFeature;
use crate::text_handler::text_handler::TextHandler;

pub struct AppRunner {
    audio_handler: Box<dyn AudioHandler>,
    audio_recorder: Box<dyn AudioRecorder>,

    text_receiver: Receiver<String>,
    text_handler: TextHandler,
    features: Vec<Box<dyn Feature>>,
    active_feature: Option<Box<dyn Feature>>,
}

impl AppRunner {
    pub fn new(
        audio_handler: Box<dyn AudioHandler>,
        audio_recorder: Box<dyn AudioRecorder>,
    ) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            audio_handler,
            audio_recorder,
            text_receiver: rx,
            text_handler: TextHandler::new(tx),
            active_feature: None,
            features: vec![
                Box::new(TimerFeature::default()),
            ],
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        self.audio_handler.init();
        self.audio_recorder.init();

        loop {
            terminal.draw(|frame| self.render(frame))?;

            // Wait up to 16ms (approx 60fps) for an event
            if poll(Duration::from_millis(16))? {
                if let Event::Key(key) = read()? {
                    if key.code == KeyCode::Esc {
                        if self.active_feature.is_some() {
                            self.active_feature = None;
                            continue;
                        } else {
                            return Ok(());
                        }
                    }

                    self.text_handler.handle_key(key);
                }
            }

            match self.text_receiver.try_recv() {
                Err(_) => {}
                Ok(input) => {
                    if self.active_feature.is_some() {
                        self.active_feature.as_mut().unwrap().handle_text(input);
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
                // Constraint::Length(1),
                // Constraint::Length(1),
            ])
            .split(frame.area());


        frame.render_widget("", layout[0]);
        frame.render_widget(format!("Input: {}", self.text_handler.get_input_state()), layout[1]);
        frame.render_widget("Press <F> to start timer", layout[2]);
        frame.render_widget("Press <S> to start timer", layout[3]);
        // frame.render_widget(format!("Timer state: {}", self.app_state.timer_state), layout[4]);
        // frame.render_widget(format!("Time: {:?}", self.app_state.get_elapsed_time()), layout[5]);
    }
}