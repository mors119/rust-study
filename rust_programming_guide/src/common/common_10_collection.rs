use std::collections::HashMap;

pub fn run() {
    // 벡터 (Vector)
    let mut v: Vec<i32> = Vec::new(); // 빈 벡터 생성
    v.push(1); // 벡터에 요소 추가
    v.push(2);
    v.push(3);
    println!("벡터 v: {:?}", v);

    // 벡터 초기화 시 요소를 바로 지정할 수도 있습니다.
    let v2 = vec![4, 5, 6];
    println!("벡터 v2: {:?}", v2);

    // 벡터의 요소에 접근하기
    let third = &v[2]; // 인덱스 2의 요소에 대한 참조
    println!("v[2]: {}", third);

    // 벡터의 요소에 접근할 때 get 메서드를 사용할 수도 있습니다. get 메서드는 Option<&T>를 반환하므로, 안전하게 요소에 접근할 수 있습니다.
    match v.get(2) {
        Some(third) => println!("v.get(2): {}", third),
        None => println!("인덱스 2는 벡터의 범위를 벗어났습니다."),
    }

    // 벡터의 요소를 반복
    for i in &v {
        println!("벡터의 요소: {}", i);
    }

    let mut v3 = vec![10, 20, 30];
    for i in &mut v3 {
        *i += 1; // i는 &mut i32 타입이므로, *i를 사용하여 값을 변경할 수 있습니다.
    }
    println!("업데이트된 벡터 v3: {:?}", v3);

    // 열거형을 이용해 벡터에 다양한 타입의 값을 저장할 수도 있습니다. 이를 위해 열거형을 정의할 수 있습니다.
    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // 문자열 (String)
    let mut s = String::new(); // 빈 문자열 생성

    let data = "initial contents";
    let _s2 = data.to_string(); // &str을 String으로 변환
    let _s3 = "initial contents".to_string(); // &str을 String으로
    let _s4 = String::from("initial contents"); // &str을 String으로 변환하는 또 다른 방법

    s.push_str("hello"); // 문자열에 문자열 추가
    println!("문자열 s: {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 이동되고(더이상 사용불가), s2는 참조로 전달됩니다.
    println!("문자열 s3: {}", s3);
    let s4 = format!("{}{}", s2, s3); // format! 매크로를 사용하여 문자열을 연결할 수도 있습니다.
    println!("문자열 s4: {}", s4);
    // + 연산자는 왼쪽 피연산자의 소유권을 가져가기 때문에 두 문자 모두 참조하는 아래 같은 코드는 사용할 수 없습니다.
    // let s5 = &s3 + " " + &s4;

    // let h1 = s1[0];는 허용되지 않습니다. Rust의 문자열은 UTF-8로 인코딩되어 있기 때문에, 인덱스로 접근하는 것은 허용되지 않습니다.
    let w = s2.chars().next().unwrap(); // 문자열의 첫 번째 문자 가져오기
    println!("s2의 첫 번째 문자: {}", w);
    let orld = &s2[1..5]; // 문자열의 일부 슬라이스 가져오기 (panic이 발생할 수 있으므로 주의해야 합니다.)
    let part: String = s2.chars().skip(1).take(5).collect(); // 문자열의 일부 슬라이스를 문자 단위로 가져오기 (안전하게 처리할 수 있습니다.)
    println!("s2의 일부 슬라이스 (문자 단위): {}", part);
    println!("s2의 일부 슬라이스: {}", orld);
    let s_bytes = s2.as_bytes(); // 문자열을 바이트 배열로 변환
    println!("s2의 바이트 배열: {:?}", s_bytes);

    for c in s3.chars() {
        println!("s3의 문자: {}", c);
    }

    // 해시맵 (HashMap)
    let mut scores = HashMap::new(); // 빈 해시맵 생성

    // 해시맵에서 값을 업데이트하는 방법
    scores.insert(String::from("Blue"), 10); // 키-값 쌍 추가
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Green")).or_insert(30); // 키가 존재하지 않을 때만 값 추가
    scores.entry(String::from("Blue")).or_insert(25); // 키가 이미 존재하므로 값이 업데이트되지 않습니다.

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("해시맵 scores: {:?}", scores);

    // 해시맵에서 값 가져오기
    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        Some(score) => println!("{}: {}", team_name, score),
        None => println!("{} 팀은 점수가 없습니다.", team_name),
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // 문자열을 공백으로 분리하여 단어를 반복합니다.
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // 단어가 이미 존재하면 해당 단어의 카운트를 가져오고, 존재하지 않으면 0으로 초기화합니다.
        *count += 1; // 단어의 카운트를 증가시킵니다.
        println!("단어: {}, 카운트: {}", word, count);
    }
    println!("단어 카운트: {:?}", map);
}
