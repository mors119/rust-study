pub fn run() {
    {
        // 블록을 사용하여 변수의 유효 범위를 제한할 수 있습니다.
        // 소유권 (ownership)
        let s1 = String::from("hello"); // s1이 String의 소유자가 됩니다. String은 힙에 데이터를 저장하는 타입입니다.
        let s2 = s1; // s1의 값이 s2로 이동(move)합니다. 이제 s1은 더 이상 유효하지 않습니다.
        // 힙 영역에 저장되는 데이터의 경우 '중복 해제'를 방지하기 위해 이동(move) 개념이 적용됩니다. s1이 s2로 이동하면서 s1은 더 이상 유효하지 않게 됩니다.
        let s3 = s2.clone(); // s2의 데이터를 복제(clone)하여 s3에 저장합니다. 이제 s2와 s3 모두 "hello"를 소유하게 됩니다.

        // println!("{}", s1); // 오류: s1은 더 이상 유효하지 않습니다.
        println!("{}", s2);
        println!("{}", s3);

        let x = 5; // x는 i32 타입의 정수입니다. 스택 영역에 저장됩니다.
        let y = x; // x의 값이 y로 복사(copy)됩니다. i32는 Copy 트레이트를 구현하므로 이동(move)이 아니라 복사(copy)가 발생합니다.

        println!("x = {}, y = {}", x, y); // x와 y 모두 5를 출력합니다.
        // 이 이후부터는 블록 내부의 변수들이 유효하지 않게 됩니다. 메모리 해제는 Rust가 자동으로 처리합니다.
    }

    // 소유권 이동
    let s = String::from("hello"); // s가 String의 소유자가 됩니다.
    takes_ownership(s); // s의 값이 함수로 이동(move)합니다. 이제 s는 더 이상 유효하지 않습니다.

    // 소유권 재반환
    let s = String::from("world");
    let s2 = gives_ownership(s); // s의 값이 gives_ownership 함수로 이동(move)합니다. 함수가 반환한 String의 소유권이 s2로 이동(move)합니다.
    println!("world: {}", s2); // s2는 "world"를 출력합니다.

    // 매개변수와 소유권 동시 반환
    let (s3, len) = calculate_length(s2); // s2의 값이 calculate_length 함수로 이동(move)합니다. 함수가 반환한 튜플의 첫 번째 요소인 String의 소유권이 s3로 이동(move)하고, 두 번째 요소인 usize는 len에 저장됩니다.
    println!("The length of '{}' is {}.", s3, len); // s3는 "world"를 출력하고, len은 5를 출력합니다.

    // 소유권과 참조자 (references) -> 대여(borrowing)
    let s4 = String::from("hello");
    let _len = calculate_length_ref(&s4); // s4의 참조자(&s4)를 calculate_length_ref 함수로 전달합니다. 참조자는 소유권을 이동(move)하지 않고, 함수가 s4를 읽기만 할 수 있도록 허용합니다.

    // 가변 참조자 (mutable references)
    let mut s5 = String::from("hello");
    change(&mut s5); // s5의 가변 참조자(&mut s5)를 change 함수로 전달합니다. 가변 참조자는 소유권을 이동(move)하지 않고, 함수가 s5를 수정할 수 있도록 허용합니다.
    println!("s5 after change: {}", s5); // s5는 "hello, world"를 출력합니다

    // 참조자 규칙
    // 1. 한 번에 '하나'의 '가변 참조자'만 허용됩니다. (데이터 경합 방지)
    // 2. 불변 참조자와 가변 참자가 동시에 허용되지 않습니다. (데이터 경합 방지)
    // 3. 참조자는 항상 유효해야 합니다.
    let r1 = &s5; // 불변 참조자 r1이 s5를 참조합니다.
    let r2 = &s5; // 불변 참조자 r2가 s5를 참조합니다.
    // let r3 = &mut s5; // 오류: 불변 참자 r1과 r2가 존재하는 동안 가변 참자 r3를 만들 수 없습니다.
    println!("r1: {}, r2: {}", r1, r2); // r1과 r2는 모두 "hello, world"를 출력합니다.
    let r3 = &mut s5; // r1과 r2가 더 이상 사용되지 않으므로, 이제 가변 참자 r3를 만들 수 있습니다.
    println!("r3: {}", r3); // r3는 "hello, world"를 출력합니다.

    // 댕글링 참조자 (dangling references)
    // Rust는 댕글링 참조자를 생성하는 것을 컴파일 시점에서 방지합니다.
    // let _reference_to_nothing = dangle(); // 오류: dangle 함수는 댕글링 참조자를 반환하려고 시도합니다.

    // 참조자와 슬라이스
    // 슬라이스는 일종의 참조자로 소유권을 가지지 않고 데이터의 일부분을 참조하는 타입입니다.
    let s = String::from("hello world");
    let hello = &s[0..5]; // s의 첫 5글자를 참조하는 슬라이스입니다.
    let world = first_word(&s); // s의 6번째부터 11번째까지의 글자를 참조하는 슬라이스입니다.
    println!("hello: {}, world: {}", hello, world);
}

// 소유권 이동 예시
fn takes_ownership(some_string: String) {
    // some_string이 함수의 매개변수로 이동(move)합니다. 이제 some_string이 String의 소유자가 됩니다.
    println!("{}", some_string);
    // 함수가 끝나면 some_string이 유효 범위를 벗어나고 메모리가 해제됩니다.
}

// 소유권 재반환 예시
// gives_ownership 함수는 String을 반환합니다. 반환된 String의 소유권이 호출한 곳으로 이동(move)합니다.
fn gives_ownership(some_string: String) -> String {
    some_string
}

// 매개변수와 소유권 동시 반환 예시
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // s의 길이를 계산합니다.
    (s, length) // s와 length를 튜플로 반환합니다. s의 소유권이 호출한 곳으로 이동(move)합니다.
}

// 참조자 예시
fn calculate_length_ref(s: &String) -> usize {
    s.len() // s는 참조자이므로 소유권이 이동하지 않습니다. s의 길이를 반환합니다.
}

// 가변 참조자 예시
fn change(some_string: &mut String) {
    some_string.push_str(", world"); // some_string은 가변 참조자이므로 소유권이 이동하지 않습니다. some_string을 수정합니다.
}

// 댕글링 참조자 예시
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 오류: 소유권 이동이 일어나지 않으므로 s가 함수가 끝나면 유효 범위를 벗어나고 메모리가 해제됩니다. 댕글링 참조자가 됩니다.
// }

// 참조자와 슬라이스 예시
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // 문자열을 바이트 배열로 변환합니다.

    for (i, &item) in bytes.iter().enumerate() {
        // iter는 순회를, enumerate는 inter의 결과를 튜플(인덱스, 참조자)로 리턴합니다.
        if item == b' ' {
            // 공백 문자를 찾습니다.
            return &s[0..i]; // 공백이 발견되면 문자열의 처음부터 공백까지의 슬라이스를 반환합니다.
        }
    }

    &s[..] // 공백이 없는 경우 문자열 전체를 슬라이스로 반환합니다.
}
