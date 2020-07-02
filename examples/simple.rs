extern crate cow_mug;
use cow_mug::Prompt;

fn main() {
    let prompt = Prompt::default();
    prompt
        .wait_question()
        .expect("erro ao sair do wait question");
}
