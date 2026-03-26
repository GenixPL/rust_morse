use std::cell::RefCell;
use std::rc::Rc;
use crate::audio_handler::audio_handler::AudioHandler;
use crate::features::feature::Feature;
use crate::morse::morse_encoder::morse_encode;

pub struct EncodeMorseFeature {
    audio_handler: Rc<RefCell<dyn AudioHandler>>,
}

impl EncodeMorseFeature {
    pub fn new(audio_handler:Rc<RefCell<dyn AudioHandler>>) -> Self {
        Self {
            audio_handler,
        }
    }
}

impl Feature for EncodeMorseFeature {
    fn get_name(&self) -> &'static str {
        "Encode Morse"
    }

    fn get_command(&self) -> &'static str {
        "e"
    }

    fn handle_text(&mut self, text: String) {
        let file_path = "recordings/working_morse_encode.wav";
        morse_encode(text.as_str(), file_path).unwrap();
        self.audio_handler.borrow().play(file_path)
    }

    fn print(&self) -> Vec<String> {
        vec![
            "Enter the text and press <Enter> to play morse".to_string()
        ]
    }
} 