pub fn run() {
    // 불변성 변수 (immutable)
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // 불변성 에러

    // 가변성 변수 (mutable)
    let mut y = 6;
    println!("The value of x is: {y}");
    y = 3;
    println!("The value of x is: {y}");

    // 상수 (constant)
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // 새도잉 shadowing
    let z = 5;
    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z int the inner scope is: {z}");
    }

    println!("The value of z is: {z}");

    // mut와 shadowing의 차이점은
    // shadowing은 타입의 변경이 가능하다.
    let _spaces = "       "; // 문자
    let _spaces = _spaces.len(); // 숫자
    // mut는 변수의 타입을 변경할 수 없다.
    let mut _spaces2 = "       "; // 문자
    // let _spaces2 = _spaces2.len(); // 숫자 - 타입에러 발생

    // 타입 변경
    let _guess: u32 = "42".parse().expect("Not a number!");
}
