// 복합 타입
// 복합 타입의 내용을 출력할 때는 println!("{:?}", data);와 같이 {:?}를 사용

pub fn run() {
    tuple();
    array();
}

fn tuple() {
    println!("------Tuple-------");
    // 튜플은 여러 데이터 타입을 괄호로 감싼 것이다.
    // 전체 출력 시 {:?}를 사용한다.
    let p: (&str, u32) = ("Lee", 20);
    println!("name: {}, age={}", p.0, p.1); // 이런 식으로 개별 출력
    println!("{:?}", p);

    let info = get_info();
    println!("age:{}, height={}", info.0, info.1);
    println!("{:?}", info);
}

// 튜플 타입으로 리턴
fn get_info() -> (i32, f64) {
    let age = 20;
    let height = 60.5;

    return (age, height);
}

// array
fn array() {
    println!("------Array-------");
    // 같은 데이터 타입끼리만 넣을 수 있다.
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]); // 1
    println!("{:?}", a); // [1, 2, 3, 4, 5]
    println!("{:?}", &a[0..2]); // [1,2] a[0], a[1]
    println!("{:?}", &a[3..]); // [4,5] a[3], a[4]

    // [값; 크기]로 여러 개의 값 지정
    let b = [1; 3];
    println!("{:?}", b); // [1, 1, 1]

    // [타입; 크기] 배열 선언 후 나중에 값 지정
    let b1: [char; 5];
    b1 = ['a'; 5]; // 선언된 크기만큼 값 복사해야함 ['a';3]같이 크기가 작으면 에러
    println!("{:?}", b1);

    // 벡터 사용
    let mut v = vec![1, 2, 3];
    v.insert(3, 4);
    println!("{:?}", v);
}
