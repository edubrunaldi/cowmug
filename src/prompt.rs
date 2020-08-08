use crate::CursorMove;
use crate::QuestionList;
use crate::Terminal;
use std::{thread, time};
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
    fn test() {
        let five_sec = time::Duration::from_secs(2);
        thread::sleep(five_sec);
    }

    pub fn exec(&mut self, questions: &mut Vec<QuestionList>) -> Result<(), std::io::Error> {
        self.terminal.hold_stdout();
        for question in questions {
            self.cursor_position = question.choices().len();
            self.next_question = false;
            self.cursor_position = 0;
            self.cursor_last_position = 0;
            // self.draw(question);
            while !self.next_question {
                self.refresh_screen(&question);
                self.process_keypress(question.choices().len()).unwrap();
                if self.next_question {
                    *question.answer_mut() = Some(self.cursor_position);
                }
            }
        }
        Terminal::flush()?;
        self.terminal.free_stdout();
        Ok(())
    }

    fn refresh_screen(&mut self, question: &QuestionList) {
        Terminal::cursor_hide();
        self.terminal.reset_cursor_position();
        self.terminal.clean_after_cursor();
        // Terminal::flush().unwrap();
        self.draw(question);
        self.terminal.set_cursor_first_choice();
        if self.cursor_last_position > self.cursor_position {
            self.terminal.move_cursor_up(self.cursor_last_position);
        } else if self.cursor_last_position < self.cursor_position {
            self.terminal.move_cursor_down(self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush().unwrap();
    }

    fn draw(&mut self, question: &QuestionList) {
        self.terminal.writeln(question.question().as_str()).unwrap();
        for choice in question.choices() {
            self.terminal.writeln(choice.as_str()).unwrap();
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
                }
            }
            Key::Down => {
                if self.cursor_position < choices_len - 1 {
                    self.cursor_last_position = self.cursor_position;
                    self.cursor_position += 1;
                }
            }
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&self, pressed_key: Key) {}
}

fn die(e: std::io::Error) {
    panic!(e);
}
