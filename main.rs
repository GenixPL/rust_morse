use chrono::Local;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::*;
use std::thread;
use std::time::Duration;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut text: String = String::default();

    loop {
        terminal.draw(render)?;

        // if crossterm::event::read()?.is_key_press() {
        //     break Ok(());
        // }

        thread::sleep(Duration::from_millis(500))
    }
}

fn render(frame: &mut Frame) {
    let time_text: String = Local::now().to_string();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    frame.render_widget(time_text, layout[0]);
    frame.render_widget("hello", layout[1]);
}
