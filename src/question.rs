pub struct QuestionList {
    question: String,
    choices: Vec<String>,
    answer: Option<usize>,
}

impl QuestionList {
    pub fn new() -> Self {
        QuestionList {
            question: String::new(),
            choices: Vec::new(),
            answer: None,
        }
    }

    pub fn question_mut(&mut self) -> &mut String {
        &mut self.question
    }

    pub fn question(&self) -> &String {
        &self.question
    }

    pub fn answer_mut(&mut self) -> &mut Option<usize> {
        &mut self.answer
    }

    pub fn answer(&self) -> &Option<usize> {
        &self.answer
    }

    pub fn add_choice(&mut self, choice: String) {
        self.choices.push(choice);
    }

    pub fn add_choices(&mut self, choices: Vec<String>) {
        self.choices.extend(choices);
    }

    pub fn choices(&self) -> &Vec<String> {
        &self.choices
    }
}
