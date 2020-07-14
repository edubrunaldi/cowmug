#[derive(Clone)]
pub struct Question {
    question: String,
}

impl Question {
    pub fn new() -> Self {
        Self {
            question: String::from(""),
        }
    }

    pub fn set_question(&mut self, question: String) {
        self.question = question;
    }

    pub fn get_question(&self) -> &String {
        &self.question
    }
}
