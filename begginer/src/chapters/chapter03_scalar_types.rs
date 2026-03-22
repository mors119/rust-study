// 스칼라 타입

pub fn run() {
    // let d:u32 = 2 - 100; // attempt to compute `2_u32 - 100_u32`, which would overflow 오류 발생
    integer();
    floating_point();
}

fn integer() {
    //1. 가독성을 위해 숫자 사이에 _ 사용 가능
    let a1 = 100_000; // 100000

    //2. 16진수
    let b1 = 0xff;

    //3. 8진수
    let c1 = 0o77;

    //4. 이진수
    let d1 = 0b1111_0000;

    //5. 문자 아스키 값
    let e1 = b'A'; //e1:u8, b'A'는 u8 타입의 정수로 자동 타입 지정됨.

    println!("{}, {}, {}, {}, {}", a1, b1, c1, d1, e1)
}

fn floating_point() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let u: u32 = 40000;
    let sqrt_u = (u as f64).sqrt(); // 제곱근을 구할 때는 f타입으로 변경 후 계산해야한다.
    println!("f64={}, f32={}, sqrt(u) = {}", x, y, sqrt_u);
}

fn _boolean() {
    let _t1 = true;
    let _t2: bool = false;
}

fn _character() {
    let _a = 'a';
    let _z: char = 'z';
}
