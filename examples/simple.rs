extern crate cow_mug;
use cow_mug::Prompt;

fn main() {
    let mut prompt = Prompt::default();
    prompt
        .write_question("Pergunta???".to_string())
        .expect("erro ao sair do wait question");
}
