pub trait AudioHandler {
    fn init(&mut self);

    fn play(&self);
    
    // TODO(genix): add
    // fn pause(&self);
}