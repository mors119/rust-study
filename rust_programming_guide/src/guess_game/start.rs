use rand::RngExt;
use std::cmp::Ordering;
use std::io; // https://doc.rust-lang.org/std/prelude/index.html

pub fn run() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=180);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    // 대화형(interactive)의 경우 read_line을 쓰고
    // 비대화형(CLI)는 let args: Vec<String> = env::args().collect(); 이런 식으로 cargo run 메시지 이런 형태로 쓴다.
    io::stdin() // 표준 입력 핸들
        .read_line(&mut guess)
        .expect("Failed to read line"); // Result 처리

    let guess: u32 = guess.trim().parse().expect("Pleas type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
