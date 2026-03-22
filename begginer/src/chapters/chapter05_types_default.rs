// 각 타입 별 기본값
/*
// 정수형
i8, i16, i32, i64, i128, isize     → 0
u8, u16, u32, u64, u128, usize     → 0

// 부동소수점
f32, f64                           → 0.0

// 불리언
bool    → false

// 문자
char    → '\0' (널 문자, U+0000)

// 컬렉션
String              → "" (빈 문자열)
Vec<T>              → [] (빈 벡터)
HashMap<K, V>       → {} (빈 해시맵)
HashSet<T>          → {} (빈 해시셋)
Option<T>           → None

// 튜플
()                  → () (unit 타입)
(T1, T2, ...)      → (T1::default(), T2::default(), ...)

// 배열
[T; N]             → [T::default(); N]
// 예: [i32; 5]   → [0, 0, 0, 0, 0]

&T                 → 기본값 없음 (참조는 항상 유효한 값을 가리켜야 함)
Box<T>             → Box::new(T::default())
*/

pub fn run() {
    println!("{}", set());

    let person = Person::default();

    // 필드 값을 직접 읽어서 사용하면 경고가 해결됩니다.
    println!("이름: {}, 나이: {}", person.name, person.age);
    println!("{:?}", person);
}

fn set() -> i32 {
    // 기본값을 사용하는 변수 생성
    let num: i32 = Default::default(); // 0

    return num;
}

#[derive(Default, Debug)] // Debug를 추가해야 {:?}로 출력 가능
struct Person {
    name: String,     // ""
    age: u32,         // 0
    _is_active: bool, // false
}
