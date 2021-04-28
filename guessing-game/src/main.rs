use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 10);

    println!("the secret number is {}", secret_number);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("womp");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("that's not a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("the same");
                break;
            }
        }
    }
}
