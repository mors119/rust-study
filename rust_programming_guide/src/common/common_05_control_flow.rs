pub fn run() {
    let number = 3;

    // if 표현식
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // if는 표현식이므로 값을 반환할 수 있습니다.
    let condition = true;
    let _number = if condition { 5 } else { 6 }; // rust에서는 3항 연산을 사용할 수 없습니다.
    println!("The value of _number is: {}", _number);

    // 반복문 (loop)
    let mut count = 0;

    let _result = loop {
        count += 1;
        if count * 5 - 3 == 25 {
            break count + 3; // loop 표현식의 결과로 count + 3이 반환됩니다.
        } else if count % 2 == 0 {
            continue; // 현재 반복을 건너뛰고 다음 반복으로 넘어갑니다.
        }
    };

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; // 가장 가까운 (안쪽) 루프를 종료합니다.
            }
            if count == 2 {
                break 'counting_up; // 'counting_up 레이블이 붙은 루프를 종료합니다.
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("Count reached: {}", count);

    // while 반복문
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for 반복문
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
