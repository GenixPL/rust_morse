pub trait AudioHandler {
    fn init(&mut self);

    fn play(&self, file_path: &str);
    
    // TODO(genix): add
    // fn pause(&self);
}