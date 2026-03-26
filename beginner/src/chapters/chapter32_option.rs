use std::collections::HashMap;
/*
Rust에서 가장 많이 사용되는 열거형은 'Option'과 'Result'다.
* Option은 어떤 값을 리턴할 때 "값이 없을 수도 있을 때"를 위해 None이라는 타입이 있고,
* Result는 어떤 값을 리턴할 때 "에러가 있는 경우"를 위해 Err이라는 타입이 준비되어 있다.
*/

/*
enum Option<T> {
    Some(T),
    None, // 다른 언어에서는 Null이나 -1로 값이 없음을 나타낸다. Rust는 Option::None으로 나타낸다.
}
*/

pub fn run() {
    test1();
}

fn test1() {
    let map = HashMap::from([("Jeff", 80), ("Alice", 100)]);

    let name = "Jeff";
    let _point1 = map.get(name).unwrap(); // unwrap은 값이 없는 경우 panic 발생
    let _print2 = map.get(name).expect("Failed"); // panic + "test" 메시지 출력

    // 값이 없을 수도 있는 경우는 None을 처리해주는 코드를 써야한다.
    match map.get(name) {
        Some(point) => println!("{}'s point = {}", name, point),
        _ => println!("There is no name of {}", name),
    }
    // match 대신 None 지정을 하지 않고 if let을 쓰기도 한다.
    if let Some(point) = map.get(name) {
        println!("{}'s point = {}", name, point);
    } else {
        println!("There is no name of {}", name);
    }
}
