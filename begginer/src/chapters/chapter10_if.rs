// if문
// if는 expression이므로 값을 반환할 수 있다
// 블록 내부 마지막 표현식에는 ;를 붙이지 않는다
// 하지만 let 문장은 ;로 끝나야 한다

// let [변수명] = if [조건] { 값 } else { 값 };

pub fn run() {
    let mut n = 8;

    if n > 5 {
        n = 4;
    } else {
        n = 6;
    }

    // 반환 값이므로 중괄호 내부 마지막에 ;를 붙이지 않고 마지막에 ;
    let a = if n > 5 { 1 } else { 0 };

    println!("{}", a);
}
