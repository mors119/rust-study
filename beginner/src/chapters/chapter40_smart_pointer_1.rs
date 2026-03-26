use std::ops::Deref;

/*
스마트 포인터(smart pointer)란?

- 어떤 값을 "가리키는 타입"이지만, 단순 주소 저장을 넘어서
  추가 동작까지 함께 가진 타입입니다.
- 대표적으로 Deref, Drop 같은 trait과 연결되어 동작합니다.

핵심 개념
1. 일반 참조 &T
   - 값을 빌려서 가리키는 "참조"
   - 소유권은 없음

2. 스마트 포인터
   - 값을 가리키면서 추가 기능도 제공
   - 예: Box<T>, Rc<T>, Arc<T>, RefCell<T>

3. Deref
   - *x 를 했을 때 어떤 값을 꺼낼지 정의
   - 즉, "스마트 포인터를 일반 참조처럼 다루게" 해줌

4. Drop
   - 값이 스코프를 벗어날 때 자동 호출
   - 정리(cleanup) 작업 수행
   - Box 같은 타입은 보통 힙 메모리 정리에 사용됨

5. Deref coercion
   - Deref 덕분에 &String -> &str 같은 자동 변환이 일어남
*/

/// 우리가 직접 만들어보는 간단한 스마트 포인터
/// 내부에 T를 저장하는 튜플 구조체
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    /// 새로운 MyBox 생성
    fn new(value: T) -> Self {
        MyBox(value)
    }
}

/*
Deref 구현

- type Target = T
  : 이 스마트 포인터가 최종적으로 가리키는 실제 데이터 타입은 T라는 뜻
- deref(&self) -> &T
  : 자기 자신을 참조로 받아서, 내부 데이터에 대한 참조를 돌려줌
*/
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // self.0 = 내부 값
        // &self.0 = 내부 값에 대한 참조
        &self.0
    }
}

/*
Drop 구현

- 값이 스코프를 벗어날 때 자동 호출됨
- 보통은 사용자가 직접 호출하지 않고 Rust가 알아서 실행함
- 여기서는 "자동 정리"가 언제 일어나는지 보기 위해 메시지만 출력
*/
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("MyBox is dropped");
    }
}

/// &str를 요구하는 함수
/// String이 아니라 "문자열 슬라이스 참조"를 받음
fn print_str(s: &str) {
    println!("print_str: {}", s);
}

pub fn run() {
    println!("--- 1. 일반 값과 스마트 포인터 ---");

    let x = 10;
    let y = MyBox::new(x);

    println!("x = {}", x);

    /*
    *y가 되는 이유

    - y는 MyBox<i32>
    - MyBox<T>가 Deref를 구현했으므로
      *y 는 내부적으로 대략 *(y.deref()) 처럼 동작
    - y.deref() -> &i32
    - *(&i32) -> i32 값 접근
    */
    println!("*y = {}", *y);

    println!("\n--- 2. 일반 참조와 비슷하게 동작 ---");

    let n = 42;
    let r = &n; // 일반 참조
    println!("r = {}", r); // 참조도 출력 가능
    println!("*r = {}", *r); // 역참조로 실제 값 접근

    /*
    스마트 포인터도 Deref 덕분에 비슷한 느낌으로 사용 가능
    */
    let boxed = MyBox::new(99);
    println!("boxed = {:?}", boxed);
    println!("*boxed = {}", *boxed);

    println!("\n--- 3. Deref coercion (자동 참조 변환) ---");

    let s = String::from("hello smart pointer");

    /*
    print_str는 &str를 원함
    그런데 여기서 &s는 &String

    그래도 되는 이유:
    String이 Deref<Target = str> 를 구현했기 때문에
    Rust가 자동으로 &String -> &str 변환을 해줌
    */
    print_str(&s);

    println!("\n--- 4. 우리가 만든 MyBox도 Deref 가능 ---");

    let my_string_box = MyBox::new(String::from("from MyBox"));

    /*
    여기서 *my_string_box 는 String 자체를 꺼내는 느낌으로 보이지만,
    실제로는 deref를 통해 내부 String에 접근하게 됨.

    다만 String은 move가 일어날 수 있는 타입이라,
    보통은 아래처럼 참조를 더 많이 활용함.
    */
    println!("inside MyBox<String> = {}", &*my_string_box);

    /*
    &*my_string_box 해석:
    - *my_string_box : Deref로 내부 String 접근
    - &(...): 다시 참조를 붙여 &String 형태로 사용
    */

    println!("\n--- 5. Box<T>도 스마트 포인터 ---");

    let std_box = Box::new(1234);
    println!("*std_box = {}", *std_box);

    /*
    Box<T>는 표준 라이브러리의 대표 스마트 포인터
    - 값을 힙(heap)에 저장
    - Deref 구현
    - Drop 시 자동 정리
    */

    println!("\n--- 6. 스코프 종료 시 Drop 자동 호출 확인 ---");

    {
        let _temp1 = MyBox::new(1);
        let _temp2 = MyBox::new(2);
        println!("inner scope end soon");
    } // 여기서 _temp2, _temp1 순서로 drop 호출

    println!("after inner scope");

    println!("\n--- 핵심 정리 ---");
    println!("1) 스마트 포인터는 값을 가리키는 타입 + 추가 동작");
    println!("2) Deref는 * 연산 동작을 정의");
    println!("3) Drop은 스코프 종료 시 자동 정리");
    println!("4) Deref coercion 덕분에 &String -> &str 자동 변환 가능");
}
