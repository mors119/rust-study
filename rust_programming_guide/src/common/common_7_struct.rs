// 구조체 (Struct)
#[derive(Debug)] // 구조체를 디버깅 출력할 수 있도록 합니다.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 튜플 구조체 (Tuple Struct)
struct Color(i32, i32, i32);

// 단위 구조체 (Unit Struct)
struct AlwaysEqual;

// 유사 유닛 구조체 (Newtype Struct)
struct UserId(u64);

// 구조체에 연관 함수
impl UserId {
    fn _new(id: u64) -> Self {
        Self(id)
    }
}

impl User {
    fn new(email: String, username: String) -> User {
        User {
            email,
            username,
            active: false,
            sign_in_count: 1,
        }
    }

    // 구조체의 연관 함수 (위와 동일하지만 active 필드를 true로 설정)
    fn _new_active(email: String, username: String) -> Self {
        // Self는 현재 구조체(User)를 가리킵니다.
        Self {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // 구조체에 메서드를 정의할 수 있습니다.
    // 메서드는 첫 번째 매개변수로 &self를 받습니다. 이는 메서드가 호출된 구조체 인스턴스에 대한 참조입니다.
    fn is_active(&self) -> bool {
        self.active // active를 읽어서 반환
    }
}

pub fn run() {
    // 구조체 인스턴스 생성 (struct instance)
    let mut user1 = User {
        email: String::from("someone@example.com"), // 선언 순서는 중요하지 않습니다.
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1: {:?}", user1); // Debug 트레이트를 사용하여 user1의 내용을 출력합니다.
    // 특정 필드만 가변으로 만들 수 없습니다. 구조체 전체가 가변(mut)이어야 합니다.
    user1.email = String::from("another@example.com");
    println!("user1 email: {}", user1.email);

    // 구조체 업데이트 구문 (struct update syntax)
    let user2 = User {
        email: user1.email.clone(), // user1의 email 필드를 user2로 복제(clone)합니다.
        username: user1.username, // user1의 username 필드를 user2로 이동(move)합니다. 이제 user1.username은 더 이상 유효하지 않습니다.
        ..user1                   // user1의 나머지 필드를 user2로 복사합니다.
    };
    dbg!(&user2); // user2의 내용을 '에러 콘솔'에 출력합니다.
    println!("user1 email: {}, user2 email: {}", user1.email, user2.email);

    let user3 = build_user(
        String::from("user3@example.com"),
        String::from("user3name"),
        1,
    );
    println!(
        "user3 email: {}, username: {}, sign_in_count: {}",
        user3.email, user3.username, user3.sign_in_count
    );

    // 튜플 구조체 인스턴스 생성
    let black = Color(0, 0, 0);
    println!("black color: ({}, {}, {})", black.0, black.1, black.2);

    // 단위 구조체 인스턴스 생성
    let _subject = AlwaysEqual; // 사용시기: 단위 구조체는 타입 시스템에서 특정한 의미를 나타내기 위해 사용될 수 있습니다.

    // 유사 유닛 구조체 인스턴스 생성
    let user_id = UserId(12345); // 사용시기: 유사 유닛 구조체는 단일 필드를 가지는 튜플 구조체로, 새로운 타입을 정의할 때 사용됩니다. 
    // 예를 들어, UserId는 u64 타입의 값을 가지지만, UserId라는 새로운 타입으로 구분됩니다.
    println!("user_id: {}", user_id.0);

    // 구조체 인스턴스 생성 시 필드 init shorthand 문법을 사용할 수 있습니다.
    println!(
        "{:?} is new user",
        User::new(String::from("user4@example.com"), String::from("user4name"))
    );

    // 구조체 메서드 호출
    println!("Is user1 active? {}", user3.is_active());
}

fn build_user(email: String, username: String, sign_in_count: u64) -> User {
    User {
        email,    // 필드 init shorthand 문법
        username, // 필드 init shorthand 문법
        active: true,
        sign_in_count: sign_in_count + 1,
    }
}
