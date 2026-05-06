use rand::RngExt;
use std::cmp::Ordering; // compare 함수 라이브러리 (Ordering은 작다 같다 크다 비교 가능)
use std::io; // https://doc.rust-lang.org/std/prelude/index.html

pub fn run() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=180);

    // println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // 대화형(interactive)의 경우 read_line을 쓰고
        // 비대화형(CLI)는 let args: Vec<String> = env::args().collect(); 이런 식으로 cargo run 메시지 이런 형태로 쓴다.
        io::stdin() // 표준 입력 핸들
            .read_line(&mut guess)
            .expect("Failed to read line"); // Result 처리

        // 섀도잉: 러스트는 같은 이름으로 새로운 값을 가리키는 것을 허용함.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // 형 변환

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
