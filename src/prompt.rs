use crate::Terminal;
use termion::event::Key;

pub struct Prompt {
    terminal: Terminal,
}

impl Prompt {
    pub fn default() -> Self {
        Self {
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }

    pub fn wait_question(&self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Char('\n') => {
                println!("bla");
            }
            Key::Up => print!("UP"),
            Key::Down => print!("DOWN"),
            _ => (),
        }
        Terminal::flush()
    }
}
