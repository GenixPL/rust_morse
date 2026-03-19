pub enum AudioRecorderState {
    Recording,
    Stopped,
}

pub trait AudioRecorder {
    fn get_state(&self) -> &AudioRecorderState;

    fn init(&mut self);

    fn record(&mut self);

    fn stop(&mut self);
}