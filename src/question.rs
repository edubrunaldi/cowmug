#[derive(Clone)]
pub struct Question {
    question: String,
    answer: Option<bool>,
}

impl Question {
    pub fn new() -> Self {
        Self {
            question: String::from(""),
            answer: None,
        }
    }

    pub fn set_question(&mut self, question: String) {
        self.question = question;
    }

    pub fn get_question(&self) -> &String {
        &self.question
    }

    pub fn set_answer(&mut self, answer: bool) {
        self.answer = Some(answer);
    }

    pub fn get_answer(&self) -> Option<bool> {
        self.answer
    }
}
