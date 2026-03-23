// 매크로 정의! 메크로 이름
macro_rules! say_hello {
    // () 입력 패턴, => 이렇게 바꾼다.
    () => {
        // 실행 코드
        println!("Hello!");
    };
}

macro_rules! print_value {
    // $x 메크로 변수, expr 표현식(expression)
    ($x:expr) => {
        println!("Value: {}", $x);
    };
}

macro_rules! my_vec {
  // $( ... )* 반복의 의미, $x:expr 하나의 값
  // "expr 여러 개를 ,로 구분해서 받아라"라는 의미
  ( $( $x:expr ),* ) => {
    {
      let mut temp = Vec::new();
      $(
        temp.push($x);
      )*
      temp
    }
  };
}

pub fn run() {
    say_hello!();
    print_value!(10);
    let v = my_vec![1, 2, 3, 4];
    println!("{:?}", v);
}
