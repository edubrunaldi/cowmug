# CowMug

![License]

`cowmod` provide an interactive command line user interface to ask
questions to user.
## !!!! It's on early development and it isn't considered stable yet !!!!


## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
cowmug = "0.0.1"
```

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
  ([LICENSE-APACHE](https://github.com/rust-lang/libc/blob/master/LICENSE-APACHE))

* [MIT License](https://opensource.org/licenses/MIT)
  ([LICENSE-MIT](https://github.com/rust-lang/libc/blob/master/LICENSE-MIT))

at your option.

## Example
this example can be find at examples/sample.rs

```rust
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

    let mut pizza_flavor = Question::new(String::from("Choose the flavor:"));
    let pizza_flavor_choices: Vec<String> = vec![
        String::from("Pepperoni"),
        String::from("4 cheeses"),
        String::from("Chicken"),
    ];
    pizza_flavor.add_choices(pizza_flavor_choices);
    list_questions.push(pizza_flavor);

    let mut prompt = Prompt::new();
    prompt.exec(&mut list_questions).unwrap();

    let answer_size = list_questions.get(0).unwrap().string_answer().unwrap();
    let answer_flavor = list_questions.get(1).unwrap().string_answer().unwrap();
    let mut choices_is_correct = Question::new(format!(
        "You choose pizza {} with {}, you confirm it?",
        answer_size, answer_flavor
    ));
    choices_is_correct.add_choice(String::from("yes"));
    choices_is_correct.add_choice(String::from("no"));
    list_questions.clear();
    list_questions.push(choices_is_correct);
    prompt.exec(&mut list_questions).unwrap();

    if let Some(is_correct) = list_questions.get(0).unwrap().string_answer() {
        if is_correct.as_str() == "yes" {
            println!("Great! See ya");
        } else {
            println!("Thats is terrible!! =/ ");
        }
    }
}

```

[License]: https://img.shields.io/crates/l/libc.svg
