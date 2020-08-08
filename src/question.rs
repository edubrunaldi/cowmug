pub struct QuestionList {
    question: String,
    choices: Vec<String>,
    answer: Option<Answer>,
}

pub struct Answer {
    pub index: usize,
    pub answer: String,
}

impl QuestionList {
    pub fn new(question: String) -> Self {
        QuestionList {
            question,
            choices: Vec::new(),
            answer: None,
        }
    }

    pub fn question(&self) -> &String {
        &self.question
    }

    pub fn answer_mut(&mut self) -> &mut Option<Answer> {
        &mut self.answer
    }

    pub fn answer_index(&self) -> Option<usize> {
        if let Some(answer) = &self.answer {
            return Some(answer.index);
        }
        return None;
    }

    pub fn answer_string(&self) -> Option<&String> {
        if let Some(answer) = &self.answer {
            return Some(&answer.answer);
        }
        return None;
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
