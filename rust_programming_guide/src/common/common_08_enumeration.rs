// 열거형(enumeration)은 여러 가지 값(배리언트)을 하나의 타입으로 정의할 수 있게 해주는 기능입니다.
// 열거형을 사용하면 관련된 값을 그룹화하여 코드의 가독성을 높일 수 있습니다.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum IpAddrKind {
    V4, // 배리언트(variant)
    V6, // 배리언트
}

// 열거형을 사용하여 표현하는 구조체를 정의할 수 있습니다.
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 열거형의 배리언트는 데이터를 포함할 수도 있습니다.
#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,                       // 데이터를 포함하지 않는 배리언트
    Move { x: i32, y: i32 },    // 익명 구조체
    Write(String),              // 튜플 구조체
    ChangeColor(i32, i32, i32), // 튜플 구조체
}

// 열거형은 구조체와 마찬가지로 메서드를 정의할 수 있습니다. 열거형에 메서드를 정의하려면 impl 블록을 사용합니다.
impl Message {
    fn call(&self) {
        // 메서드 구현
        println!("Message called: {:#?}", self);
    }
}

pub fn run() {
    let home = IpAddr {
        // 열거형의 값은 :: 연산자를 사용하여 열거형의 이름과 함께 지정할 수 있습니다.
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!(
        "home: {:#?}, loopback.address: {}, loopback.kind: {:?}",
        home, loopback.address, loopback.kind
    );

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, Rust!"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    println!("msg1: {:#?}", msg1);
    println!("msg2: {:#?}", msg2);
    println!("msg3: {:#?}", msg3);
    println!("msg4: {:#?}", msg4);

    let m = Message::Write(String::from("Hello, Rust!"));
    m.call();

    // Option<T> 열거형은 값이 있을 수도 있고 없을 수도 있는 상황을 표현하는 데 사용됩니다. Option<T>는 Some(T)와 None이라는 두 가지 배리언트를 가집니다.
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_string: {:#?}", some_string);
    println!("plus_one(some_number): {:#?}", plus_one(some_number)); // Some(6)이 반환됩니다.
    println!("plus_one(absent_number): {:#?}", plus_one(absent_number)); // None이 반환됩니다.

    // if let 구문을 사용하여 Option<T>의 값을 처리할 수 있습니다. if let은 패턴 매칭을 간단하게 표현할 수 있게 해줍니다.
    if let Some(i) = some_number {
        println!("some_number contains: {}", i);
    };
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match 표현식은 None을 커버하지 않으면 컴파일 에러가 발생합니다. match 표현식은 모든 가능한 값을 다루어야 하기 때문입니다.
    match x {
        None => None, // 값이 없는 경우 None을 반환
        Some(i) => Some(i + 1), // 값이 있는 경우 i에 1을 더한 값을 Some으로 감싸서 반환
                       // other => {
                       // other(다른 표현도 가능 abs, any 등)로 표현할 수도 있습니다. 포괄 패턴은 _와 다르게 other 변수에 값을 저장할 수 있습니다.
                       // 그래서 Option 변수를 다시 처리해줘야 합니다.
                       //     match other {
                       //         None => None,
                       //         Some(i) => Some(i + 1),
                       //     }
                       // }
                       // _ => None, // 와일드카드 패턴을 사용하여 모든 나머지 경우를 처리할 수도 있습니다.
    }
}
