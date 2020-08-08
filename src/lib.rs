mod prompt;
mod question;
mod terminal;

pub use prompt::Prompt;
pub use question::Answer;
pub use question::QuestionList;
pub use terminal::CursorMove;
pub use terminal::Terminal;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
