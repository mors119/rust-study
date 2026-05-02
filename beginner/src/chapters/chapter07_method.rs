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
```rust
impl MyStruct {
    fn method(self) { ... }
}
```
내부적으로 아래와 같다.
```rust
fn method(self: MyStruct)
```

fn f1() self가 없는 경우 스태틱 메서드, 특정인스턴스에 구애받지 않기 때문에 f1:: 이렇게 호출 가능
fn f2(self): 메소드가 끝나면 더는 사용하지 않는 함수. 소유권이 이동됨. 체이닝 시에 사용
fn f3(mut self)
fn f4(&self): 소유권을 빌렸다가 원본에 다시 돌려줌
fn f5(&mut self)
*/

// ::  → 타입 기준 접근 (static 느낌), new는 self 없음 → 인스턴스 필요 없음
pub fn run() {
    let p1 = Point::new(0, 0); // 3. 연관함수의 사용  
    let p2 = Point::new(3, 4);
    assert_eq!(5.0, p1.distance(&p2)); // 4. 메서드의 사용 (값이 일치하는지 확인)
}
