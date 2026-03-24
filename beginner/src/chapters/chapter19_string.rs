// String 가변 크기의 문자열
pub fn run() {
    // from으로 기본값 지정이 가능하다.
    let mut s0 = String::from("Hello, ");
    // to_owned()는 &str 타입용으로 구현되었고,
    // to_string()은 u32 등 여러 타입들에 대해서도 String으로 변형하기 위해서 사용되는 범용 메서드이기 때문에 더 무겁다.
    let s1: String = "string &".to_owned();
    let s2: String = " string".to_string();

    // 문자열 합치기
    // String 타입 변수가 사용할 수 있는 메서드로, &str 타입 문자열을 자신의 문자열 뒤에 합쳐 넣는다.
    s0.push_str("&str");
    println!("{}", s0);

    // + 오퍼레이션 사용
    // String 변수 + &String 변수 혹은 String 변수 + &str 변수 형태
    // + 기호 앞에는 문자열에 대한 소유권을 가지고 있는 String 타입이 와야한다.
    let s3 = s1 + &s2;
    println!("{}", s3);
    // println!("{}", s1); // ! error s3으로 소유권이 이동됨.

    // format! 문자열을 모두 레퍼런스 타입으로 해서 format!매크로로 문자열을 합치는 것이다.
    let c = format!("{}{}", &s2, &s3); // &String 이용
    println!("c= {}", c);
    let c = format!("{}{}", "&str", "&str"); // &str 이용
    println!("c= {}", c);
    let c = format!("{}{}", s2, s3); // String 이용
    println!("c= {}", c);

    string_method();
}

fn string_method() {
    // as_bytes: pub fn as_bytes(&self) -> &[u8]
    // 문자열의 byte 슬라이스를 리턴, 반대는 `from_utf8`이다.
    let mut s1 = String::from("hi");
    assert_eq!(&[104, 105], s1.as_bytes());

    // as_str: pub fn as_str(&self) -> &str
    // String의 전체 문자열을 &str 타입의 슬라이스로 변환
    assert_eq!("hi", s1.as_str());

    // clear: pub fn clear(&mut self)
    // String의 모든 내용을 지우고 길이를 0으로 만든다. 그러나, 메모리 할당 크기인 capacity는 그대로 유지된다
    s1.clear();

    assert!(s1.is_empty());
    assert_eq!(0, s1.len());
    assert_eq!(2, s1.capacity());

    // from_utf8: pub fn from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error>
    // u8 타입 벡터로부터 String을 만든다. 벡터 안에 저장된 u8 타입 값은 utf-8 인코딩으로 String을 만들 수 있는 것이 보장되야한다.
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    assert_eq!("💖", sparkle_heart);

    // insert: pub fn insert(&mut self, idx: usize, ch: char)
    // idx의 위치에 문자 하나를 삽입한다. 문자를 넣을 때는 이 메서드를 이용하고,
    // &str 타입 문자열을 넣을 때는 `insert_str`을 이용한다.
    s1.insert(0, 'h');
    s1.insert(1, 'i');
    assert_eq!("hi", s1);

    // insert_str: pub fn insert_str(&mut self, idx: usize, string: &str)
    // &str 타입 문자열을 idx 위치에 삽입한다. 만약 idx의 범위가 크기를 벗어나면 panic 발생한다.
    s1.insert_str(2, ", camus");
    assert_eq!("hi, camus", s1);

    // into_bytes: pub fn into_bytes(self) -> Vec<u8>
    // 문자열을 byte 타입 벡터로 만들어서 리턴한다.
    let bytes = s1.clone().into_bytes();
    assert_eq!(&[104, 105, 44, 32, 99, 97, 109, 117, 115][..], &bytes[..]);

    // push: pub fn push(&mut self, ch: char)
    // 문자열의 제일 마지막에 주어진 문자 ch를 삽입한다.
    s1.push('!');
    assert_eq!("hi, camus!", s1);

    // push_str: pub fn push_str(&mut self, string: &str)
    // 문자열의 제일 마지막에 &str 타입 문자열을 삽입한다. String 타입을 삽입하는 메서드는 없다.
    // 따라서, as_str()이용해서 &str 타입으로 변환 후 삽입해야한다.
    s1.push_str("bar");
    assert_eq!("hi, camus!bar", s1);

    // remove: pub fn remove(&mut self, idx: usize) -> char
    // idx 위치에 있는 한 문자를 제거하고, 그 문자를 리턴한다.
    assert_eq!(s1.remove(11), 'a');

    // truncate: pub fn truncate(&mut self, new_len: usize)
    // 문자열을 new_len(지정 길이) 크기로 줄인다. 줄일 때는 앞 부분만 new_len 만큼 남기고 줄인다.
    // 만약 new_len이 문자열의 length와 같거나 크다면 아무일도 일어나지 않는다.
    // 줄어든 후에는 length는 new_len로 변하지만, 문자열을 위해 확보되어 있던 capacity는 줄어들지 않는다.
    s1.truncate(2);
    assert_eq!("hi", s1);
}
