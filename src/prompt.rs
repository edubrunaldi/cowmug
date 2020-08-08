use crate::Terminal;
use crate::{Answer, Question};
use termion::event::Key;

pub struct Prompt {
    terminal: Terminal,
    cursor_position: usize,
    cursor_last_position: usize,
    next_question: bool,
}

impl Prompt {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: 0,
            cursor_last_position: 0,
            next_question: false,
        }
    }

    pub fn exec(&mut self, questions: &mut Vec<Question>) -> Result<(), std::io::Error> {
        self.setup_start();
        for question in questions {
            self.cursor_position = question.choices().len();
            self.next_question = false;
            self.cursor_position = 0;
            self.cursor_last_position = 0;
            while !self.next_question {
                self.refresh_screen(&question);
                self.process_keypress(question.choices().len()).unwrap();
                if self.next_question {
                    *question.answer_mut() = Some(Answer {
                        index: self.cursor_position,
                        answer: format!(
                            "{}",
                            question.choices().get(self.cursor_position).unwrap()
                        ),
                    });
                }
            }
        }
        self.setup_end();
        Ok(())
    }

    fn setup_start(&mut self) {
        self.terminal.hold_stdout();
        Terminal::cursor_hide();
    }

    fn setup_end(&mut self) {
        self.terminal.reset_cursor_position();
        Terminal::clean_after_cursor();
        Terminal::cursor_show();
        Terminal::flush().unwrap();
        self.terminal.free_stdout();
    }

    fn refresh_screen(&mut self, question: &Question) {
        self.terminal.reset_cursor_position();
        Terminal::clean_after_cursor();
        self.draw(question);
        Terminal::flush().unwrap();
    }

    fn draw(&mut self, question: &Question) {
        self.terminal.writeln(question.question().as_str()).unwrap();

        for i in 0..question.choices().len() {
            let mut printable_choice = if i == self.cursor_position {
                Terminal::set_green_color();
                String::from("> ")
            } else {
                Terminal::reset_color();
                String::from("  ")
            };
            printable_choice.push_str(question.choices().get(i).unwrap().as_str());
            self.terminal.writeln(printable_choice.as_str()).unwrap();
            Terminal::reset_color();
        }
    }

    fn process_keypress(&mut self, choices_len: usize) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Char('\n') | Key::Char('\r') => self.next_question = true,
            Key::Up => {
                if self.cursor_position > 0 {
                    self.cursor_last_position = self.cursor_position;
                    self.cursor_position -= 1;
                } else {
                    self.cursor_last_position = self.cursor_position;
                    self.cursor_position = choices_len - 1;
                }
            }
            Key::Down => {
                if self.cursor_position < choices_len - 1 {
                    self.cursor_last_position = self.cursor_position;
                    self.cursor_position += 1;
                } else {
                    self.cursor_last_position = self.cursor_position;
                    self.cursor_position = 0;
                }
            }
            _ => (),
        }
        Ok(())
    }
}
