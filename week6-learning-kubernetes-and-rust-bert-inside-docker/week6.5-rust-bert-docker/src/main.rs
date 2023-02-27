use std::io;

fn main() {
    let mut user_input = String::new();

    println!("Welcome to the Rust chatbot!");
    println!("Type q or quit to (q)uit the program");
    loop {
        println!("You: ");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Cannot read user input");

        user_input = user_input.trim().to_string();
        if user_input.eq("q") || user_input.eq("quit") {
            break;
        }

        println!("Chatbot: {}", dialogue::dialogue(user_input.clone()));
        println!("");
        user_input.clear();
    }
}