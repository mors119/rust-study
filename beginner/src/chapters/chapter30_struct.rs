use serde::Serialize;
use std::mem;

pub fn run() {
    field1();
    // * 필드값을 편하게 지정하기
    let s1 = make_person("Jeff".to_owned(), 80);
    println!("name={}, point={}", s1.name, s1.point);

    let s2: Person = Default::default();
    // name = "" (default for String)
    // point = 0 (default for i32)
    println!("Default config: name='{}', point={}", s2.name, s2.point);

    // * tuple
    tuple_struct();

    let p1 = Point::new(0, 0);
    let p2 = Point::new(3, 4); // 연관함수 사용 ::
    let p3 = Point::new(-3, 5);
    assert_eq!(5.0, p1.distance(&p2)); // 메서드 사용

    let max = max_distance(&p1, &p2, &p3);
    println!("max distance = {}", max); //6.08276253029821

    // * 비어있는 구조체
    // 크기 0, 값은 있지만 데이터 없음. 여러 개만들어도 메모리 부담이 없음.
    let a = Foo;
    let _b = Foo;
    println!("{:?}", a); // Foo
    println!("{}", mem::size_of::<Foo>()); // 0
}

// * 정의하기
// 필드들은 <필드명> : <타입> 형태이고, 필드와 필드 사이는 콤마(,)로 구분, { }안에 담는다. 블록 기호 다음에는 ;가 없다.
#[derive(Debug, Default)] // 
struct Person {
    name: String,
    point: i32,
}

fn field1() {
    // 주의할 것은 mut에 의해서 구조체의 모든 필드가 쓰기 가능하게 된다
    let mut s1 = Person {
        name: "Jeff".to_owned(),
        point: 80,
    };
    s1.point = 100;
    println!("name={}, point={}", s1.name, s1.point);
}

fn make_person(name: String, point: i32) -> Person {
    // Person { name: name, point: point } 동일
    Person { name, point }
}

// * 튜플 구조체
// 튜플 구조체는 struct <구조체이름> 다음에 바로 튜플 형태의 (i32, i32, i32)를 적으면 된다.
struct _Color(i32, i32, i32);
struct TuPoint(i32, i32);

fn tuple_struct() {
    let p1 = TuPoint(0, 0);
    let p2 = TuPoint(3, 4);
    let dist = cal_distance(&p1, &p2);
    assert_eq!(5.0, dist);
}

fn cal_distance(p1: &TuPoint, p2: &TuPoint) -> f64 {
    (((p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2)) as f64).sqrt()
}

// * 연관함수
struct Point {
    x: i32,
    y: i32,
}

// <구조체 이름>::<함수명>과 같은 형태로 호출
impl Point {
    // 연관 함수
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    // 메서드
    // &self는 호출한 인스턴스의 참조자 변수
    fn distance(&self, p: &Point) -> f64 {
        (((p.x - self.x).pow(2) + (p.y - self.y).pow(2)) as f64).sqrt()
    }
}

fn max_distance(p1: &Point, p2: &Point, p3: &Point) -> f64 {
    // &Point처럼 레퍼런스로 받았고 (*p1).distance라고 하는게 맞지만,
    // object.something() 코드로 메서드를 호출하면 시그니처에 맞춰 자동으로 &, &mut, *를 추가한다.
    let d1 = p1.distance(p2);
    let d2 = p1.distance(p3);
    let d3 = p2.distance(p3);

    let mut max = d1;
    if d2 > max {
        max = d2;
    }
    if d3 > max {
        max = d3;
    }
    return max;
}

/*
* 비어있는 구조체 (Unit-like struct)

→ 필드가 하나도 없는 구조체
→ 크기가 0 (Zero-Sized Type, ZST)
→ "데이터"가 아니라 "의미/타입"을 표현할 때 사용 (ts interface(ts는 데이터 x, rust struct는 데이터 o)와 유사)
*/
#[derive(Debug)]
struct Foo;

// * 사용 시기
// 상태 표현용 (의미있는 타입을 만들 때)
struct _Connected;
struct _Disconnected;

// 타입으로 역할 구분(Marker type, 타입 구분용)
struct _ReadOnly;
struct _ReadWrite;

// Trait 구현용 (상태 없이 기능만 제공)
struct _Logger;

impl _Logger {
    fn _log() {
        println!("log!");
    }
}
// 비어 있는 struct (빈 객체를 표현)
#[derive(Serialize)]
struct _JsonData {}

/*
Debug
→ {:?}로 구조체/열거형 전체 필드 출력 (디버깅용)
#[derive(Debug)]
*/

/*
Default
→ 기본값 생성 (Type::default())
→ 모든 필드도 Default여야 함
#[derive(Default)]
*/

/*
Clone
→ clone()으로 명시적 복사 (힙 데이터 포함)
#[derive(Copy, Clone)]
*/

/*
Copy
→ 대입 시 자동 복사 (move 없음)
→ 모든 필드가 Copy여야 함
→ Clone도 같이 필요
#[derive(Copy, Clone)]
*/

/*
PartialEq
→ ==, != 비교 가능
#[derive(PartialEq)]
*/

/*
Eq
→ 완전한 동등성 비교 (PartialEq 필요)
#[derive(PartialEq, Eq)]
*/

/*
PartialOrd
→ <, >, <=, >= 비교 가능
#[derive(PartialOrd)]
*/

/*
Ord
→ 정렬 가능 (sort 등에서 사용)
→ PartialOrd + Eq 필요
#[derive(PartialOrd, Ord)]
*/

/*
Hash
→ HashMap / HashSet 키로 사용 가능
#[derive(Hash)]
*/

//

/*
자주 쓰는 조합
#[derive(Debug, Clone, PartialEq)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
*/
