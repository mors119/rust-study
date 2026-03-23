// &str: 고정된 문자열 주소값(포인터, 길이 데이터만을 가지고 있다.)
// Rust에서의 문자열은 항상 utf-8 인코딩(문자 하나를 표현함에 있어 1바이트 혹은 2~4바이트까지 가변하여 표현)을 사용
// String: 가변 문자열

pub fn run() {
    str_fn();
    str_method();
}

fn str_fn() {
    // ! 문자 개수와 &str의 길이는 다르다
    let en = "hello"; // len = 5
    let ko = "안녕하세요"; // len = 15, 한글은 보통 3byte 정도의 크기를 갖는다.

    println!("hello-len = {}, 안녕하세요-len = {}", en.len(), ko.len());

    // 실제 문자의 개수 구하기
    println!(
        "hello-chars-count = {}, 안녕하세요-chars-count = {}",
        en.chars().count(),
        ko.chars().count()
    );

    // 문자열 슬라이스
    // println!("{}", en[0]); // 에러
    // println!("{}", &en[0]); // 에러
    println!("{}", &en[0..1]); // 'h'
    // println!("{}",&ko[0..1]); // 에러
    println!("{}", &ko[0..3]); // '안'

    // &str 문자열을 문자로 바꿔야 할 때는 위에서처럼 슬라이스로 하기보다는
    // chars()메서드 이용해서 문자로 변환한 후 다루는게 편리
    let phone = "02-123-4567";
    println!("{:?}", get_number(phone));
}

fn get_number(s: &str) -> Vec<u32> {
    s.chars()
        .into_iter() // 문자를 문자열로
        .filter_map(|c| c.to_digit(10)) // Some에 대한 처리를 위해 filter_map, to_digit(10)은 10진수로 변환
        .collect::<Vec<u32>>() // 벡터타입으로 지정 let v:Vec<u32>와 유사
}

fn str_method() {
    // bytes: pub fn bytes(&self) -> Bytes<'_>
    // 문자열의 바이트들에 대한 반복자를 리턴한다.
    let mut bytes = "ok".bytes();
    assert_eq!(Some(b'o'), bytes.next());
    assert_eq!(Some(b'k'), bytes.next());
    assert_eq!(None, bytes.next());

    // contains: pub fn contains<P>(&self, pat: P)
    // 주어진 pat 문자열과 일치하는 문자열 슬라이스가 있으면 true를 리턴한다.
    let camus = "albert camus";
    assert!(camus.contains("camus"));

    // find: pub fn find<P>(&self, pat: P) -> Option<usize>
    // 주어진 pat 문자 혹은 문자열과 일치하는 문자열 슬라이스의 처음 위치 인덱스를 리턴한다.
    assert_eq!(camus.find('c'), Some(7));

    // lines: pub fn lines(&self) -> Lines<'_>
    // 문자열을 라인 단위로 짤라서 문자열 슬라이스 반복자로 리턴한다. 라인의 구분은 '\n' 혹은 '\r\n' 'r'만 있는 경우는 라인으로 구분하지 않는다.
    let text = "camus\r\nbooks\n\nis good\r";
    let mut lines = text.lines();
    assert_eq!(Some("camus"), lines.next());
    assert_eq!(Some("books"), lines.next());
    assert_eq!(Some(""), lines.next());
    // Trailing carriage return is included in the last line
    assert_eq!(Some("is good\r"), lines.next());

    // parse: pub fn parse<F>(&self) -> Result<F, <F as FromStr>::Err>
    // 다른 타입으로 변환
    // 1. Basic usage: 변환 타입을 변수에 명시
    let four: u32 = "4".parse().unwrap();
    assert_eq!(4, four);

    // 2. 'turbofish' 방법: parse 메서드에 변환타입을 알려준다.
    let four = "4".parse::<u32>();
    assert_eq!(Ok(4), four);

    // split_whitespace: pub fn split_whitespace(&self) -> SplitWhitespace<'_>
    // 문자열에서 공백을 기준으로 문자열 슬라이스를 만들어내서 반복자로 리턴한다.
    let mut iter = camus.split_whitespace();
    assert_eq!(Some("albert"), iter.next());
    assert_eq!(Some("camus"), iter.next());

    // trim: pub fn trim(&self) -> &str
    // 문자열의 제일 앞과 뒤에 있는 공백을 제거한 후 새로운 str을 만들어서 &str 타입으로 리턴
    let s = "\n Hello\tworld\t\n";
    assert_eq!("Hello\tworld", s.trim());

    // to_lowercase: pub fn to_lowercase(&self) -> String
    // 소문자로 바꿔서 String 타입으로 리턴한다.
    assert_eq!("hello world", s.to_lowercase());
}
