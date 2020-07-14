extern crate cow_mug;
use cow_mug::Prompt;
use cow_mug::Question;

fn main() {
    let mut v: Vec<Question> = Vec::new();
    let mut q = Question::new();
    q.set_question(String::from("Is this working?"));

    v.push(q);

    let mut prompt = Prompt::new();
    prompt
        .exec(&mut v)
        .ok()
        .expect("Error While using prompt.exec");

    for i in v {
        println!(
            "question: {} \n answer: {:?}",
            i.get_question(),
            i.get_answer()
        );
    }
    return;
}
