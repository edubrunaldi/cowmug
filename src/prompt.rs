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

    pub fn exec(&mut self, questions: &mut Vec<Question>) -> Result<(), std::io::Error> {
        for question in questions {
            // self.refresh_screen()?;
            self.refresh_screen(&question)?;
            let answer = Terminal::read_line();
            self.terminal.write(answer.trim());
            match answer.trim() {
                "y" => question.set_answer(true),
                "n" => question.set_answer(false),
                _ => (),
            };
        }
        Terminal::flush()?;
        Ok(())
    }

    fn refresh_screen(&mut self, question: &Question) -> Result<(), std::io::Error> {
        self.terminal.reset_cursor_position();
        self.terminal.clean_after_cursor();
        Terminal::cursor_hide();
        self.draw(question);
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw(&self, question: &Question) {
        print!("{}", question.get_question());
    }

    fn process_keypress(&mut self) -> Result<String, std::io::Error> {
        let answer = Terminal::read_line();
        loop {
            let pressed_key = Terminal::read_key()?;
            self.terminal.write(answer.as_str());
            match pressed_key {
                Key::Ctrl('q') => {
                    return Ok(answer);
                }
                _ => print!("{:?}", pressed_key),
            }
        }
    }
}

fn die(e: std::io::Error) {
    panic!(e);
}
