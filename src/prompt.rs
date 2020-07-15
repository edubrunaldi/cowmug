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
        self.terminal.hold_stdout();
        for question in questions {
            // self.refresh_screen()?;
            self.refresh_screen(&question)?;
            let answer = self.process_keypress()?;
            match answer.trim() {
                "y" => question.set_answer(true),
                "n" => question.set_answer(false),
                _ => (),
            };
        }
        Terminal::flush()?;
        self.terminal.free_stdout();
        Ok(())
    }

    fn refresh_screen(&mut self, question: &Question) -> Result<(), std::io::Error> {
        self.terminal.reset_cursor_position();
        self.terminal.clean_after_cursor();
        Terminal::cursor_hide();
        self.draw(question);
        self.terminal
            .move_cursor_horizontal(question.get_question().len());
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw(&mut self, question: &Question) {
        self.terminal
            .write(question.get_question().as_str())
            .unwrap();
    }

    fn process_keypress(&mut self) -> Result<String, std::io::Error> {
        let mut answer = String::new();
        loop {
            let pressed_key = Terminal::read_key()?;
            match pressed_key {
                Key::Char('\n') => {
                    return Ok(answer);
                }
                Key::Char('\r') => {
                    return Ok(answer);
                }
                Key::Char(pressed_key) => answer.push(pressed_key),
                _ => (),
            }
        }
    }
}

fn die(e: std::io::Error) {
    panic!(e);
}
