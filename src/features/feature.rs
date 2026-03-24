pub trait Feature {
    fn get_name(&self) -> &'static str;

    fn get_command(&self) -> &'static str;

    fn handle_text(&mut self, text: String);

    fn print(&self) -> Vec<String>;
}