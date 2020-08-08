mod prompt;
mod question;
mod terminal;

pub use prompt::Prompt;
use question::Answer;
pub use question::Question;
use terminal::Terminal;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
