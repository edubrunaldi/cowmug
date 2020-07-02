mod prompt;
mod terminal;

pub use prompt::Prompt;
pub use terminal::Terminal;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
