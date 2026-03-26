/*
Copy 되는 타입
* 모든 숫자 타입
i8, i16, i32, i64, i128
u8, u16, u32, u64, u128
f32, f64
* bool char
bool
char
* 참조타입 (포인터 값만 복사하면 되기 때문에 Copy 가능)
&i32
&str
&T
* 튜플 (안에 있는 모든 요소가 Copy가능 할 때만)
(i32, bool)        // ⭕
(i32, String)      // ❌
* 배열 (조건 있음)
[i32; 3]           // ⭕
[String; 3]        // ❌

* 안되는 대표 타입
String
Vec
Box, Rc, Arc

! 변수를 재할당할 때 기본적으로 "스택만 사용하는 타입은 copy, 힙을 사용하는 타입은 move"
*/

pub fn run() {
    // a1이 힙 메모리를 사용하는 변수가 아니기 때문이다.
    // let a1=10으로 되어 있기에 a1은 기본 타입(Primitive type)이고 스택만을 사용해 저장되는 변수다.
    let a1 = 10;
    let a2 = a1; // copy: 데이터가 복사되어서 a2에 할당됨

    println!("a1={}", a1); // a1=10
    println!("a2={}", a2); // a2=10

    // struct에 대해 Copy 트레잇을 구현하도록 #[derive(Copy, Clone)] 속성을 부여
    // {:?}에 의해서 출력이 가능하도록하는 것이고, Copy, Clone은 Copy가 일어나도록 하기 위함이다.
    // 내부 데이터가 copy가 불가능한(힙에 저장되는) 타입이면 Copy 구현이 불가능 하다.
    #[derive(Debug, Copy, Clone)]
    struct Foo;

    // 이런 식으로도 가능하다. (derive는 내부적으로 impl을 자동으로 만들어주는 것)
    // impl Copy for Foo {}
    // 역시 Clone도 필수적으로 구현해야한다. (trait Copy: Clone {})
    // impl Clone for Foo {
    //     fn clone(&self) -> Foo {
    //         *self
    //     }
    // }

    // String은 Copy가 아님 → struct도 Copy 불가
    // #[derive(Copy, Clone)] // ❌ 컴파일 에러
    // struct User {
    //     name: String,
    // }

    let st1 = Foo;
    let st2 = st1;
    println!("st1={:?}", st1); // st1=Foo
    println!("st2={:?}", st2); // st2=Foo
}
