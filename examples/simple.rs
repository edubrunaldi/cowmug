extern crate cow_mug;
use cow_mug::Prompt;
use cow_mug::Question;
fn main() {
    let mut list_questions: Vec<Question> = Vec::new();
    let mut pizza_size = Question::new(String::from("Choose the size of your pizza: "));
    pizza_size.add_choice(String::from("Large"));
    pizza_size.add_choice(String::from("Medium"));
    pizza_size.add_choice(String::from("Small"));
    list_questions.push(pizza_size);

    let mut pizza_flavor = Question::new(String::from("Choose pizza:"));
    pizza_flavor.add_choice(String::from("peperoni"));
    pizza_flavor.add_choice(String::from("4 cheese"));
    pizza_flavor.add_choice(String::from("chicken"));
    list_questions.push(pizza_flavor);

    let mut prompt = Prompt::new();
    prompt.exec(&mut list_questions).unwrap();

    let answer_size = list_questions.get(0).unwrap().answer_string().unwrap();
    let answer_flavor = list_questions.get(1).unwrap().answer_string().unwrap();
    let mut choices_is_correct = Question::new(format!(
        "You choose pizza {} with {}, you confirm?",
        answer_size, answer_flavor
    ));
    choices_is_correct.add_choice(String::from("yes"));
    choices_is_correct.add_choice(String::from("no"));
    list_questions.clear();
    list_questions.push(choices_is_correct);
    prompt.exec(&mut list_questions).unwrap();

    if let Some(is_correct) = list_questions.get(0).unwrap().answer_string() {
        if is_correct.as_str() == "yes" {
            println!("Great! See ya");
        } else {
            println!("Thats is terrible!! =/ ");
        }
    }

    return;
}
