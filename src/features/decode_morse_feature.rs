use crate::features::feature::Feature;
use crate::morse::morse_decoder::morse_decode;

#[derive(Default)]
pub struct DecodedMorseFeature {}

impl Feature for DecodedMorseFeature {
    fn get_name(&self) -> &'static str {
        "Decoded Morse"
    }

    fn get_command(&self) -> &'static str {
        "d"
    }

    fn handle_text(&mut self, text: String) {
        let decoded = morse_decode(&text);
        print!("decoded: {} ", decoded);
    }

    fn print(&self) -> Vec<String> {
        vec![
            "Enter file path and press <Enter>.".to_string()
        ]
    }
}