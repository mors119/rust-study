use std::rc::Rc;

// Rc = Reference counted, 레퍼런싱하는 메모리로의 접근을 카운터를 써서 관리한다
// 여러 변수가 하나의 힙 메모리 공간을 가리킬 수 있게 해준다.
pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;

    // Rc::clone()를 수행한다고 해서 Rc 객체가 추가로 복제되서 생기는 것이 아니고, 단지 Rc 내의 'Ref count(참조 카운트)'의 값만 더 증가하게 된다.
    // 'Ref count'가 0이라는 것은 참조하는 객체가 아무것도 없다는 의미이고, 이때는 해당 객체가 힙 메모리에서 자동으로 정리
    let s3 = Rc::new(String::from("hello"));
    let s4 = Rc::clone(&s3); // 데이터를 기리키는 포인터만 하나 더 생성됨.

    //println!("{s1}");  // 소유권 이동으로 오류가 발생한다.
    println!("{s2}");

    // s3가 Rc 객체를 가리키고, Rc 객체에 있는 포인터가 String 객체를 가리키는 형태다.
    println!("{s3}");
    println!("{s4}");
} // rc가 모두 사라지면 데이터도 drop 된다.
