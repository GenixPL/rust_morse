use std::sync::mpsc::Sender;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};


pub struct TextHandler {
    input: String,
    on_enter_callback: Sender<String>,

}

impl TextHandler {
    pub fn new(sender: Sender<String>) -> Self {
        Self {
            input: "".to_string(),
            on_enter_callback: sender,
        }
    }

    pub fn get_input_state(&self) -> &str {
        self.input.as_str()
    }

    pub fn handle_key(&mut self, key_event: KeyEvent) {
        if key_event.kind != KeyEventKind::Press {
            return;
        }

        if key_event.code == KeyCode::Backspace {
            self.input.pop();
            return;
        }

        if key_event.code == KeyCode::Enter {
            let _ = self.on_enter_callback.send(self.input.clone());
            self.input.clear();
            return;
        }

        match key_event.code.as_char() {
            None => {
                println!("Unrecognized key event: {:?}", key_event);
            }
            Some(char) => {
                self.input.push(char);
            }
        }
    }
}