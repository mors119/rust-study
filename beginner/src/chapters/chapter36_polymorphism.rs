/*
impl = 특정 타입이 trait(능력)을 "구현한다(implementation)"고 연결하는 키워드

trait = 공통 기능(행동) 계약
→ impl Trait for Type = "이 타입은 이 기능을 할 수 있다"

----------------------------------------
[impl Trait 사용 위치]

1) 파라미터
fn f(x: impl Trait)

→ "이 trait을 구현한 아무 타입이나 받겠다"
→ 정적 바인딩(static dispatch, 컴파일 시점에 타입 확정)

2) 반환값
fn f() -> impl Trait

→ "이 trait을 구현한 어떤 하나의 구체 타입을 반환"
→ 실제 타입은 숨기지만 내부적으로는 하나로 고정됨

----------------------------------------
[trait bound (제네릭 방식)]

fn f<T: Trait>(x: T)

→ impl Trait과 동일 의미
→ 차이: T를 여러 곳에서 재사용 가능

----------------------------------------
[연관 타입 (associated type)]

trait Trait {
    type Item;
}

→ 구현체마다 내부 타입을 다르게 정의 가능
→ Trait<type = ...> 형태로 제한 가능

----------------------------------------
[정적 vs 동적 바인딩]

정적 바인딩 (impl Trait, T: Trait)
→ 컴파일 시점에 타입 확정
→ 빠르고 최적화 가능

동적 바인딩 (Box<dyn Trait>)
→ 실행 시점에 타입 결정 (vtable 사용)
→ 여러 타입을 하나로 다룰 수 있음

----------------------------------------

trait         = 기능 계약
impl          = 기능 구현 연결
impl Trait    = 해당 기능 가진 타입 아무거나
T: Trait      = 제네릭 + 기능 제한
dyn Trait     = 런타임에 타입 결정
*/

trait Animal {
    // 연관 타입: 구현체가 "반환 타입"을 정하게 함
    // “이 trait를 구현하는 타입마다 딸려오는 타입 하나를 정하게 하고 싶다”할 때 사용
    type Food;

    fn name(&self) -> &'static str;
    fn eat(&self, food: Self::Food) -> String;
}

struct Dog;
struct Cat;

// Dog는 Food를 i32로 정함
impl Animal for Dog {
    type Food = i32;

    fn name(&self) -> &'static str {
        "Dog"
    }

    fn eat(&self, food: Self::Food) -> String {
        format!("{} eats {} bones", self.name(), food)
    }
}

// Cat은 Food를 &'static str로 정함
impl Animal for Cat {
    type Food = &'static str;

    fn name(&self) -> &'static str {
        "Cat"
    }

    fn eat(&self, food: Self::Food) -> String {
        format!("{} eats {}", self.name(), food)
    }
}

// 1) impl Trait: "Animal<Food = i32> 를 구현한 아무 타입이나 받기"
//    → 정적 바인딩(static dispatch)
fn feed_impl(a: impl Animal<Food = i32>) {
    println!("{}", a.eat(3));
}

// 2) trait bound: 위와 사실상 같은 뜻
//    → 제네릭 + 조건
fn feed_bound<T: Animal<Food = i32>>(a: T) {
    println!("{}", a.eat(5));
}

// 3) impl Trait 반환
//    → "Animal<Food = i32>를 구현한 어떤 구체 타입 하나를 반환"
fn make_dog() -> impl Animal<Food = i32> {
    Dog
}

// 4) 동적 바인딩(dynamic dispatch)
//    → 실행 중에 어떤 구현체인지 결정
fn show_name_dyn(a: Box<dyn Animal<Food = i32>>) {
    println!("dynamic: {}", a.name());
}

pub fn run() {
    let dog = Dog;
    let cat = Cat;

    // trait + 연관 타입 사용
    println!("{}", dog.eat(2)); // Dog eats 2 bones
    println!("{}", cat.eat("fish")); // Cat eats fish

    // 정적 바인딩 2가지 문법
    feed_impl(Dog);
    feed_bound(Dog);

    // impl Trait 반환
    let a = make_dog();
    println!("{}", a.eat(1));

    // 동적 바인딩(Box<dyn Trait>)
    let boxed: Box<dyn Animal<Food = i32>> = Box::new(Dog);
    show_name_dyn(boxed);
}
