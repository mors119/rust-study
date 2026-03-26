/*
trait = "이 타입은 이런 기능(능력)을 할 수 있어야 한다"
Java의 인터페이스(interface)와 유사 / struct는 Java의 Class와 유사

* 타입 (type):
공통으로 사용될 수 있는 타입 정의 가능
디폴트 할당 불가능
* 상수 (const):
트레잇에 정의되어 해당 트레잇을 impl하는 개체들에서 공통으로 사용 가능
디폴트 할당 가능
* 함수 (function):
공통으로 사용되는 함수
디폴트 할당 가능

* impl "이 타입이 이 능력(trait)을 가지고 있다”를 연결해주는 키워드
 */

pub fn run() {
    // 다양한 형태로 사용
    let d = Dog;
    d.speak();
    Cat.speak();
    Bird.speak();
    make_sound(d); // Speak를 구현한 타입만 들어올 수 있음
    make_sound(Cat);

    // Dog인지 몰라도 되고 중요한 건 Speak를 구현했다는 사실만 알면 된다
    let animal = get_animal(); // impl Speak
    animal.speak();

    // impl Speak = 한 종류 타입을 trait 관점에서 받기, 컴파일 시점에 정해지므로 빠름.
    // dyn Speak  = 여러 종류 타입을 trait 하나로 묶기, 런타임에 어떤 타입인지 보고 trait 호출
    let animals: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Cat)];
    make_all_sound(animals);

    car_fn();
}

// trait 정의
// 의미: Speak를 구현한 타입은 speak()를 반드시 가져야 한다
trait Speak {
    // fn speak(&self); // 오버라이드 필수
    fn speak(&self) {
        println!("기본 소리"); // 기본값 지정
    }
}

// struct + impl
struct Dog;
struct Cat;

struct Bird;

// speak 직접 구현하지 않고 기본값 사용
impl Speak for Bird {}

// “Dog는 Speak이라는 능력을 가진다”
impl Speak for Dog {
    fn speak(&self) {
        println!("멍멍");
    }
}

impl Speak for Cat {
    fn speak(&self) {
        println!("야옹");
    }
}

// 함수에서 trait 사용 (trait bound, 제네릭 타입을 Trait를 구현한 객체만으로 하도록 한정)
fn make_sound<T: Speak>(animal: T) {
    animal.speak();
}
// 위와 동일 (impl Trait 문법, 사용법도 동일, 파라미터 타입을 Trait를 구현한 객체만으로 하도록 한정)
fn _make_sound(animal: impl Speak) {
    animal.speak();
}

// 반환값 struct로 구현 / "반환값의 정확한 타입보다 반환값이 어떤 기능을 하느냐가 중요할 때" 사용
fn get_animal() -> impl Speak {
    Dog
}

// !에러: impl Speak는 위처럼 항상 구체적인 타입을 반환해야함.
// fn get_animal(flag: bool) -> impl Speak {
//     if flag {
//         Dog
//     } else {
//         Cat
//     }
// }

// 모든 소리 출력
// Box<dyn Speak>: Dog(각 객체) 자체를 직접 담는 게 아니라 "Speak 할 수 있는 어떤 값에 대한 포인터"를 담는다
fn make_all_sound(animals: Vec<Box<dyn Speak>>) {
    for a in animals {
        a.speak();
    }
}

// ------------------ car -------------------
fn car_fn() {
    let car1 = Truck {};
    car1.drive(); //Truck is driving.

    let car2 = SUV {};
    car2.drive(); //SUV is driving. 

    let car3 = Sedan {};
    car3.drive(); //Sedan is driving.
}

trait Car {
    fn drive(&self);
}

struct Truck {}
struct SUV {}
struct Sedan {}

impl Car for Truck {
    fn drive(&self) {
        println!("Truck is driving.");
    }
}

impl Car for SUV {
    fn drive(&self) {
        println!("SUV is driving.");
    }
}

impl Car for Sedan {
    fn drive(&self) {
        println!("Sedan is driving.");
    }
}
