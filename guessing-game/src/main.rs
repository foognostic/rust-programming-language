use std::io;

fn main() {
    println!("Please input your guess: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("womp");

    println!("You guessed: {}", guess);
}
