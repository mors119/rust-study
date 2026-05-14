fn another_function(x: i32, c: char) {
    // 인수(매개변수) x를 받는 함수입니다.
    // 인수에 타입은 반드시 명시해야 합니다.
    println!(
        "This is another function with value: {} and character: {}",
        x, c
    );
}

pub fn run() {
    println!("This is a function.");
    another_function(5, 'A');

    // let _x = y = 10; // 오류
    // 대신 블록을 사용하여 값을 계산할 수 있습니다.
    let _x = {
        let y = 10;
        y + 5
    };

    let rs = plus_one(five());
    println!("The result of plus_one(five()) is: {}", rs);
}

// 반환 값 타입은 -> 화살표 뒤에 명시합니다.
fn five() -> i32 {
    5 // 표현식이므로 세미콜론이 없습니다.
    // 표현식 끝에 세미콜론이 없으면 해당 표현식의 결과가 반환됩니다.
}

fn plus_one(x: i32) -> i32 {
    x + 1 // x에 1을 더한 값을 반환합니다.
}
