pub trait AudioHandler {
    fn init(&mut self);

    fn play(&self);
}