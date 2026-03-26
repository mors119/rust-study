pub fn run() {
    let gender = get_customer(4);
    match gender {
        // 열거형 타입에 접근 <열거형 이름>::<원소 타입 이름>
        Gender::Male => println!("Male"),
        Gender::Female => println!("Female"),
    }

    // '값'이 아니라 그 '타입'으로 구분해서 처리
    let ex_gender = get_ex_customer(4);
    match ex_gender {
        EXGender::Male {
            name: n,
            is_military: b,
        } => {
            println!("name={}, is_military={}", n, b);
        }
        EXGender::Female { name: n } => {
            println!("name={}", n);
        }
    }
}

// 선언은 구조체와 유사
enum Gender {
    Male,
    Female,
}

fn get_customer(id: i32) -> Gender {
    if id % 2 == 0 {
        return Gender::Male;
    }
    return Gender::Female;
}

enum EXGender {
    Male { name: String, is_military: bool },
    Female { name: String },
}

fn get_ex_customer(id: i32) -> EXGender {
    if id % 2 == 0 {
        return EXGender::Male {
            name: "Jeff".to_owned(),
            is_military: true,
        };
    }
    return EXGender::Female {
        name: "Alice".to_owned(),
    };
}
