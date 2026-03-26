/*
라이프타임 = "참조가 유효한 범위" 즉, 리턴 타입이 참조값일 때 사용된다. (&str일 때 특히 중요)
라이프타임 파라미터는 일종의 '제네릭'이다.

첫 번째 변수는 'a 두 번째는 'b와 같이 표기
  i32 라이프타임 표기 예시
  &'a i32
  &'a mut i32

오류 상황
  fn main() {
      let s1 = String::from("hello");
      let result;

      {
          let s2 = String::from("hi");
          result = longest(&s1, &s2);
      }

      println!("{}", result); // ❌ result가 s2를 가리킬 수도 있으므로 에러
  }

  fn longest<'a>(s1:&'a str, s2:&'a str) -> &'a str {
      if s1.len() > s2.len() { s1 }
      else { s2 }
  }
  참조가 여러 개라면 -> 하나의 라이프타임을 따라가라고 명시

* 생략 규칙
1. 입력 참조마다 각각 다른 라이프타임
  fn foo(x: &str, y: &str)
  이렇게 값이 들어오면, 컴파일러는
  fn foo<'a, 'b>(x: &'a str, y: &'b str)
  각각 따로 존재한다고 본다.

2. 입력이 1개면, 출력은 그걸 따라감
  fn first(s: &str) -> &str
  라면 출력은
  fn first<'a>(s: &'a str) -> &'a str

3. &self가 있으면, 출력은 self를 따라감
  struct User {
      name: String,
  }
  impl User {
      fn get_name(&self) -> &str {
          &self.name
      }
  }
  일 때,
  fn get_name<'a>(&'a self) -> &'a str

*/

pub fn run() {
    let _g1 = good1();
    let _g2 = good2();
    main();
}

// fn bad() -> &str {
//     let s = String::from("hello");
//     &s // s는 함수가 끝나면 죽음
// }

// 해결 1. 소유권 반환
fn good1() -> String {
    String::from("hello")
}

// 해결 2: static (문자열 리터럴은 프로그램 끝까지 살아있음)
fn good2() -> &'static str {
    "hello"
}

// 구조체 (구조체의 구성원의 수명은 구조체보다 짧아서는 안된다.)
struct Point<'a> {
    x: &'a i32,
    y: i32,
}

fn main() {
    let x1 = 10;
    let p;
    {
        let y1 = 20;
        // p = Point { x: &x1, y: &y1 }; 이럴 때는 참조 대신 y1의 소유권을 가지고 이동해야한다.
        p = Point { x: &x1, y: y1 };
    }
    println!("point = {}, {}", p.x, p.y);
}
