fn print_message(message: String) {
    println!("Message: {message}");
}

fn main() {
    let text = String::from("hello ownership");

    print_message(text);

    // text was moved into print_message, so it cannot be used here anymore.
    // println!("text: {text}")
}
