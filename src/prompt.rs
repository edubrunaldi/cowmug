use crate::CursorMove;
use crate::Terminal;
use termion::event::Key;

pub struct Prompt {
    terminal: Terminal,
    clean_all_terminal: bool,
    cursor_position: usize,
}

impl Prompt {
    pub fn default() -> Self {
        Self {
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            clean_all_terminal: true,
            cursor_position: 4,
        }
    }

    pub fn write_question(&mut self, question: String) -> Result<(), std::io::Error> {
        if self.clean_all_terminal {
            self.terminal.clean()?;
            Terminal::flush()?;
        }
        self.terminal.write(&question[..])?;
        self.terminal.write("opcao A")?;
        self.terminal.write("opcao B")?;
        self.terminal.write("opcao C")?;
        self.wait_question()?;
        if self.clean_all_terminal {
            self.terminal.clean()?;
        }
        Ok(())
    }

    fn wait_question(&mut self) -> Result<(), std::io::Error> {
        loop {
            let pressed_key = Terminal::read_key()?;
            match pressed_key {
                Key::Char('q') => break,
                Key::Up => self.cursor_position -= 1,
                Key::Down => self.cursor_position += 1,
                _ => (),
            }

            self.terminal.clean_line()?;
        }
        self.terminal.show()?;
        self.refresh_screen()?;
        Ok(())
    }
    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        // Terminal::cursor_position(&Position::default());
        self.terminal.cursor_position(CursorMove::Up);
        Terminal::cursor_show();
        Terminal::flush()
    }
}
