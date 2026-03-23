/*
fn 함수이름(파라미터1 이름: 파라미터1 타입, 파라미터2 이름: 파라미터2 타입) -> 리턴 타입 { ... }
*/

pub fn run() {
    let a = add1(12, 13);
    let b = add2(12, 13);
    println!("a={}, b={}", a, b);
}

// statement 문
fn add1(a: i32, b: i32) -> i32 {
    return a + b;
}

// expression 식
fn add2(a: i32, b: i32) -> i32 {
    // 리턴값을 위와 같이 return 키워드를 사용하지 않고, 함수의 맨 끝에 세미콜론 ; 없이 값이나 변수명을 적어 놓으면, 그 값이 리턴된다.
    a + b
}
