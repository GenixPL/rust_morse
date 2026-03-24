use std::cell::RefCell;
use std::fmt::format;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
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
    features: Vec<Rc<RefCell<dyn Feature>>>,
    active_feature: Option<Rc<RefCell<dyn Feature>>>,
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
                Rc::new(RefCell::new(TimerFeature::default()))
                // Box::new(),
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
                    if let Some(ref mut feature) = self.active_feature {
                        feature.borrow_mut().handle_text(input);
                    } else {
                        for feature in self.features.iter() {
                            if input == feature.borrow().get_command() {
                                self.active_feature = Some(self.features[0].clone());
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        // Create one block for the whole header/info section
        let area = frame.area();

        // Build a single string with newlines or a Vec of Spans
        let mut lines = vec![
            Line::from(format!("Active feature: {}", self.active_feature.as_ref().map_or("None", |f| f.borrow().get_name()))),
            Line::from(format!("Input: {}", self.text_handler.get_input_state())),
        ];

        if let Some(ref feature) = self.active_feature {
            lines.extend(
                feature.borrow().print().iter().map(
                    |e| {
                        Line::from(e.to_string())
                    }
                )
            );
        } else {
            lines.insert(0, Line::from("Available features:"));
            for feature in self.features.iter() {
                lines.insert(
                    1,
                    Line::from(
                        format!(
                            "\t {} (press <{}>)",
                            feature.borrow().get_name(),
                            feature.borrow().get_command(),
                        ),
                    ),
                );
            }
        }

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::NONE));

        frame.render_widget(paragraph, area);
    }
}