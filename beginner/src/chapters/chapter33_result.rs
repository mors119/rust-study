use std::{fs::File, io::Write};
/*
Option: Some, None
Result: Ok, Err

enum Result<T, E> {
  Ok(T),
  Err(E),
}

* 안전성이 요구되는 대규모의 프로그램에서는 Result 사용

* 간단한 프로그램이면, 에러가 발생한 곳에서 처리하게 한다.
* 복잡한 프로그램이면, 최초 호출된 곳에서 에러를 처리하게한다. 즉, 하위단 함수들은 에러를 상위로 위임하게한다.
* UI가 있는 프로그램이면, 최종 UI 단에서 에러를 처리하게 한다.

* match: 복잡하고, 상용 프로그램이면 match 사용한다.
* ? : 에러 위임방식을 사용하는 함수의 경우는 '?' 사용한다.
* expect, unwrap: 간단한 프로그램의 경우에만 expect 혹은 unwrap(unwrap_or / unwrap_or_else) 사용

* unwrap / expect → panic + 메시지 출력 + 종료 (터져도 되는 곳)
* ?               → 에러 반환 (출력 없음, 종료 없음) (위에서 처리하도록 에러 위임)
*/
pub fn run() {
    // unwrap / expect : 실패하면 panic
    let (q, r) = divmod(10, 3).unwrap();
    println!("(quotient, remainder)={:?}", (q, r)); //(3,1)

    // match : 안전하고 명확
    match divmod(10, 3) {
        Ok((q, r)) => println!("(quotient, remainder)={:?}", (q, r)),
        Err(e) => println!("Error: {}", e),
    }

    // if let : 성공만 필요
    if let Ok((q, r)) = divmod(10, 3) {
        println!("(quotient, remainder)=({}, {})", q, r);
    }

    // write_file("foo.txt");
    match write_file("foo.txt") {
        Ok(()) => {
            println!("File Writing Sucess");
        }
        Err(error) => {
            println!("Error:{}", error);
            return;
        }
    }
}

fn divmod(n: i32, d: i32) -> Result<(i32, i32), String> {
    if d == 0 {
        Err("can't divide by zero".to_owned())
    } else {
        Ok((n / d, n % d))
    }
}

// fn write_file(f_name: &str) {
//     let mut f = match File::create(f_name) {
//         Ok(file) => file,
//         Err(error) => {
//             println!("Error: {}", error);
//             return;
//         }
//     };
//     let _ = f.write_all(b"hello");
// }

// fn write_file(f_name: &str) -> Result<(), String> {
//     let mut f = match File::create(f_name) {
//         Ok(file) => file,
//         Err(error) => {
//             return Err(format!("File Creation Error: {}", error));
//         }
//     };
//     let _ = f.write_all(b"hello");
//     return Ok(());
// }

// 위와 동일하지만, 숏컷 ?를 사용해서 에러를 쉽게 전파.
// 숏컷 ?는 None이면 바로 return None, Error면 바로 return Error
// - 성공 → 값 꺼냄
// - 실패 → 에러를 panic 없이 "그대로 반환"
fn write_file(f_name: &str) -> Result<(), std::io::Error> {
    let mut f = File::create(f_name)?;
    let _ = f.write_all(b"hello");
    return Ok(());
}
