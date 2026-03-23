// 모든 변수/상수의 타입이 컴파일 시점에 결정

/* const는 let과 다르게 런타입 전에 확정되는 메모리에 고정되는 불변의 값이다. */
const MAX: u32 = 1000;

/*
let 변수명 : 타입 = 값;
let 변수명 : 타입;
 - 이때 초기값이 없다면 타입을 생략할 수 없다.
 - let 변수는 읽기 전용이다
 - 쓰기가 가능하려면 let mut로 변수를 선언해야한다.
*/

pub fn run() {
    // let mut x:i32 = 0;
    let mut x = 0; // i32
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    println!("MAX = {MAX}");

    // let y = "2".parse().expect("not a number"); // u32인지 i32인지 알수 없어 오류가 발생한다.
    let y = "2".parse::<u32>().expect("not a number"); // u32로 지정
    println!("y = {y}");
}
