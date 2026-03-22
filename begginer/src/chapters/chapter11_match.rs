// .. = "중간을 다 포함하거나 생략한다"

pub fn run() {
    //1. 변수 값에 따른 matching
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"), // 지정된 조건이 아닌 모든 것에 대해서는 이렇게 처리
    }

    //2. 변수 범위에 따른 matching
    let age: u32 = 30;
    let group = match age {
        0..=10 => "baby",
        11..=20 => "teen",
        21..=60 => "adult",
        _ => "old",
    }; //let group = ...; 형태의 statement라서 ; 있어야 함  

    //3. 변수의 타입에 따른 matching
    let c = '5';
    let num = match c.to_digit(10) {
        //to_digit는 Option 타입 리턴
        Some(n) => n,
        None => 0,
    };

    println!("{}, {}", num, group);

    //4. 튜플에 대한 matching
    // _는 다른 모든 것의 의미를 가진다.
    let n = 33;
    match (n % 3, n % 5) {
        (0, 0) => println!("3과 5의 배수"),
        (0, _) => println!("3의 배수"),
        (_, 0) => println!("5의 배수"),
        (_, _) => println!("3의 배수도 5의 배수도 아님"),
    }
}
