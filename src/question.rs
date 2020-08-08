/// pub struct QuestionList
/// -
///
/// A Struct to wrapper questions and send it to Prompt to ask it
///
/// # Examples
///
/// ```
/// let mut question = Question::new(String::from("choose a color: ");
/// question.add_choice(String::from("Red"));
/// question.add_choice(String::from("Blue"));
/// question.add_choice(String::from("Green"));
/// ```
///
pub struct Question {
    question: String,
    choices: Vec<String>,
    answer: Option<Answer>,
}

pub struct Answer {
    pub index: usize,
    pub answer: String,
}

impl Question {
    pub fn new(question: String) -> Self {
        Question {
            question,
            choices: Vec::new(),
            answer: None,
        }
    }

    /// Return the question of this instance
    ///
    /// # Examples
    ///
    /// ```
    /// let mut question = Question::new(String::from("choose a color: ");
    /// println!("{}", question.question());
    /// ```
    pub fn question(&self) -> &String {
        &self.question
    }

    /// !WARNING This method is used only on prompt::Prompt
    /// Because only prompt::Prompt must use it the return
    /// Answer is private
    pub fn answer_mut(&mut self) -> &mut Option<Answer> {
        &mut self.answer
    }

    /// Return the index of choices
    ///
    /// # Examples
    /// ```
    /// let mut list_questions: Vec<Question> = Vec::new();
    /// let mut question = Question::new(String::from("choose a color: ");
    /// question.add_choice(String::from("Red"));
    /// question.add_choice(String::from("Blue"));
    /// question.add_choice(String::from("Green"));
    /// list_questions.push(question);
    /// prompt.exec(&mut list_questions).unwrap();
    /// let index_answer = list_questions.get(0).unwrap().question.index_answer().unwrap();
    /// println!("Index chosen by the user: {}", index_answer);
    /// ```
    pub fn index_answer(&self) -> Option<usize> {
        if let Some(answer) = &self.answer {
            return Some(answer.index);
        }
        return None;
    }

    /// Return the string of the chosen choice.
    ///
    /// # Examples
    /// ```
    /// let mut list_questions: Vec<Question> = Vec::new();
    /// let mut question = Question::new(String::from("choose a color: ");
    /// question.add_choice(String::from("Red"));
    /// question.add_choice(String::from("Blue"));
    /// question.add_choice(String::from("Green"));
    /// list_questions.push(question);
    /// prompt.exec(&mut list_questions).unwrap();
    /// let string_answer = list_questions.get(0).unwrap().question.string_answer().unwrap();
    /// println!("choice from user: {}", string_answer);
    /// ```
    pub fn string_answer(&self) -> Option<&String> {
        if let Some(answer) = &self.answer {
            return Some(&answer.answer);
        }
        return None;
    }

    /// Add a choice to this question
    ///
    /// # Examples
    ///
    /// ```
    /// let mut question = Question::new(String::from("choose a color: ");
    /// question.add_choice(String::from("Red"));
    /// question.add_choice(String::from("Blue"));
    /// question.add_choice(String::from("Green"));
    /// ```
    pub fn add_choice(&mut self, choice: String) {
        self.choices.push(choice);
    }

    /// Add N choices to this question
    ///
    /// # Examples
    ///
    /// ```
    /// let mut question = Question::new(String::from("choose a color: ");
    ///    let choices: Vec<String> = vec![
    /// String::from("Red"),
    /// String::from("Blue"),
    /// String::from("Green"),
    /// ];
    /// question.add_choices(choices);
    /// ```

    pub fn add_choices(&mut self, choices: Vec<String>) {
        self.choices.extend(choices);
    }

    /// Return a reference to vec choices of this question
    pub fn choices(&self) -> &Vec<String> {
        &self.choices
    }
}
