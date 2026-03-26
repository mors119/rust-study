/*
* 제네릭: 구조체, 열거형, 함수, 메서드에서 하나의 코드로 여러 타입을 처리
struct로 같은 이름의 다른 타입을 선언 할 수 없다.

struct Point { x:i32, y:i32 }
대신
struct Point <T> { x:T, y:T }
이런 식으로 사용된다.

 */

pub fn run() {
    let a: i32 = 2;
    let b: i32 = 3;
    let c = min(a, b);
    println!("min={}", c);

    let (d, e) = add_sub(a, b);
    println!("(add,sub)=({},{})", d, e);
}

/*
fn min(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    } else {
        return b;
    }
}
 */
// 위 코드를 아래의 제네릭으로 표현하면 아래와 같다.
// fn min<T>(a:T, b:T) -> T { 바꿀 수 있다. 하지만 제네릭은 연산을 지원하지 않기 때문에 아래와 같이 사용한다.
fn min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a < b {
        return a;
    } else {
        return b;
    }
}

/* 동일한 내용을 where를 사용해서 표현할 수 있다.
fn add_sub(a: i32, b: i32) -> (i32, i32) {
    return (a + b, a - b);
}
*/
fn add_sub<T>(a: T, b: T) -> (T, T)
// where T: std::ops::Add + std::ops::Sub, 이렇게는 사용하지 못하는데. Add와 Sub는 Output을 리턴한다.
// 또한 소유권 문제를 피하기 위해 a, b를 복사하는 Copy 트레잇을 바운딩한다.
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    return (a + b, a - b);
}
