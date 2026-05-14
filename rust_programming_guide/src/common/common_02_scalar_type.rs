pub fn run() {
    // 정수형 (integer)
    let _x: i8 = -128; // 8비트 정수
    let _x: i32 = 42; // 32비트 정수
    // 부호 없는 정수형 (unsigned integer)
    let _y: u16 = 65535; // 16비트 부호 없는 정수
    let _y: u64 = 100; // 64비트 부호 없는 정수

    // 정수형 리터럴
    let _z = 123; // 기본적으로 i32로 추론
    let _z = 0b1010; // 2진수 리터럴 (10)
    let _z = 0o77; // 8진수 리터럴 (63)
    let _z = 0xff; // 16진수 리터럴 (255)
    let _z = b'A'; // 바이트 리터럴 (65)
    let _z = 1_000_000; // 가독성을 위한 밑줄 사용 (1000000)

    // 부동 소수점형 (floating-point)
    let _f1: f32 = 3.14; // 32비트 부동 소수점
    let _f2: f64 = 2.71828; // 64비트 부동 소수점

    // 불리언형 (boolean)
    let _is_active: bool = true;
    let _is_inactive: bool = false;

    // 문자형 (character)
    let _c: char = 'A'; // 유니코드 문자 (4바이트)

    println!("All types have been defined successfully.");
}
