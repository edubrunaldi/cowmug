extern crate cow_mug;
use cow_mug::Prompt;
use cow_mug::QuestionList;

fn main() {
    let mut v: Vec<QuestionList> = Vec::new();
    let mut q = QuestionList::new();
    *q.question_mut() = String::from("Choose pizza:");
    q.add_choice(String::from("peperoni"));
    q.add_choice(String::from("4 cheese"));
    q.add_choice(String::from("chicken"));

    v.push(q);

    let mut prompt = Prompt::new();
    prompt
        .exec(&mut v)
        .ok()
        .expect("Error While using prompt.exec");

    for i in v {
        if let Some(answer) = i.answer() {
            println!(
                "question: {} \n index answer: {:?}, answer: {:?}",
                i.question(),
                i.answer(),
                i.choices().get(*answer)
            );
        }
    }
    return;
}
