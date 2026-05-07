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

pub fn run() {
    let home = IpAddr {
        // 열거형의 값은 :: 연산자를 사용하여 열거형의 이름과 함께 지정할 수 있습니다.  ㅇ ㅇ
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
}
