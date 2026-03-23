// 메소드

// struct: 실제 메모리에 존재하고 생성도 가능한
// typescript interface같은 느낌
struct Point {
    x: i32,
    y: i32,
}

// impl: Point 타입에 기능을 추가하는 느낌으로 씀.
// enum이나 struct 등 직접 정의한 타입에만 impl 가능
impl Point {
    fn new(x: i32, y: i32) -> Point {
        // 1. 연관 함수
        Point { x: x, y: y } // Point {x, y}로도 사용 가능 (필드 이름과 변수 이름이 같으면 자동으로 매핑)
    }
    // 이름을 바꿔보면 아래와 같음.
    // fn new(xt: i32, yt: i32) -> Point {
    //   Point { x: xt, y: yt } // struct필드:함수 매개변수
    // }

    fn distance(&self, p: &Point) -> f64 {
        // 2. 메서드
        (((p.x - self.x).pow(2) + (p.y - self.y).pow(2)) as f64).sqrt() // sqrt는 float만 가능하기 때문에 as f64
    }
}
/*
&self란?
self는 해당 메서드를 호출한 인스턴스를 의미하고
&를 붙였기에 해당 인스턴스를 레퍼런스(주소값) 형태로 참조한다.
*/

// ::  → 타입 기준 접근 (static 느낌), new는 self 없음 → 인스턴스 필요 없음
pub fn run() {
    let p1 = Point::new(0, 0); // 3. 연관함수의 사용  
    let p2 = Point::new(3, 4);
    assert_eq!(5.0, p1.distance(&p2)); // 4. 메서드의 사용 (값이 일치하는지 확인)
}
