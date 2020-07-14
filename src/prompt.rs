use crate::CursorMove;
use crate::Question;
use crate::Terminal;
use termion::event::Key;

pub struct Prompt {
    terminal: Terminal,
    clean_all_terminal: bool,
    cursor_position: usize,
}

impl Prompt {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            clean_all_terminal: true,
            cursor_position: 0,
        }
    }

    pub fn exec(&mut self, questions: &mut Vec<Question>) {
        for question in questions {
            println!("{:?}", question.get_question());
        }
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        self.terminal.cursor_position(self.cursor_position);
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Char('q') => print!("aaa"),
            _ => (),
        }

        Ok(())
    }
}

fn die(e: std::io::Error) {
    panic!(e);
}
